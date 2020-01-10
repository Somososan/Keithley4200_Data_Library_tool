module UnitMeasured exposing (UnitMeasured, decode, encode, toString)

import Json.Decode as Decode
import Json.Encode as Encode


type UnitMeasured
    = Measured
    | Programmed


encode : UnitMeasured -> Encode.Value
encode um =
    case um of
        Measured ->
            Encode.string "Measured"

        Programmed ->
            Encode.string "Programmed"


decode : Decode.Decoder UnitMeasured
decode =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "Measured" ->
                        Decode.succeed Measured

                    "Programmed" ->
                        Decode.succeed Programmed

                    _ ->
                        Decode.fail "Error parsing UnitMeasured"
            )


toString : UnitMeasured -> String
toString um =
    case um of
        Measured ->
            "Measured"

        Programmed ->
            "Programmed"
