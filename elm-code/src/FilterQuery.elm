module FilterQuery exposing (FilterQuery, empty, encode)

import Date exposing (Date)
import EncodeHelper
import Json.Encode as Encode
import Maybe exposing (Maybe)


type alias FilterQuery =
    { sheet_names : List String
    , widths : List String
    , lengths : List String
    , temps : List String
    , wafer : String
    , dies : List String
    , test_type : String
    , measurement_speeds : List String
    , dates_between : ( Maybe Date, Maybe Date )
    }


encode : FilterQuery -> Encode.Value
encode query =
    Encode.object
        [ ( "sheet_names", Encode.list Encode.string query.sheet_names )
        , ( "widths", Encode.list Encode.string query.widths )
        , ( "lengths", Encode.list Encode.string query.lengths )
        , ( "temps", Encode.list Encode.string query.temps )
        , ( "wafer", Encode.string query.wafer )
        , ( "dies", Encode.list Encode.string query.dies )
        , ( "test_type", Encode.string query.test_type )
        , ( "measurement_speeds", Encode.list Encode.string query.measurement_speeds )
        , ( "dates_between", EncodeHelper.tuple2 (EncodeHelper.maybe Date.encode) (EncodeHelper.maybe Date.encode) query.dates_between )
        ]


empty : FilterQuery
empty =
    FilterQuery [] [] [] [] "" [] "" [] ( Nothing, Nothing )
