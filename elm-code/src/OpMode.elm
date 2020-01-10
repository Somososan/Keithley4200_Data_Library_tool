module OpMode exposing (OpMode, decode, encode)

import EncodeHelper
import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode
import OpModeType exposing (OpModeType, decode, encode)


type alias OpMode =
    { op_type : OpModeType
    , bias : Maybe Float
    , start : Maybe Float
    , stop : Maybe Float
    , stepsize : Maybe Float
    }


encode : OpMode -> Encode.Value
encode opmode =
    Encode.object
        [ ( "op_type", OpModeType.encode opmode.op_type )
        , ( "bias", EncodeHelper.maybe Encode.float opmode.bias )
        , ( "start", EncodeHelper.maybe Encode.float opmode.start )
        , ( "stop", EncodeHelper.maybe Encode.float opmode.stop )
        , ( "stepsize", EncodeHelper.maybe Encode.float opmode.stepsize )
        ]


decode : Decode.Decoder OpMode
decode =
    Decode.succeed OpMode
        |> required "op_type" OpModeType.decode
        |> required "bias" (Decode.nullable Decode.float)
        |> required "start" (Decode.nullable Decode.float)
        |> required "stop" (Decode.nullable Decode.float)
        |> required "stepsize" (Decode.nullable Decode.float)
