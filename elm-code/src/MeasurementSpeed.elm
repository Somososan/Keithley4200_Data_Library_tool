module MeasurementSpeed exposing (MeasurementSpeed, decode, encode, toString)

import Json.Decode as Decode
import Json.Encode as Encode


type MeasurementSpeed
    = Fast
    | Normal
    | Quiet
    | Custom


encode : MeasurementSpeed -> Encode.Value
encode speed =
    case speed of
        Fast ->
            Encode.string "Fast"

        Normal ->
            Encode.string "Normal"

        Quiet ->
            Encode.string "Quiet"

        Custom ->
            Encode.string "Custom"


decode : Decode.Decoder MeasurementSpeed
decode =
    let
        fn string =
            case string of
                "Fast" ->
                    Decode.succeed Fast

                "Normal" ->
                    Decode.succeed Normal

                "Quiet" ->
                    Decode.succeed Quiet

                "Custom" ->
                    Decode.succeed Custom

                _ ->
                    Decode.fail "Error parsing MeasurementSpeed"
    in
    Decode.string
        |> Decode.andThen fn


toString : MeasurementSpeed -> String
toString speed =
    case speed of
        Fast ->
            "Fast"

        Normal ->
            "Normal"

        Quiet ->
            "Quiet"

        Custom ->
            "Custom"
