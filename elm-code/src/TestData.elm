module TestData exposing (TestData, decode, encode, toString, toString_concise)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode
import Terminal exposing (Terminal)
import Unit exposing (Unit)


type alias TestData =
    { terminal : Terminal
    , unit : Unit
    , data : List (List Float)
    }


encode : TestData -> Encode.Value
encode test_data =
    Encode.object
        [ ( "terminal", Terminal.encode test_data.terminal )
        , ( "unit", Unit.encode test_data.unit )
        , ( "data", Encode.list (Encode.list Encode.float) test_data.data )
        ]


decode : Decode.Decoder TestData
decode =
    Decode.succeed TestData
        |> required "terminal" Terminal.decode
        |> required "unit" Unit.decode
        |> required "count" (Decode.list (Decode.list Decode.float))


toString : TestData -> String
toString test_data =
    Terminal.toString test_data.terminal ++ " " ++ Unit.toString test_data.unit


toString_concise : TestData -> String
toString_concise test_data =
    Unit.toString_concise test_data.unit ++ Terminal.toString_concise test_data.terminal
