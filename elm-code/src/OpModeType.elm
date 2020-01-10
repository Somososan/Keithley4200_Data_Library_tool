module OpModeType exposing (OpModeType, decode, encode, toString)

import Json.Decode as Decode
import Json.Encode as Encode


type OpModeType
    = VoltageBias
    | VoltageLinearSweep
    | VoltageStep
    | CurrentBias
    | CurrentLinearSweep
    | CurrentStep
    | Common
    | Floating


encode : OpModeType -> Encode.Value
encode opmode =
    case opmode of
        VoltageBias ->
            Encode.string "VoltageBias"

        VoltageLinearSweep ->
            Encode.string "VoltageLinearSweep"

        VoltageStep ->
            Encode.string "VoltageStep"

        CurrentBias ->
            Encode.string "CurrentBias"

        CurrentLinearSweep ->
            Encode.string "CurrentLinearSweep"

        CurrentStep ->
            Encode.string "CurrentStep"

        Common ->
            Encode.string "Common"

        Floating ->
            Encode.string "Floating"


decode : Decode.Decoder OpModeType
decode =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "VoltageBias" ->
                        Decode.succeed VoltageBias

                    "VoltageLinearSweep" ->
                        Decode.succeed VoltageLinearSweep

                    "VoltageStep" ->
                        Decode.succeed VoltageStep

                    "CurrentBias" ->
                        Decode.succeed CurrentBias

                    "CurrentLinearSweep" ->
                        Decode.succeed CurrentLinearSweep

                    "CurrentStep" ->
                        Decode.succeed CurrentStep

                    "Common" ->
                        Decode.succeed Common

                    "Floating" ->
                        Decode.succeed Floating

                    _ ->
                        Decode.fail "Error parsing OpModeType"
            )


toString : OpModeType -> String
toString opmode =
    case opmode of
        VoltageBias ->
            "Voltage bias"

        VoltageLinearSweep ->
            "Voltage linear sweep"

        VoltageStep ->
            "Voltage step"

        CurrentBias ->
            "Current bias"

        CurrentLinearSweep ->
            "Current linear sweep"

        CurrentStep ->
            "Current step"

        Common ->
            "Common"

        Floating ->
            "Floating"
