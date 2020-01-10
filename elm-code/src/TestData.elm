module TestData exposing (TestData, decode, encode)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (optional, resolve)
import Json.Encode as Encode


type TestData
    = DrainVoltage (List (List Float))
    | GateVoltage (List (List Float))
    | BulkVoltage (List (List Float))
    | SourceVoltage (List (List Float))
    | DrainCurrent (List (List Float))
    | GateCurrent (List (List Float))
    | BulkCurrent (List (List Float))
    | SourceCurrent (List (List Float))
    | Time (List Float)


encode : TestData -> Encode.Value
encode test_data =
    case test_data of
        DrainVoltage inner ->
            Encode.object [ ( "DrainVoltage", Encode.list (Encode.list Encode.float) inner ) ]

        GateVoltage inner ->
            Encode.object [ ( "GateVoltage", Encode.list (Encode.list Encode.float) inner ) ]

        BulkVoltage inner ->
            Encode.object [ ( "BulkVoltage", Encode.list (Encode.list Encode.float) inner ) ]

        SourceVoltage inner ->
            Encode.object [ ( "SourceVoltage", Encode.list (Encode.list Encode.float) inner ) ]

        DrainCurrent inner ->
            Encode.object [ ( "DrainCurrent", Encode.list (Encode.list Encode.float) inner ) ]

        GateCurrent inner ->
            Encode.object [ ( "GateCurrent", Encode.list (Encode.list Encode.float) inner ) ]

        BulkCurrent inner ->
            Encode.object [ ( "BulkCurrent", Encode.list (Encode.list Encode.float) inner ) ]

        SourceCurrent inner ->
            Encode.object [ ( "SourceCurrent", Encode.list (Encode.list Encode.float) inner ) ]

        Time inner ->
            Encode.object [ ( "Time", Encode.list Encode.float inner ) ]


decode : Decode.Decoder TestData
decode =
    let
        decode_further : Maybe (List (List Float)) -> Maybe (List (List Float)) -> Maybe (List (List Float)) -> Maybe (List (List Float)) -> Maybe (List (List Float)) -> Maybe (List (List Float)) -> Maybe (List (List Float)) -> Maybe (List (List Float)) -> Maybe (List Float) -> Decode.Decoder TestData
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
        |> optional "DrainVoltage" (Decode.nullable (Decode.list (Decode.list Decode.float))) Nothing
        |> optional "GateVoltage" (Decode.nullable (Decode.list (Decode.list Decode.float))) Nothing
        |> optional "BulkVoltage" (Decode.nullable (Decode.list (Decode.list Decode.float))) Nothing
        |> optional "SourceVoltage" (Decode.nullable (Decode.list (Decode.list Decode.float))) Nothing
        |> optional "DrainCurrent" (Decode.nullable (Decode.list (Decode.list Decode.float))) Nothing
        |> optional "GateCurrent" (Decode.nullable (Decode.list (Decode.list Decode.float))) Nothing
        |> optional "BulkCurrent" (Decode.nullable (Decode.list (Decode.list Decode.float))) Nothing
        |> optional "SourceCurrent" (Decode.nullable (Decode.list (Decode.list Decode.float))) Nothing
        |> optional "Time" (Decode.nullable (Decode.list Decode.float)) Nothing
        |> resolve
