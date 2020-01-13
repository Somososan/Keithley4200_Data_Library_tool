module TestDataCompact exposing (TestDataCompact, decode, encode, toString, toString_concise)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode
import Terminal exposing (Terminal)
import Unit exposing (Unit)


type alias TestDataCompact =
    { terminal : Terminal
    , unit : Unit
    , count : Int
    }


encode : TestDataCompact -> Encode.Value
encode test_data =
    Encode.object
        [ ( "terminal", Terminal.encode test_data.terminal )
        , ( "unit", Unit.encode test_data.unit )
        , ( "count", Encode.int test_data.count )
        ]


decode : Decode.Decoder TestDataCompact
decode =
    Decode.succeed TestDataCompact
        |> required "terminal" Terminal.decode
        |> required "unit" Unit.decode
        |> required "count" Decode.int


toString : TestDataCompact -> String
toString test_data =
    Terminal.toString test_data.terminal ++ " " ++ Unit.toString test_data.unit


toString_concise : TestDataCompact -> String
toString_concise test_data =
    Unit.toString_concise test_data.unit ++ Terminal.toString_concise test_data.terminal
