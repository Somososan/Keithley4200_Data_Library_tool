module CurrentRange exposing (CurrentRange, decode, encode, toString)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (optional, resolve)
import Json.Encode as Encode


type CurrentRange
    = LimitedAuto String
    | Auto


encode : CurrentRange -> Encode.Value
encode crange =
    case crange of
        LimitedAuto range ->
            Encode.object [ ( "LimitedAuto", Encode.string range ) ]

        Auto ->
            Encode.string "Auto"


decode : Decode.Decoder CurrentRange
decode =
    let
        limited : String -> Decode.Decoder CurrentRange
        limited string =
            if string == "Auto" then
                Decode.succeed Auto

            else
                Decode.succeed (LimitedAuto string)
    in
    Decode.succeed limited
        |> optional "LimitedAuto" Decode.string "Auto"
        |> resolve


toString : CurrentRange -> String
toString crange =
    case crange of
        LimitedAuto range ->
            "Limited auto " ++ range

        Auto ->
            "Auto"
