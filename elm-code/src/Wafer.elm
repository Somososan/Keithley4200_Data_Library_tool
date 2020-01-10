module Wafer exposing (Wafer, decode, encode, toString)

import Json.Decode as Decode
import Json.Encode as Encode


type Wafer
    = MINOXG
    | GF22


encode : Wafer -> Encode.Value
encode process =
    case process of
        MINOXG ->
            Encode.object [ ( "wafer", Encode.string "MINOXG" ) ]

        GF22 ->
            Encode.object [ ( "wafer", Encode.string "GF22" ) ]


decode : Decode.Decoder Wafer
decode =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "MINOXG" ->
                        Decode.succeed MINOXG

                    "GF22" ->
                        Decode.succeed GF22

                    _ ->
                        Decode.fail "Error parsing Wafer"
            )


toString : Wafer -> String
toString wafer =
    case wafer of
        MINOXG ->
            "MINOXG"

        GF22 ->
            "GF22"
