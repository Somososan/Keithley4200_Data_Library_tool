module ToRust exposing (filter, init, log, process)

import FilterQuery exposing (FilterQuery)
import Json.Encode as Encode
import ProcessQuery exposing (ProcessQuery)


init : Encode.Value
init =
    Encode.object [ ( "torust", Encode.string "Init" ) ]


log : String -> Encode.Value
log string =
    Encode.object
        [ ( "torust", Encode.string "Log" )
        , ( "content", Encode.string string )
        ]


filter : FilterQuery -> Encode.Value
filter query =
    Encode.object
        [ ( "torust", Encode.string "Filter" )
        , ( "content", FilterQuery.encode query )
        ]


process : ProcessQuery -> Encode.Value
process query =
    Encode.object
        [ ( "torust", Encode.string "Process" )
        , ( "content", ProcessQuery.encode query )
        ]
