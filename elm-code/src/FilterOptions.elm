module FilterOptions exposing (FilterOptions, decode, empty, encode)

import Dict exposing (Dict)
import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode


type alias FilterOptions =
    { sheet_names : Dict String Int
    , widths : Dict String Int
    , lengths : Dict String Int
    , temps : Dict String Int
    , processes : Dict String Int
    , dies : Dict String Int
    , test_types : Dict String Int
    , measurement_speeds : Dict String Int
    , dates : Dict String Int
    }


encode : FilterOptions -> Encode.Value
encode query =
    Encode.object
        [ ( "sheet_names", Encode.dict identity Encode.int query.sheet_names )
        , ( "widths", Encode.dict identity Encode.int query.widths )
        , ( "lengths", Encode.dict identity Encode.int query.lengths )
        , ( "temps", Encode.dict identity Encode.int query.temps )
        , ( "processes", Encode.dict identity Encode.int query.processes )
        , ( "dies", Encode.dict identity Encode.int query.dies )
        , ( "test_types", Encode.dict identity Encode.int query.test_types )
        , ( "measurement_speeds", Encode.dict identity Encode.int query.test_types )
        , ( "dates", Encode.dict identity Encode.int query.dates )
        ]


decode : Decode.Decoder FilterOptions
decode =
    Decode.succeed FilterOptions
        |> required "sheet_names" (Decode.dict Decode.int)
        |> required "widths" (Decode.dict Decode.int)
        |> required "lengths" (Decode.dict Decode.int)
        |> required "temps" (Decode.dict Decode.int)
        |> required "processes" (Decode.dict Decode.int)
        |> required "dies" (Decode.dict Decode.int)
        |> required "test_types" (Decode.dict Decode.int)
        |> required "measurement_speeds" (Decode.dict Decode.int)
        |> required "dates" (Decode.dict Decode.int)


empty : FilterOptions
empty =
    FilterOptions Dict.empty Dict.empty Dict.empty Dict.empty Dict.empty Dict.empty Dict.empty Dict.empty Dict.empty
