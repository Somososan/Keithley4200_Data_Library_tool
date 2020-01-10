module EncodeHelper exposing (maybe, tuple2, tuple3)

import Json.Encode as Encode


maybe : (a -> Encode.Value) -> Maybe a -> Encode.Value
maybe fn mbe =
    case mbe of
        Just a ->
            fn a

        Nothing ->
            Encode.null


tuple2 : (a -> Encode.Value) -> (b -> Encode.Value) -> ( a, b ) -> Encode.Value
tuple2 enc1 enc2 ( val1, val2 ) =
    Encode.list identity [ enc1 val1, enc2 val2 ]


tuple3 : (a -> Encode.Value) -> (b -> Encode.Value) -> (c -> Encode.Value) -> ( a, b, c ) -> Encode.Value
tuple3 enc1 enc2 enc3 ( val1, val2, val3 ) =
    Encode.list identity [ enc1 val1, enc2 val2, enc3 val3 ]
