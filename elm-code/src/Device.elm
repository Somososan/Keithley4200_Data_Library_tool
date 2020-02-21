module Device exposing (Device, decode, encode)

import DecodeHelper
import EncodeHelper
import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode
import Wafer exposing (Wafer)


type alias Device =
    { wafer : Maybe Wafer
    , die : Maybe String
    , temperature : Maybe Int
    , width : Maybe Float
    , length : Maybe Float
    }


encode : Device -> Encode.Value
encode device =
    Encode.object
        [ ( "wafer", EncodeHelper.maybe Wafer.encode device.wafer )
        , ( "die", EncodeHelper.maybe Encode.string device.die )
        , ( "temperature", EncodeHelper.maybe Encode.int device.temperature )
        , ( "width", EncodeHelper.maybe Encode.float device.width )
        , ( "length", EncodeHelper.maybe Encode.float device.length )
        ]


decode : Decode.Decoder Device
decode =
    Decode.succeed Device
        |> required "wafer" (Decode.nullable Wafer.decode)
        |> required "die" (Decode.nullable Decode.string)
        |> required "temperature" (Decode.nullable Decode.int)
        |> required "width" (Decode.nullable Decode.float)
        |> required "length" (Decode.nullable Decode.float)
