port module Main exposing (..)

import Browser
import CurrentRange
import Dict exposing (..)
import Die
import EncodeHelper exposing (..)
import FilterOptions exposing (FilterOptions)
import FilterQuery exposing (..)
import FromRust exposing (..)
import Html exposing (Html, br, button, div, form, h1, hr, input, li, option, select, span, table, td, text, th, tr, ul)
import Html.Attributes exposing (autofocus, checked, class, classList, colspan, disabled, id, multiple, selected, size, type_, value)
import Html.Events exposing (onCheck, onClick, onInput, onSubmit)
import Html.Keyed exposing (..)
import Html.Lazy exposing (lazy)
import Instrument
import Json.Decode as Decode
import Json.Decode.Pipeline exposing (..)
import Json.Encode as Encode
import List exposing (..)
import List.Extra exposing (zip)
import MeasurementCompact exposing (MeasurementCompact)
import MeasurementSpeed
import MultiSelect exposing (Item, Options, multiSelect)
import OpModeType
import ProcessQuery exposing (ProcessQuery)
import ProcessingType exposing (ProcessingType)
import RustTask exposing (..)
import Terminal
import TestData exposing (TestData)
import TestDataCompact exposing (TestDataCompact)
import TestType exposing (..)
import TimeStamp
import ToRust exposing (..)
import UnitMeasured
import VoltageRange
import Wafer exposing (..)


port toRust : Encode.Value -> Cmd msg


port fromRust : (Encode.Value -> msg) -> Sub msg


main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- MODEL


type alias M_ID =
    String


type Pages
    = SelectPage
    | ProcessPage


type alias Entry =
    { data : List TestDataCompact
    , selected : Bool
    }


type alias Model =
    { message_nr : Int
    , page : Pages
    , selected_entries : Dict M_ID Entry
    , measurements : List MeasurementCompact
    , filters_used : FilterQuery
    , filter_options : FilterOptions
    , combine_data : Bool
    , normalize_data : Bool
    , process_options_selected : List ProcessingType
    }


type Msg
    = SelectEntry M_ID Bool
    | ChangeCombineData Bool
    | ChangeNormalizeData Bool
    | SelectProcessType ProcessingType Bool
    | ChangeAllEntries Bool
    | SendToRust String
    | ChangeFilter FilterQuery
    | ChangeWafer String
    | ChangeTestType String
    | UpdateModel FromRust
    | ToSelectPage
    | ToProcessPage
    | ProcessData


init : () -> ( Model, Cmd Msg )
init _ =
    ( { message_nr = 0, page = SelectPage, selected_entries = Dict.empty, measurements = [], filters_used = FilterQuery.empty, filter_options = FilterOptions.empty, combine_data = False, normalize_data = False, process_options_selected = [] }, toRust (Encode.object [ ( "torust", Encode.string "Init" ) ]) )



----- UPDATE


