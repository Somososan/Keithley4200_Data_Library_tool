module ToRust exposing (filter, init, log, process)

import FilterQuery exposing (FilterQuery)
import Json.Encode as Encode
import ProcessQuery exposing (ProcessQuery)


init : Encode.Value
init =
    Encode.string "Init"


log : String -> Encode.Value
log string =
    Encode.object [ ( "Log", Encode.string string ) ]


filter : FilterQuery -> Encode.Value
filter query =
    Encode.object [ ( "Filter", Encode.object [ ( "FilterQuery", FilterQuery.encode query ) ] ) ]


process : ProcessQuery -> Encode.Value
process query =
    Encode.object [ ( "Process", Encode.object [ ( "ProcessQuery", ProcessQuery.encode query ) ] ) ]
