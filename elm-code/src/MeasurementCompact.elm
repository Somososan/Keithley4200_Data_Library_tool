module MeasurementCompact exposing (MeasurementCompact, decode, encode)

import Device exposing (Device)
import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode
import TerminalParameter exposing (TerminalParameter)
import TestDataCompact exposing (TestDataCompact)
import TestParameter exposing (TestParameter)
import TimeStamp exposing (TimeStamp)


type alias MeasurementCompact =
    { id : String
    , file_path : String
    , sheet_name : String

    --Device Under Test
    , device : Device

    --Test parameters
    , test_parameter : TestParameter
    , test_time_stamp : TimeStamp
    , terminals : List TerminalParameter

    --data
    , test_data : List TestDataCompact
    }


encode : MeasurementCompact -> Encode.Value
encode measurement =
    Encode.object
        [ ( "id", Encode.string measurement.id )
        , ( "file_path", Encode.string measurement.file_path )
        , ( "sheet_name", Encode.string measurement.sheet_name )
        , ( "device", Device.encode measurement.device )
        , ( "test_parameter", TestParameter.encode measurement.test_parameter )
        , ( "test_time_stamp", TimeStamp.encode measurement.test_time_stamp )
        , ( "teminals", Encode.list TerminalParameter.encode measurement.terminals )
        , ( "test_data", Encode.list TestDataCompact.encode measurement.test_data )
        ]


decode : Decode.Decoder MeasurementCompact
decode =
    Decode.succeed MeasurementCompact
        |> required "id" Decode.string
        |> required "file_path" Decode.string
        |> required "sheet_name" Decode.string
        |> required "device" Device.decode
        |> required "test_parameter" TestParameter.decode
        |> required "test_time_stamp" TimeStamp.decode
        |> required "terminals" (Decode.list TerminalParameter.decode)
        |> required "test_data" (Decode.list TestDataCompact.decode)