onChange : (String -> msg) -> Html.Attribute msg
onChange handler =
    Html.Events.on "change" <| Decode.map handler <| Decode.at [ "target", "value" ] Decode.string


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SelectEntry id state ->
            let
                make_data_list testdata =
                    let
                        base_data n =
                            { terminal = testdata.terminal, unit = testdata.unit, count = n }
                    in
                    List.range 1 testdata.count
                        |> List.map base_data

                all_data =
                    model
                        |> .measurements
                        |> List.filter (\m -> m.id == id)
                        |> List.head
                        |> Maybe.map (\m -> m.test_data)
                        |> Maybe.withDefault []
                        |> List.concatMap make_data_list

                updateEntry : Maybe Entry -> Maybe Entry
                updateEntry maybeEntry =
                    case maybeEntry of
                        Just oldEntry ->
                            Just { oldEntry | data = all_data, selected = state }

                        Nothing ->
                            Just { data = all_data, selected = state }

                entries =
                    model
                        |> .selected_entries
                        |> Dict.update id updateEntry

                string =
                    if state then
                        id ++ "checked"

                    else
                        id ++ "unchecked"
            in
            ( { model | selected_entries = entries }, toRust (ToRust.log string) )

        ChangeCombineData state ->
            let
                string =
                    if state then
                        "Combine data checked"

                    else
                        "Combine data unchecked"
            in
            ( { model | combine_data = state }, toRust (ToRust.log string) )

        ChangeNormalizeData state ->
            let
                string =
                    if state then
                        "Normalize data checked"

                    else
                        "Normalize data unchecked"

                old_raw =
                    if List.member ProcessingType.raw model.process_options_selected then
                        [ ProcessingType.raw ]

                    else
                        []
            in
            ( { model | normalize_data = state, process_options_selected = old_raw }, toRust (ToRust.log string) )

        SelectProcessType kind state ->
            let
                new_list =
                    if state == False then
                        List.Extra.remove kind model.process_options_selected

                    else if List.member kind model.process_options_selected then
                        model.process_options_selected

                    else
                        kind :: model.process_options_selected

                string =
                    if state then
                        ProcessingType.toString_concise kind ++ " checked"

                    else
                        ProcessingType.toString_concise kind ++ " unchecked"
            in
            ( { model | process_options_selected = new_list }, toRust (ToRust.log string) )

        ChangeAllEntries state ->
            let
                make_data_list testdata =
                    let
                        base_data n =
                            { terminal = testdata.terminal, unit = testdata.unit, count = n }
                    in
                    List.range 1 testdata.count
                        |> List.map base_data

                all_data id =
                    model
                        |> .measurements
                        |> List.filter (\m -> m.id == id)
                        |> List.head
                        |> Maybe.map (\m -> m.test_data)
                        |> Maybe.withDefault []
                        |> List.concatMap make_data_list

                updateEntry : M_ID -> Entry -> Entry
                updateEntry id oldEntry =
                    { oldEntry | selected = state, data = all_data id }

                entries =
                    model
                        |> .selected_entries
                        |> Dict.map updateEntry

                string =
                    if state then
                        "Set all entries"

                    else
                        "Cleared all entries"
            in
            ( { model | selected_entries = entries }, toRust (ToRust.log string) )

        SendToRust log ->
            if log == "Init" then
                ( model, toRust ToRust.init )

            else
                ( model, toRust (ToRust.log log) )

        ChangeFilter filter ->
            ( { model | filters_used = filter }, toRust (ToRust.filter filter) )

        ChangeWafer wfr ->
            let
                filter =
                    model.filters_used

                new_filter =
                    { filter | wafer = wfr }
            in
            ( { model | filters_used = new_filter }, toRust (ToRust.filter new_filter) )

        ChangeTestType ttype ->
            let
                filter =
                    model.filters_used

                new_filter =
                    { filter | test_type = ttype }
            in
            ( { model | filters_used = new_filter }, toRust (ToRust.filter new_filter) )

        UpdateModel fromrust ->
            let
                selected_entries_adjusted =
                    let
                        filter_fn : M_ID -> Entry -> Bool
                        filter_fn id _ =
                            fromrust
                                |> .measurements
                                |> List.map .id
                                |> List.member id
                    in
                    Dict.union (Dict.filter filter_fn model.selected_entries) selected_entries_init

                selected_entries_init =
                    let
                        default_entry =
                            { data = [], selected = False }

                        list_size =
                            List.length fromrust.measurements

                        ids =
                            List.map .id fromrust.measurements

                        entries =
                            List.repeat list_size default_entry
                    in
                    Dict.fromList (zip ids entries)

                filters_used_init : FilterQuery
                filters_used_init =
                    { sheet_names = Dict.keys fromrust.filter_options.sheet_names
                    , widths = Dict.keys fromrust.filter_options.widths
                    , lengths = Dict.keys fromrust.filter_options.lengths
                    , temps = Dict.keys fromrust.filter_options.temps
                    , wafer = Maybe.withDefault "MINOXG" (List.head (Dict.keys fromrust.filter_options.processes))
                    , dies = Dict.keys fromrust.filter_options.dies
                    , test_type = Maybe.withDefault "Sampling" (List.head (Dict.keys fromrust.filter_options.test_types))
                    , measurement_speeds = Dict.keys fromrust.filter_options.measurement_speeds
                    , dates_between = ( Nothing, Nothing )
                    }
            in
            if model.message_nr /= fromrust.message_nr then
                if fromrust.task_done == RustTask.init then
                    ( { model | combine_data = False, normalize_data = False, process_options_selected = [], page = SelectPage, message_nr = fromrust.message_nr, measurements = fromrust.measurements, selected_entries = selected_entries_init, filters_used = filters_used_init, filter_options = fromrust.filter_options }, toRust (ToRust.log "Init received") )

                else
                    ( { model | message_nr = fromrust.message_nr, measurements = fromrust.measurements, selected_entries = selected_entries_adjusted, filter_options = fromrust.filter_options }, toRust (ToRust.log "Done") )

            else
                ( model, Cmd.none )

        ToSelectPage ->
            ( { model | page = SelectPage }, toRust (ToRust.log "Select Page Opened") )

        ToProcessPage ->
            ( { model | page = ProcessPage }, toRust (ToRust.log "Process Page Opened") )

        ProcessData ->
            let
                processdata =
                    model
                        |> .selected_entries
                        |> Dict.filter (\k e -> e.selected)
                        |> Dict.toList
                        |> List.map (\( k, e ) -> { id = k, data = e.data })

                query =
                    { what = model.process_options_selected, combined = model.combine_data, from = processdata }
            in
            ( model, toRust (ToRust.process query) )



