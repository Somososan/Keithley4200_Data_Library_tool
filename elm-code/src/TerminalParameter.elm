module TerminalParameter exposing (TerminalParameter, decode, encode)

import CurrentRange exposing (CurrentRange)
import EncodeHelper
import Instrument exposing (Instrument)
import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode
import OpMode exposing (OpMode)
import Terminal exposing (Terminal)
import UnitMeasured exposing (UnitMeasured)
import VoltageRange exposing (VoltageRange)


type alias TerminalParameter =
    { terminal : Terminal
    , instrument : Instrument
    , operational_mode : OpMode
    , compliance : Maybe Float --current limit of the terminal
    , voltage : Maybe UnitMeasured
    , voltage_range : Maybe VoltageRange
    , current : Maybe UnitMeasured
    , current_range : Maybe CurrentRange
    }


encode : TerminalParameter -> Encode.Value
encode parameter =
    Encode.object
        [ ( "terminal", Terminal.encode parameter.terminal )
        , ( "instrument", Instrument.encode parameter.instrument )
        , ( "operational_mode", OpMode.encode parameter.operational_mode )
        , ( "compliance", EncodeHelper.maybe Encode.float parameter.compliance )
        , ( "voltage", EncodeHelper.maybe UnitMeasured.encode parameter.voltage )
        , ( "voltage_range", EncodeHelper.maybe VoltageRange.encode parameter.voltage_range )
        , ( "current", EncodeHelper.maybe UnitMeasured.encode parameter.current )
        , ( "current_range", EncodeHelper.maybe CurrentRange.encode parameter.current_range )
        ]


decode : Decode.Decoder TerminalParameter
decode =
    Decode.succeed TerminalParameter
        |> required "terminal" Terminal.decode
        |> required "instrument" Instrument.decode
        |> required "operational_mode" OpMode.decode
        |> required "compliance" (Decode.nullable Decode.float)
        |> required "voltage" (Decode.nullable UnitMeasured.decode)
        |> required "voltage_range" (Decode.nullable VoltageRange.decode)
        |> required "current" (Decode.nullable UnitMeasured.decode)
        |> required "current_range" (Decode.nullable CurrentRange.decode)
