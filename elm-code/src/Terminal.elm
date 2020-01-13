module Terminal exposing (Terminal, decode, encode, toString, toString_concise)

import Json.Decode as Decode
import Json.Encode as Encode


type Terminal
    = Gate
    | Drain
    | Source
    | Bulk
    | Time


encode : Terminal -> Encode.Value
encode terminal =
    case terminal of
        Gate ->
            Encode.string "Gate"

        Drain ->
            Encode.string "Drain"

        Source ->
            Encode.string "Source"

        Bulk ->
            Encode.string "Bulk"

        Time ->
            Encode.string "Time"


decode : Decode.Decoder Terminal
decode =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "Gate" ->
                        Decode.succeed Gate

                    "Drain" ->
                        Decode.succeed Drain

                    "Source" ->
                        Decode.succeed Source

                    "Bulk" ->
                        Decode.succeed Bulk

                    "Time" ->
                        Decode.succeed Time

                    _ ->
                        Decode.fail "Error parsing Terminal"
            )


toString : Terminal -> String
toString terminal =
    case terminal of
        Gate ->
            "Gate"

        Drain ->
            "Drain"

        Source ->
            "Source"

        Bulk ->
            "Bulk"

        Time ->
            "Time"


toString_concise : Terminal -> String
toString_concise terminal =
    case terminal of
        Gate ->
            "g"

        Drain ->
            "d"

        Source ->
            "s"

        Bulk ->
            "b"

        Time ->
            "T"