-- VIEW


filterview : Model -> List (Html Msg)
filterview model =
    let
        opt_items options =
            let
                item : ( String, Int ) -> MultiSelect.Item
                item ( key, count ) =
                    { value = key, text = key ++ "      (" ++ String.fromInt count ++ ")", enabled = True }
            in
            options
                |> Dict.toList
                |> List.map item

        opts =
            model.filter_options

        filter =
            model.filters_used

        sel_process =
            let
                opt_gen : ( String, Int ) -> Html msg
                opt_gen ( key, count ) =
                    option [ value key, selected (model.filters_used.wafer == key) ] [ text (key ++ " (" ++ String.fromInt count ++ ")") ]

                optionList =
                    opts
                        |> .processes
                        |> Dict.toList
                        |> List.map opt_gen
            in
            if Dict.size opts.processes > 1 then
                select [ class "filter", autofocus True, Html.Attributes.size (Dict.size opts.test_types), onChange ChangeWafer ] optionList

            else if Dict.size opts.processes == 1 then
                div [ class "filter" ]
                    [ text "Wafer:"
                    , br [] []
                    , opts
                        |> .processes
                        |> Dict.keys
                        |> List.head
                        |> Maybe.withDefault "default"
                        |> text
                    ]

            else
                div [ class "filter" ] [ text "nope" ]

        sel_dies =
            if Dict.size opts.dies > 1 then
                multiSelect { items = opt_items opts.dies, onChange = \s -> ChangeFilter { filter | dies = s } } [ class "filter", autofocus True, Html.Attributes.size (Dict.size opts.dies) ] filter.dies

            else if Dict.size opts.dies == 1 then
                div [ class "filter" ]
                    [ text "Die:"
                    , br [] []
                    , opts
                        |> .dies
                        |> Dict.keys
                        |> List.head
                        |> Maybe.withDefault "default"
                        |> text
                    ]

            else
                div [ class "filter" ] [ text "nope" ]

        sel_widths =
            if Dict.size opts.widths > 1 then
                multiSelect { items = opt_items opts.widths, onChange = \s -> ChangeFilter { filter | widths = s } } [ class "filter", autofocus True, Html.Attributes.size (Dict.size opts.widths) ] filter.widths

            else if Dict.size opts.widths == 1 then
                div [ class "filter" ]
                    [ text "Width:"
                    , br [] []
                    , opts
                        |> .widths
                        |> Dict.keys
                        |> List.head
                        |> Maybe.withDefault "default"
                        |> text
                    ]

            else
                div [ class "filter" ] [ text "nope" ]

        sel_lengths =
            if Dict.size opts.lengths > 1 then
                multiSelect { items = opt_items opts.lengths, onChange = \s -> ChangeFilter { filter | lengths = s } } [ class "filter", autofocus True, Html.Attributes.size (Dict.size opts.lengths) ] filter.lengths

            else if Dict.size opts.lengths == 1 then
                div [ class "filter" ]
                    [ text "Length:"
                    , br [] []
                    , opts
                        |> .lengths
                        |> Dict.keys
                        |> List.head
                        |> Maybe.withDefault "default"
                        |> text
                    ]

            else
                div [ class "filter" ] [ text "nope" ]

        sel_temps =
            if Dict.size opts.temps > 1 then
                multiSelect { items = opt_items opts.temps, onChange = \s -> ChangeFilter { filter | temps = s } } [ class "filter", autofocus True, Html.Attributes.size (Dict.size opts.temps) ] filter.temps

            else if Dict.size opts.temps == 1 then
                div [ class "filter" ]
                    [ text "Temperature:"
                    , br [] []
                    , opts
                        |> .temps
                        |> Dict.keys
                        |> List.head
                        |> Maybe.withDefault "default"
                        |> text
                    ]

            else
                div [ class "filter" ] [ text "nope" ]

        sel_test_types =
            let
                opt_gen : ( String, Int ) -> Html msg
                opt_gen ( key, count ) =
                    option [ value key, selected (model.filters_used.test_type == key) ] [ text (key ++ " (" ++ String.fromInt count ++ ")") ]

                optionList =
                    opts
                        |> .test_types
                        |> Dict.toList
                        |> List.map opt_gen
            in
            if Dict.size opts.test_types > 1 then
                select [ class "filter", autofocus True, Html.Attributes.size (Dict.size opts.test_types), onChange ChangeTestType ] optionList

            else if Dict.size opts.test_types == 1 then
                div [ class "filter" ]
                    [ text "Test type"
                    , br [] []
                    , opts
                        |> .test_types
                        |> Dict.keys
                        |> List.head
                        |> Maybe.withDefault "default"
                        |> text
                    ]

            else
                div [ class "filter" ] [ text "nope" ]

        sel_measurement_speeds =
            if Dict.size opts.measurement_speeds > 1 then
                multiSelect { items = opt_items opts.measurement_speeds, onChange = \s -> ChangeFilter { filter | measurement_speeds = s } } [ class "filter", autofocus True, Html.Attributes.size (Dict.size opts.measurement_speeds) ] filter.measurement_speeds

            else if Dict.size opts.measurement_speeds == 1 then
                div [ class "filter" ]
                    [ text "Measurement"
                    , br [] []
                    , text "speed:"
                    , br [] []
                    , opts
                        |> .measurement_speeds
                        |> Dict.keys
                        |> List.head
                        |> Maybe.withDefault "default"
                        |> text
                    ]

            else
                div [ class "never" ] [ text "nope" ]
    in
    [ sel_process
    , sel_dies
    , sel_temps
    , sel_widths
    , sel_lengths
    , sel_test_types
    , sel_measurement_speeds
    ]


