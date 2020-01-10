module RustTask exposing (RustTask, decode, init)

import Json.Decode as Decode


type RustTask
    = Init
    | Filtering
    | Processing


init : RustTask
init =
    Init


decode : Decode.Decoder RustTask
decode =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "Init" ->
                        Decode.succeed Init

                    "Filtering" ->
                        Decode.succeed Filtering

                    "Processing" ->
                        Decode.succeed Processing

                    _ ->
                        Decode.fail "Error parsing Task"
            )
