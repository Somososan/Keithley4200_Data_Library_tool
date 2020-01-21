module ProcessingType exposing (ProcessingType, encode, id_bins, id_bins_normalized, id_for_swept_VDS_and_VGS, id_normalized_versus_time, id_versus_time, psd, raw, toString, toString_concise)

import Json.Encode as Encode


type ProcessingType
    = Raw
    | Id_versus_time
    | Id_normalized_versus_time
    | Id_bins
    | Id_bins_normalized
    | Id_for_swept_VDS_and_VGS
    | Psd


raw : ProcessingType
raw =
    Raw


id_normalized_versus_time : ProcessingType
id_normalized_versus_time =
    Id_normalized_versus_time


id_versus_time : ProcessingType
id_versus_time =
    Id_versus_time


id_bins : ProcessingType
id_bins =
    Id_bins


id_bins_normalized : ProcessingType
id_bins_normalized =
    Id_bins_normalized


id_for_swept_VDS_and_VGS : ProcessingType
id_for_swept_VDS_and_VGS =
    Id_for_swept_VDS_and_VGS


psd : ProcessingType
psd =
    Psd


encode : ProcessingType -> Encode.Value
encode kind =
    case kind of
        Raw ->
            Encode.object [ ( "process_type", Encode.string "Raw" ) ]

        Id_versus_time ->
            Encode.object [ ( "process_type", Encode.string "Id_versus_time" ) ]

        Id_normalized_versus_time ->
            Encode.object [ ( "process_type", Encode.string "Id_normalized_versus_time" ) ]

        Id_bins ->
            Encode.object [ ( "process_type", Encode.string "Id_bins" ) ]

        Id_bins_normalized ->
            Encode.object [ ( "process_type", Encode.string "Id_bins_normalized" ) ]

        Id_for_swept_VDS_and_VGS ->
            Encode.object [ ( "process_type", Encode.string "Id_for_swept_VDS_and_VGS" ) ]

        Psd ->
            Encode.object [ ( "process_type", Encode.string "Psd" ) ]


toString : ProcessingType -> String
toString kind =
    case kind of
        Raw ->
            "Raw"

        Id_versus_time ->
            "Drain current versus Time"

        Id_normalized_versus_time ->
            "Drain current versus Time with the current divided by its average"

        Id_bins ->
            "Histogram of the Drain current values"

        Id_bins_normalized ->
            "Histogram of the Drain current values with the current divided by its average"

        Id_for_swept_VDS_and_VGS ->
            "Drain current over the gate voltage for various Drain - Source voltages"

        Psd ->
            "Power Spectral density of the Drain current"


toString_concise : ProcessingType -> String
toString_concise kind =
    case kind of
        Raw ->
            "Raw"

        Id_versus_time ->
            "Id(t)"

        Id_normalized_versus_time ->
            "Id(t)/Id,avg"

        Id_bins ->
            "f(Id)"

        Id_bins_normalized ->
            "f(Id/Id,avg)"

        Id_for_swept_VDS_and_VGS ->
            "Id(Vgs,Vds)"

        Psd ->
            "Psd"
