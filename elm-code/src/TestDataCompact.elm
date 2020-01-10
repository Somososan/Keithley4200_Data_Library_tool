module TestDataCompact exposing (TestDataCompact, decode, encode, toString, toString_concise)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (optional, resolve)
import Json.Encode as Encode


type TestDataCompact
    = DrainVoltage Int
    | GateVoltage Int
    | BulkVoltage Int
    | SourceVoltage Int
    | DrainCurrent Int
    | GateCurrent Int
    | BulkCurrent Int
    | SourceCurrent Int
    | Time Int


encode : TestDataCompact -> Encode.Value
encode test_data =
    case test_data of
        DrainVoltage inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]

        GateVoltage inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]

        BulkVoltage inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]

        SourceVoltage inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]

        DrainCurrent inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]

        GateCurrent inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]

        BulkCurrent inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]

        SourceCurrent inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]

        Time inner ->
            Encode.object [ ( "test_data_compact", Encode.int inner ) ]


decode : Decode.Decoder TestDataCompact
decode =
    let
        decode_further : Maybe Int -> Maybe Int -> Maybe Int -> Maybe Int -> Maybe Int -> Maybe Int -> Maybe Int -> Maybe Int -> Maybe Int -> Decode.Decoder TestDataCompact
        decode_further drainvoltage gatevoltage bulkvoltage sourcevoltage draincurrent gatecurrent bulkcurrent sourcecurrent time =
            case time of
                Just a ->
                    Decode.succeed (Time a)

                _ ->
                    case sourcecurrent of
                        Just a ->
                            Decode.succeed (SourceCurrent a)

                        _ ->
                            case bulkcurrent of
                                Just a ->
                                    Decode.succeed (BulkCurrent a)

                                _ ->
                                    case gatecurrent of
                                        Just a ->
                                            Decode.succeed (GateCurrent a)

                                        _ ->
                                            case draincurrent of
                                                Just a ->
                                                    Decode.succeed (DrainCurrent a)

                                                _ ->
                                                    case sourcevoltage of
                                                        Just a ->
                                                            Decode.succeed (SourceVoltage a)

                                                        _ ->
                                                            case bulkvoltage of
                                                                Just a ->
                                                                    Decode.succeed (BulkVoltage a)

                                                                _ ->
                                                                    case gatevoltage of
                                                                        Just a ->
                                                                            Decode.succeed (GateVoltage a)

                                                                        _ ->
                                                                            case drainvoltage of
                                                                                Just a ->
                                                                                    Decode.succeed (DrainVoltage a)

                                                                                _ ->
                                                                                    Decode.fail "Error Parsing TestData"
    in
    Decode.succeed decode_further
        |> optional "DrainVoltage" (Decode.nullable Decode.int) Nothing
        |> optional "GateVoltage" (Decode.nullable Decode.int) Nothing
        |> optional "BulkVoltage" (Decode.nullable Decode.int) Nothing
        |> optional "SourceVoltage" (Decode.nullable Decode.int) Nothing
        |> optional "DrainCurrent" (Decode.nullable Decode.int) Nothing
        |> optional "GateCurrent" (Decode.nullable Decode.int) Nothing
        |> optional "BulkCurrent" (Decode.nullable Decode.int) Nothing
        |> optional "SourceCurrent" (Decode.nullable Decode.int) Nothing
        |> optional "Time" (Decode.nullable Decode.int) Nothing
        |> resolve


toString : TestDataCompact -> String
toString test_data =
    case test_data of
        DrainVoltage _ ->
            "Drain Voltage"

        GateVoltage _ ->
            "Gate Voltage"

        BulkVoltage _ ->
            "Bulk Voltage"

        SourceVoltage _ ->
            "Source Voltage"

        DrainCurrent _ ->
            "Drain Current"

        GateCurrent _ ->
            "Gate Current"

        BulkCurrent _ ->
            "Bulk Current"

        SourceCurrent _ ->
            "Source Current"

        Time _ ->
            "Time"


toString_concise : TestDataCompact -> String
toString_concise test_data =
    case test_data of
        DrainVoltage _ ->
            "Vd"

        GateVoltage _ ->
            "Vg"

        BulkVoltage _ ->
            "Vb"

        SourceVoltage _ ->
            "Vs"

        DrainCurrent _ ->
            "Id"

        GateCurrent _ ->
            "Ig"

        BulkCurrent _ ->
            "Ib"

        SourceCurrent _ ->
            "Is"

        Time _ ->
            "T"
