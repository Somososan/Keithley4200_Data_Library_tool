module Instrument exposing (Instrument, decode, encode, toString)

import Json.Decode as Decode
import Json.Encode as Encode


type Instrument
    = SMU1
    | SMU2
    | SMU3
    | SMU4
    | GNDU
    | PMU1
    | PMU2
    | PMU3
    | PMU4


encode : Instrument -> Encode.Value
encode instrument =
    case instrument of
        SMU1 ->
            Encode.string "SMU1"

        SMU2 ->
            Encode.string "SMU2"

        SMU3 ->
            Encode.string "SMU3"

        SMU4 ->
            Encode.string "SMU4"

        GNDU ->
            Encode.string "GNDU"

        PMU1 ->
            Encode.string "PMU1"

        PMU2 ->
            Encode.string "PMU2"

        PMU3 ->
            Encode.string "PMU3"

        PMU4 ->
            Encode.string "PMU4"


decode : Decode.Decoder Instrument
decode =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "SMU1" ->
                        Decode.succeed SMU1

                    "SMU2" ->
                        Decode.succeed SMU2

                    "SMU3" ->
                        Decode.succeed SMU3

                    "SMU4" ->
                        Decode.succeed SMU4

                    "GNDU" ->
                        Decode.succeed GNDU

                    "PMU1" ->
                        Decode.succeed PMU1

                    "PMU2" ->
                        Decode.succeed PMU2

                    "PMU3" ->
                        Decode.succeed PMU3

                    "PMU4" ->
                        Decode.succeed PMU4

                    _ ->
                        Decode.fail "Error parsing Instrument"
            )


toString : Instrument -> String
toString instrument =
    case instrument of
        SMU1 ->
            "SMU1"

        SMU2 ->
            "SMU2"

        SMU3 ->
            "SMU3"

        SMU4 ->
            "SMU4"

        GNDU ->
            "GNDU"

        PMU1 ->
            "PMU1"

        PMU2 ->
            "PMU2"

        PMU3 ->
            "PMU3"

        PMU4 ->
            "PMU4"
