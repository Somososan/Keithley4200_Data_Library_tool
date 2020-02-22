use crate::database::Database;
use clap::{App, Arg};
use std::fs::{self};
use std::path::Path;
use webview::*;

mod measurement;

mod calamine_helper;

mod database;

mod elm;

pub trait Extract {
    fn extract(sheet: &calamine_helper::MyRange) -> Option<Self>
    where
        Self: std::marker::Sized;
}

fn populate_from_path(
    root: String,
    relative_dir: String,
    storage: &mut database::Database,
) -> std::io::Result<()> {
    let dir_string = format!("{}{}", root.clone(), relative_dir);

    let dir = Path::new(dir_string.as_str());

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path: String = path.to_str().unwrap().replace(root.clone().as_str(), "");
            if path.is_dir() {
                populate_from_path(root.clone(), relative_path, storage)?;
            } else if path.extension().unwrap() == "xls" {
                measurement::Measurement::extract(root.clone(), relative_path, storage);
            } else {
            }
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("Library tool")
        .version("0.1")
        .author("C Karaliolios")
        .about(
            "Processes Keithley 4200 parameter analyzer data and generates images and a interface",
        )
        .arg(
            Arg::with_name("input_directory")
                .short("i")
                .long("input_dir")
                .value_name("PATH")
                .help("Sets the directory to be searched"),
        )
        .arg(
            Arg::with_name("output_directory")
                .short("o")
                .long("output_dir")
                .value_name("PATH")
                .help("Sets the directory for the outputs to be collected in"),
        )
        .arg(
            Arg::with_name("script_directory")
                .short("s")
                .long("script_dir")
                .value_name("PATH")
                .help("Sets the directory where the python scripts selected out of"),
        )
        .get_matches();

    //default input directory
    let input_string: String = format!("{}/tests", env!("CARGO_MANIFEST_DIR"));
    // get input directory from CLI
    let input_dir = matches
        .value_of("input_directory")
        .unwrap_or(input_string.as_str())
        .to_string();

    //default output directory
    let output_string: String = format!("{}/output", env!("CARGO_MANIFEST_DIR"));
    // get output directory from CLI
    let output_dir = matches
        .value_of("output_directory")
        .unwrap_or(output_string.as_str())
        .to_string();

    //default script directory
    let script_string: String = format!("{}/scripts", env!("CARGO_MANIFEST_DIR"));
    // get input directory from CLI
    let script_dir = matches
        .value_of("script_directory")
        .unwrap_or(script_string.as_str())
        .to_string();

    //initialize id and measurement vector
    let json = fs::read_to_string(format!("{}/result.json", output_dir));
    let storage: &mut Database = &mut Database::new();
    match json {
        Ok(string) => {
            *storage = serde_json::from_str::<Database>(string.as_str()).unwrap_or(Database::new())
        }
        _ => (),
    };
    populate_from_path(input_dir, String::from(""), storage).expect("Error transfercing path");

    let v = serde_json::to_string(storage).unwrap();
    fs::write(format!("{}/result.json", output_dir), v.as_str()).expect("error writing json");

    let html = format!(
        r#"<!doctype html>
        <html>
        <head>
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta charset="UTF-8">
            {styles}
        </head>
        <body>
            <!--[if lt IE 11]>
            <div class="ie-upgrade-container">
                <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
                <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
            </div>
            <![endif]-->
            <div id="elm"></div>
            {scripts}
        </body>
        </html>
		"#,
        styles = inline_style(include_str!("../elm-code/styles.css")),
        scripts = inline_script(include_str!("../elm-code/elm.js"))
            + &inline_script(include_str!("../elm-code/app.js")),
    );

    let mut webview = webview::builder()
        .title("Ckaraliolios Data Analysis tool")
        .content(Content::Html(html))
        .size(1600, 900)
        .resizable(true)
        .debug(true)
        .user_data({
            let message_nr = 0;
            let task_done = elm::Task::Init;
            let measurements: Vec<measurement::MeasurementCompact> = storage
                .measurements
                .clone()
                .into_iter()
                .map(|m| m.to_compact())
                .collect();
            let filter_options = elm::filter::FilterOptions::new(&measurements);
            let measurements: Vec<measurement::MeasurementCompact> =
                elm::filter::FilterQuery::from(filter_options).filter(measurements);
            let filter_options = elm::filter::FilterOptions::new(&measurements);
            let result = elm::ToElm {
                message_nr,
                task_done,
                measurements,
                filter_options,
            };
            //println!("{}", serde_json::to_string_pretty(&result).unwrap());
            result
        })
        .invoke_handler(|webview, arg| {
            let compact_msmt: Vec<measurement::MeasurementCompact> = storage
                .measurements
                .clone()
                .into_iter()
                .map(|m| m.to_compact())
                .collect();
            let to_elm = webview.user_data_mut();
            if serde_json::from_str::<elm::FromElm>(arg).is_err() {
                println!("{:#?}", arg);
            }
            match serde_json::from_str(arg).unwrap() {
                elm::FromElm::Init => {
                    *to_elm = {
                        println!("Init {}", to_elm.message_nr);
                        let message_nr = to_elm.message_nr + 1;
                        let task_done = elm::Task::Init;
                        let measurements = compact_msmt.clone();
                        let filter_options = elm::filter::FilterOptions::new(&measurements);
                        let measurements: Vec<measurement::MeasurementCompact> =
                            elm::filter::FilterQuery::from(filter_options.clone())
                                .filter(measurements);
                        elm::ToElm {
                            message_nr,
                            task_done,
                            measurements,
                            filter_options,
                        }
                    }
                }
                elm::FromElm::Log(string) => println!("{}", string),
                elm::FromElm::Filter(query) => {
                    *to_elm = {
                        let message_nr = to_elm.message_nr + 1;
                        println!("Filtering");
                        let task_done = elm::Task::Filtering;
                        let measurements = query.filter(compact_msmt.clone());
                        let filter_options =
                            elm::filter::FilterOptions::filtered(&compact_msmt, query);
                        elm::ToElm {
                            message_nr,
                            task_done,
                            measurements,
                            filter_options,
                        }
                    }
                }
                elm::FromElm::Process(query) => {
                    *to_elm = {
                        println!("Processing");
                        let message_nr = to_elm.message_nr + 1;
                        query.process(
                            storage.measurements.clone(),
                            output_dir.as_str(),
                            script_dir.as_str(),
                        );
                        let task_done = elm::Task::Processing;
                        let measurements = to_elm.measurements.clone();
                        let filter_options = to_elm.filter_options.clone();
                        elm::ToElm {
                            message_nr,
                            task_done,
                            measurements,
                            filter_options,
                        }
                    }
                }
            }

            render(webview)
        })
        .build()
        .unwrap();

    webview.set_color((255, 255, 255));

    let res = webview.run().expect("Error in webview part");

    println!("final state: {:?}", res);
}

fn render(webview: &mut WebView<elm::ToElm>) -> WVResult {
    let render_tasks = {
        let to_elm = webview.user_data();
        format!(
            "app.ports.fromRust.send({})",
            serde_json::to_string(to_elm).unwrap()
        )
    };
    webview.eval(&render_tasks)
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
