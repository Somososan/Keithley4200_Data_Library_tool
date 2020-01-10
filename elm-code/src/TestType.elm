module TestType exposing (TestType, decode, encode, sampling, sweeping, toProcessingtypes, toString)

import Json.Decode as Decode
import Json.Encode as Encode
import ProcessingType exposing (ProcessingType, id_bins, id_bins_normalized, id_for_swept_VDS_and_VGS, id_normalized_versus_time, id_versus_time)


type TestType
    = Sampling
    | Sweeping


sampling : TestType
sampling =
    Sampling


sweeping : TestType
sweeping =
    Sweeping


encode : TestType -> Encode.Value
encode test_type =
    case test_type of
        Sampling ->
            Encode.string "Sampling"

        Sweeping ->
            Encode.string "Sweeping"


decode : Decode.Decoder TestType
decode =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "Sampling" ->
                        Decode.succeed Sampling

                    "Sweeping" ->
                        Decode.succeed Sweeping

                    _ ->
                        Decode.fail "Error parsing TestType"
            )


toString : TestType -> String
toString testtype =
    case testtype of
        Sampling ->
            "Sampling"

        Sweeping ->
            "Sweeping"


toProcessingtypes : TestType -> List ProcessingType
toProcessingtypes testtype =
    case testtype of
        Sampling ->
            [ id_bins, id_bins_normalized, id_versus_time, id_normalized_versus_time ]

        Sweeping ->
            [ id_for_swept_VDS_and_VGS ]