measurementview : Model -> List (Html Msg)
measurementview model =
    let
        row : MeasurementCompact -> Html Msg
        row measurement =
            let
                entry =
                    Dict.get measurement.id model.selected_entries

                value =
                    case entry of
                        Just e ->
                            e.selected

                        Nothing ->
                            False
            in
            tr
                []
                [ td [] [ input [ onCheck (SelectEntry measurement.id), type_ "checkbox", checked value ] [] ]
                , td [ class "text_table" ] [ text measurement.id ]
                , td [ class "text_table" ]
                    [ measurement
                        |> .device
                        |> .wafer
                        |> Maybe.map Wafer.toString
                        |> Maybe.withDefault ""
                        |> text
                    ]
                , td [ class "text_table" ]
                    [ measurement
                        |> .device
                        |> .die
                        |> Maybe.map (\( a, b ) -> a ++ String.fromInt b)
                        |> Maybe.withDefault ""
                        |> text
                    ]
                , td [ class "number_table" ]
                    [ measurement
                        |> .device
                        |> .width
                        |> Maybe.map String.fromFloat
                        |> Maybe.withDefault ""
                        |> text
                    ]
                , td [ class "number_table" ]
                    [ measurement
                        |> .device
                        |> .length
                        |> Maybe.map String.fromFloat
                        |> Maybe.withDefault ""
                        |> text
                    ]
                , td [ class "text_table" ]
                    [ measurement
                        |> .device
                        |> .temperature
                        |> Maybe.map String.fromInt
                        |> Maybe.withDefault ""
                        |> text
                    ]
                , td [ class "text_table" ]
                    [ measurement
                        |> .test_parameter
                        |> .test_type
                        |> TestType.toString
                        |> text
                    ]
                , td [ class "text_table" ]
                    [ measurement
                        |> .test_parameter
                        |> .measurement_speed
                        |> MeasurementSpeed.toString
                        |> text
                    ]
                ]
    in
    [ tr []
        [ th []
            [ button [ onClick (ChangeAllEntries True) ] [ text "all" ]
            , br [] []
            , button [ onClick (ChangeAllEntries False) ] [ text "none" ]
            ]
        , th [] [ text "ID" ]
        , th [] [ text "Wafer" ]
        , th [] [ text "Die" ]
        , th [] [ text "Width", br [] [], text "(nm)" ]
        , th [] [ text "Length", br [] [], text "(nm)" ]
        , th [] [ text "Temp", br [] [], text "(K)" ]
        , th [] [ text "Test mode" ]
        , th [] [ text "Measurement", br [] [], text "speed" ]
        ]
    ]
        ++ (model
                |> .measurements
                |> List.map (lazy row)
           )


