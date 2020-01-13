module Unit exposing (Unit, decode, encode, toString, toString_concise)

import Json.Decode as Decode
import Json.Encode as Encode


type Unit
    = Voltage
    | Current
    | Seconds


encode : Unit -> Encode.Value
encode unit =
    case unit of
        Voltage ->
            Encode.string "Voltage"

        Current ->
            Encode.string "Current"

        Seconds ->
            Encode.string "Seconds"


decode : Decode.Decoder Unit
decode =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "Voltage" ->
                        Decode.succeed Voltage

                    "Current" ->
                        Decode.succeed Current

                    "Seconds" ->
                        Decode.succeed Seconds

                    _ ->
                        Decode.fail "Error parsing Terminal"
            )


toString : Unit -> String
toString unit =
    case unit of
        Voltage ->
            "Voltage"

        Current ->
            "Current"

        Seconds ->
            "(s)"


toString_concise : Unit -> String
toString_concise unit =
    case unit of
        Voltage ->
            "V"

        Current ->
            "I"

        Seconds ->
            ""
