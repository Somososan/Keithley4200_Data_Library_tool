module FromRust exposing (FromRust, decode)

import FilterOptions exposing (FilterOptions)
import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import MeasurementCompact exposing (MeasurementCompact)
import RustTask exposing (RustTask)


type alias FromRust =
    { message_nr : Int
    , task_done : RustTask
    , measurements : List MeasurementCompact
    , filter_options : FilterOptions
    }


decode : Decode.Decoder FromRust
decode =
    Decode.succeed FromRust
        |> required "message_nr" Decode.int
        |> required "task_done" RustTask.decode
        |> required "measurements" (Decode.list MeasurementCompact.decode)
        |> required "filter_options" FilterOptions.decode