processoptionsview : Model -> List (Html Msg)
processoptionsview model =
    let
        ids_selected : List M_ID
        ids_selected =
            let
                filter_fn : M_ID -> Entry -> Bool
                filter_fn _ entry =
                    entry.selected
            in
            model
                |> .selected_entries
                |> Dict.filter filter_fn
                |> Dict.keys

        selected_measurements =
            let
                filter_fn : MeasurementCompact -> Bool
                filter_fn measurement =
                    List.member measurement.id ids_selected
            in
            model
                |> .measurements
                |> List.filter filter_fn

        test_type =
            model.filters_used.test_type

        wafer =
            model.filters_used.wafer

        measurement_speeds =
            List.Extra.uniqueBy MeasurementSpeed.toString (List.map (\m -> m.test_parameter.measurement_speed) selected_measurements)

        speed_selected_list =
            measurement_speeds
                |> List.map (\s -> span [ class "selection_option" ] [ text (MeasurementSpeed.toString s) ])

        dies =
            List.Extra.uniqueBy Die.toString (List.filterMap (\m -> m.device.die) selected_measurements)

        die_selected_list =
            dies
                |> List.map (\( s, i ) -> span [ class "selection_option" ] [ text ("(" ++ s ++ "," ++ String.fromInt i ++ ")") ])

        widths =
            List.sort (List.Extra.unique (List.filterMap (\m -> m.device.width) selected_measurements))

        width_selected_list =
            widths
                |> List.map (\w -> span [ class "selection_option" ] [ text (String.fromFloat w) ])

        lengths =
            List.sort (List.Extra.unique (List.filterMap (\m -> m.device.length) selected_measurements))

        length_selected_list =
            lengths
                |> List.map (\l -> span [ class "selection_option" ] [ text (String.fromFloat l) ])

        possible_process_options : List ProcessingType
        possible_process_options =
            if test_type == "Sampling" then
                if model.normalize_data then
                    [ ProcessingType.raw, ProcessingType.id_bins_normalized, ProcessingType.id_normalized_versus_time ]

                else
                    [ ProcessingType.raw, ProcessingType.id_bins, ProcessingType.ts_bins, ProcessingType.id_versus_time, ProcessingType.psd ]

            else
                [ ProcessingType.raw, ProcessingType.id_for_swept_VDS_and_VGS ]

        process_options =
            let
                combine =
                    if List.length widths > 1 || List.length lengths > 1 || List.length dies > 1 then
                        div [ class "process_selector" ]
                            [ input [ type_ "checkbox", checked model.combine_data, onCheck ChangeCombineData ] []
                            , span [] [ text "Combine the selected data into one plot" ]
                            ]

                    else
                        text ""

                normalize =
                    if test_type == "Sampling" then
                        div [ class "process_selector" ]
                            [ input [ type_ "checkbox", checked model.normalize_data, onCheck ChangeNormalizeData ] []
                            , span [] [ text "Normalize the data by dividing by the average current" ]
                            ]

                    else
                        text ""

                opt : ProcessingType -> List (Html Msg)
                opt kind =
                    let
                        check =
                            List.member kind model.process_options_selected
                    in
                    [ div [ class "process_selector" ]
                        [ input [ type_ "checkbox", checked check, onCheck (SelectProcessType kind) ] []
                        , span [] [ text (ProcessingType.toString kind) ]
                        ]
                    ]
            in
            [ combine ]
                ++ [ normalize ]
                ++ List.concatMap opt possible_process_options
                ++ [ button [ onClick ProcessData ] [ text "Process" ] ]
    in
    [ div [ id "param_used" ]
        [ h1 [] [ text "Parameters used:" ]
        , div []
            [ text "Measurement mode:"
            , span [ class "selection_option" ] [ text test_type ]
            ]
        , div [] ([ text "Measurement speed:" ] ++ speed_selected_list)
        , div []
            [ text "Wafer:"
            , span [ class "selection_option" ] [ text wafer ]
            ]
        , div [] ([ text "Die:" ] ++ die_selected_list)
        , div [] ([ text "Width:" ] ++ width_selected_list)
        , div [] ([ text "Length:" ] ++ length_selected_list)
        ]
    , hr [] []
    , div [ id "process_options" ] process_options
    ]


