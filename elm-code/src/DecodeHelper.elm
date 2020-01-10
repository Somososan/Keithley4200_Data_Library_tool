module DecodeHelper exposing (tuple2, tuple3)

import Json.Decode as Decode exposing (Decoder, map2, map3)
import Tuple3 exposing (join)


tuple2 : Decoder a -> Decoder b -> Decoder ( a, b )
tuple2 a b =
    Decode.map2 Tuple.pair
        (Decode.index 0 a)
        (Decode.index 1 b)


tuple3 : Decoder a -> Decoder b -> Decoder c -> Decoder ( a, b, c )
tuple3 a b c =
    Decode.map3 Tuple3.join
        (Decode.index 0 a)
        (Decode.index 1 b)
        (Decode.index 2 c)
