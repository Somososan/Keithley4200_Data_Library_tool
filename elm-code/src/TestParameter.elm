module TestParameter exposing (TestParameter, decode, encode)

import EncodeHelper
import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode
import MeasurementSpeed exposing (MeasurementSpeed)
import TestType exposing (TestType)


type alias TestParameter =
    { test_type : TestType
    , measurement_speed : MeasurementSpeed
    , ad_aperture : Maybe Float
    , filter_factor : Maybe Float
    , interval_time : Maybe Float
    , sweep_delay_time : Maybe Float
    , hold_time : Float
    }


encode : TestParameter -> Encode.Value
encode param =
    Encode.object
        [ ( "test_type", TestType.encode param.test_type )
        , ( "measurement_speed", MeasurementSpeed.encode param.measurement_speed )
        , ( "ad_aperture", EncodeHelper.maybe Encode.float param.ad_aperture )
        , ( "filter_factor", EncodeHelper.maybe Encode.float param.filter_factor )
        , ( "interval_time", EncodeHelper.maybe Encode.float param.interval_time )
        , ( "sweep_delay_time", EncodeHelper.maybe Encode.float param.sweep_delay_time )
        , ( "hold_time", Encode.float param.hold_time )
        ]


decode : Decode.Decoder TestParameter
decode =
    Decode.succeed TestParameter
        |> required "test_type" TestType.decode
        |> required "measurement_speed" MeasurementSpeed.decode
        |> required "ad_aperture" (Decode.nullable Decode.float)
        |> required "filter_factor" (Decode.nullable Decode.float)
        |> required "interval_time" (Decode.nullable Decode.float)
        |> required "sweep_delay_time" (Decode.nullable Decode.float)
        |> required "hold_time" Decode.float
