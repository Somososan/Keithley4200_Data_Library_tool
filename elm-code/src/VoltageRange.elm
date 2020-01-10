module VoltageRange exposing (VoltageRange, decode, encode, toString)

import Json.Decode as Decode
import Json.Encode as Encode


type VoltageRange
    = BestFixed


encode : VoltageRange -> Encode.Value
encode vrange =
    case vrange of
        BestFixed ->
            Encode.string "BestFixed"


decode : Decode.Decoder VoltageRange
decode =
    Decode.succeed BestFixed


toString : VoltageRange -> String
toString vrange =
    case vrange of
        BestFixed ->
            "Best fixed"