testdataselectionview : Model -> List (Html Msg)
testdataselectionview model =
    let
        ids_selected : List M_ID
        ids_selected =
            let
                filter_fn : M_ID -> Entry -> Bool
                filter_fn _ entry =
                    entry.selected
            in
            model
                |> .selected_entries
                |> Dict.filter filter_fn
                |> Dict.keys

        selected_measurements =
            let
                filter_fn : MeasurementCompact -> Bool
                filter_fn measurement =
                    List.member measurement.id ids_selected
            in
            model
                |> .measurements
                |> List.filter filter_fn

        singleview measurement =
            let
                wafer =
                    case measurement.device.wafer of
                        Just w ->
                            [ tr [] [ td [] [ text "Wafer" ], td [] [ text (Wafer.toString w) ] ] ]

                        Nothing ->
                            []

                die =
                    case measurement.device.die of
                        Just d ->
                            [ tr [] [ td [] [ text "Die" ], td [] [ text (Die.toString d) ] ] ]

                        Nothing ->
                            []

                temp =
                    case measurement.device.temperature of
                        Just t ->
                            [ tr [] [ td [] [ text "Temp." ], td [] [ text (String.fromInt t ++ "°K") ] ] ]

                        Nothing ->
                            []

                width =
                    case measurement.device.width of
                        Just w ->
                            [ tr [] [ td [] [ text "Width" ], td [] [ text (String.fromFloat w ++ "nm") ] ] ]

                        Nothing ->
                            []

                length =
                    case measurement.device.length of
                        Just l ->
                            [ tr [] [ td [] [ text "Length" ], td [] [ text (String.fromFloat l ++ "nm") ] ] ]

                        Nothing ->
                            []

                testtype =
                    let
                        string =
                            TestType.toString measurement.test_parameter.test_type
                    in
                    [ tr [] [ td [] [ text "Type" ], td [] [ text string ] ] ]

                speed =
                    let
                        string =
                            MeasurementSpeed.toString measurement.test_parameter.measurement_speed
                    in
                    [ tr [] [ td [] [ text "Speed" ], td [] [ text string ] ] ]

                ad_aperture =
                    case measurement.test_parameter.ad_aperture of
                        Just ad ->
                            [ tr [] [ td [] [ text "AD aperture" ], td [] [ text (String.fromFloat ad) ] ] ]

                        Nothing ->
                            []

                filter_factor =
                    case measurement.test_parameter.filter_factor of
                        Just ff ->
                            [ tr [] [ td [] [ text "Filter factor" ], td [] [ text (String.fromFloat ff) ] ] ]

                        Nothing ->
                            []

                interval_time =
                    case measurement.test_parameter.interval_time of
                        Just it ->
                            [ tr [] [ td [] [ text "Interval time" ], td [] [ text (String.fromFloat it) ] ] ]

                        Nothing ->
                            []

                sweep_delay_time =
                    case measurement.test_parameter.sweep_delay_time of
                        Just sdt ->
                            [ tr [] [ td [] [ text "Sweep delay time" ], td [] [ text (String.fromFloat sdt) ] ] ]

                        Nothing ->
                            []

                hold_time =
                    let
                        string =
                            String.fromFloat measurement.test_parameter.hold_time
                    in
                    [ tr [] [ td [] [ text "Hold time" ], td [] [ text string ] ] ]

                instruments_list amount_of_measurements =
                    let
                        terminals =
                            measurement.terminals

                        terminal =
                            let
                                list =
                                    List.map (\t -> Terminal.toString t.terminal) terminals

                                inner t =
                                    td [ class "text_table" ] [ text t ]
                            in
                            tr [] (td [ class "text_table" ] [ text "Terminal" ] :: List.map inner list)

                        instrument =
                            let
                                list =
                                    List.map (\t -> Instrument.toString t.instrument) terminals

                                inner inst =
                                    td [ class "text_table" ] [ text inst ]
                            in
                            tr [] (td [ class "text_table" ] [ text "Instrument" ] :: List.map inner list)

                        opmode =
                            let
                                list =
                                    terminals
                                        |> List.map
                                            (\t ->
                                                t
                                                    |> .operational_mode
                                                    |> .op_type
                                                    |> OpModeType.toString
                                                    |> String.words
                                                    |> List.map (\s -> text s)
                                                    |> List.intersperse (br [] [])
                                            )

                                inner op =
                                    td [ class "text_table" ] op
                            in
                            tr [] (td [ class "text_table" ] [ text "Operational", br [] [], text "mode", br [] [] ] :: List.map inner list)

                        biasorstart =
                            let
                                t_to_bs t =
                                    let
                                        bias =
                                            t.operational_mode.bias

                                        start =
                                            t.operational_mode.start
                                    in
                                    case ( bias, start ) of
                                        ( Just b, _ ) ->
                                            String.fromFloat b

                                        ( _, Just s ) ->
                                            String.fromFloat s

                                        ( _, _ ) ->
                                            "-"

                                list =
                                    List.map t_to_bs terminals

                                inner bs =
                                    if bs == "-" then
                                        td [ class "text_table" ] [ text bs ]

                                    else
                                        td [ class "number_table" ] [ text bs ]
                            in
                            tr [] (td [ class "text_table" ] [ text "Bias / Start" ] :: List.map inner list)

                        stop =
                            let
                                t_to_s t =
                                    let
                                        stp =
                                            t.operational_mode.stop
                                    in
                                    case stp of
                                        Just s ->
                                            String.fromFloat s

                                        Nothing ->
                                            "-"

                                list =
                                    List.map t_to_s terminals

                                inner s =
                                    if s == "-" then
                                        td [ class "text_table" ] [ text s ]

                                    else
                                        td [ class "number_table" ] [ text s ]
                            in
                            if List.length (List.filterMap (\t -> t.operational_mode.stop) terminals) > 0 then
                                tr [] (td [ class "text_table" ] [ text "Stop" ] :: List.map inner list)

                            else
                                text ""

                        stepsize =
                            let
                                t_to_s t =
                                    let
                                        step =
                                            t.operational_mode.stepsize
                                    in
                                    case step of
                                        Just s ->
                                            String.fromFloat s

                                        Nothing ->
                                            "-"

                                list =
                                    List.map t_to_s terminals

                                inner s =
                                    if s == "-" then
                                        td [ class "text_table" ] [ text s ]

                                    else
                                        td [ class "number_table" ] [ text s ]
                            in
                            if List.length (List.filterMap (\t -> t.operational_mode.stop) terminals) > 0 then
                                tr [] (td [ class "text_table" ] [ text "Stepsize" ] :: List.map inner list)

                            else
                                text ""

                        compliance =
                            let
                                t_to_c c =
                                    case c.compliance of
                                        Just s ->
                                            String.fromFloat s

                                        Nothing ->
                                            "-"

                                list =
                                    List.map t_to_c terminals

                                inner s =
                                    if s == "-" then
                                        td [ class "text_table" ] [ text s ]

                                    else
                                        td [ class "number_table" ] [ text s ]
                            in
                            if List.length (List.filterMap (\t -> t.compliance) terminals) > 0 then
                                tr [] (td [ class "text_table" ] [ text "Compliance" ] :: List.map inner list)

                            else
                                text ""

                        voltage =
                            tr [] (td [ class "text_table" ] [ text "Voltage" ] :: List.repeat amount_of_measurements (td [] []))

                        vrange =
                            let
                                t_to_v v =
                                    case v.voltage_range of
                                        Just s ->
                                            VoltageRange.toString s

                                        Nothing ->
                                            "-"

                                list =
                                    List.map t_to_v terminals

                                inner s =
                                    td [ class "text_table" ] [ text s ]
                            in
                            if List.length (List.filterMap (\t -> t.voltage_range) terminals) > 0 then
                                tr [] (td [ class "text_table" ] [ text "Range" ] :: List.map inner list)

                            else
                                text ""

                        vmeasured =
                            let
                                t_to_v v =
                                    case v.voltage of
                                        Just s ->
                                            UnitMeasured.toString s

                                        Nothing ->
                                            "-"

                                list =
                                    List.map t_to_v terminals

                                inner s =
                                    td [ class "text_table" ] [ text s ]
                            in
                            if List.length (List.filterMap (\t -> t.voltage) terminals) > 0 then
                                tr [] (td [ class "text_table" ] [ text "Measured" ] :: List.map inner list)

                            else
                                text ""

                        crange =
                            let
                                list =
                                    terminals
                                        |> List.map
                                            (\t ->
                                                t
                                                    |> .current_range
                                                    |> Maybe.map CurrentRange.toString
                                                    |> Maybe.withDefault "-"
                                                    |> String.words
                                                    |> List.map (\s -> text s)
                                                    |> List.intersperse (br [] [])
                                            )

                                inner s =
                                    td [ class "text_table" ] s
                            in
                            if List.length (List.filterMap (\t -> t.current_range) terminals) > 0 then
                                tr [] (td [ class "text_table" ] [ text "Range" ] :: List.map inner list)

                            else
                                text ""

                        cmeasured =
                            let
                                t_to_c c =
                                    case c.current of
                                        Just s ->
                                            UnitMeasured.toString s

                                        Nothing ->
                                            "-"

                                list =
                                    List.map t_to_c terminals

                                inner s =
                                    td [ class "text_table" ] [ text s ]
                            in
                            if List.length (List.filterMap (\t -> t.current) terminals) > 0 then
                                tr [] (td [ class "text_table" ] [ text "Measured" ] :: List.map inner list)

                            else
                                text ""

                        current =
                            tr [] (td [ class "text_table" ] [ text "Current" ] :: List.repeat amount_of_measurements (td [] []))
                    in
                    [ terminal
                    , instrument
                    , opmode
                    , biasorstart
                    , stop
                    , stepsize
                    , compliance
                    , voltage
                    , vmeasured
                    , vrange
                    , current
                    , cmeasured
                    , crange
                    ]

                data_selection =
                    let
                        testdata_without_time =
                            List.filter
                                (\t ->
                                    case Terminal.toString_concise t.terminal of
                                        "T" ->
                                            False

                                        _ ->
                                            True
                                )
                                measurement.test_data

                        testdata_time =
                            List.filter
                                (\t ->
                                    case Terminal.toString_concise t.terminal of
                                        "T" ->
                                            True

                                        _ ->
                                            False
                                )
                                measurement.test_data

                        colgroup_amount =
                            case List.head testdata_without_time of
                                Just data ->
                                    data.count

                                Nothing ->
                                    0

                        colgroup_range =
                            List.range 1 colgroup_amount

                        cols _ =
                            let
                                inner testdata =
                                    th [ class "text_table" ] [ text (TestDataCompact.toString_concise testdata) ]
                            in
                            List.map inner testdata_without_time

                        colgroup i =
                            th [ class "text_table", colspan (List.length testdata_without_time) ] [ input [ type_ "checkbox" ] [], text ("Series " ++ String.fromInt i) ]
                    in
                    if measurement.test_parameter.test_type == TestType.sampling then
                        [ tr [] (th [ class "text_table" ] [ text "" ] :: List.map colgroup colgroup_range)
                        , tr [] ([ th [ class "text_table" ] [ text "T" ] ] ++ List.concatMap cols colgroup_range)
                        ]

                    else
                        [ tr [] (List.map colgroup colgroup_range)
                        , tr [] (List.concatMap cols colgroup_range)
                        ]
            in
            div [ class "testdata_container" ]
                [ div [ class "testdata_top" ]
                    [ div [ class "testdata_header" ]
                        [ text "ID"
                        , br [] []
                        , h1 [] [ text measurement.id ]
                        , text "Timestamp"
                        , br [] []
                        , text (TimeStamp.toString measurement.test_time_stamp)
                        ]
                    , table [ class "testdata_device" ]
                        (tr [] [ th [ colspan 2 ] [ text "Device" ] ]
                            :: wafer
                            ++ die
                            ++ temp
                            ++ width
                            ++ length
                        )
                    , table
                        [ class "testdata_measurement" ]
                        (tr [] [ th [ colspan 2 ] [ text "Test Parameters" ] ]
                            :: testtype
                            ++ speed
                            ++ ad_aperture
                            ++ filter_factor
                            ++ interval_time
                            ++ sweep_delay_time
                            ++ hold_time
                        )
                    ]
                , table [ class "testdata_terminals" ]
                    (tr [] [ th [ colspan (1 + List.length measurement.terminals), class "text_table" ] [ text "Terminals" ] ]
                        :: instruments_list (List.length measurement.terminals)
                    )
                , table [ class "testdata_data" ] data_selection
                ]
    in
    List.map singleview selected_measurements


view : Model -> Html Msg
view model =
    let
        amount_selected =
            let
                filter_fn : M_ID -> Entry -> Bool
                filter_fn _ entry =
                    entry.selected
            in
            model
                |> .selected_entries
                |> Dict.filter filter_fn
                |> Dict.size

        selectorprocess =
            case model.page of
                SelectPage ->
                    [ div [ id "filter_container" ] (filterview model)
                    , table [ id "data_container" ] (measurementview model)
                    ]

                ProcessPage ->
                    [ div [ id "process_options_container" ] (processoptionsview model)
                    , div [ id "selected_testdata_list" ] (testdataselectionview model)
                    ]
    in
    div
        [ class "container" ]
        (List.append
            [ div [ id "nav_bar" ]
                [ h1 [ id "title" ]
                    [ text "C Karaliolios's Data Library tool for the Keithley 4200" ]
                , div [ id "page_selection" ]
                    [ button
                        [ onClick (SendToRust "Init") ]
                        [ text "Reset" ]
                    , button
                        [ onClick ToSelectPage ]
                        [ text "Select Data" ]
                    , button
                        [ onClick ToProcessPage, disabled (amount_selected == 0) ]
                        [ text "Process Data" ]
                    ]
                ]
            ]
            selectorprocess
        )



-- SUBSCRIPTIONS


decodeValue : Encode.Value -> Msg
decodeValue x =
    let
        result =
            Decode.decodeValue FromRust.decode x
    in
    case result of
        Ok fromrust ->
            UpdateModel fromrust

        Err err ->
            SendToRust (Decode.errorToString err)


subscriptions : Model -> Sub Msg
subscriptions model =
    fromRust decodeValue
