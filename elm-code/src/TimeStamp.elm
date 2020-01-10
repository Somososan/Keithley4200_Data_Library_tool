module TimeStamp exposing (TimeStamp, decode, encode, toString)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Json.Encode as Encode


type alias TimeStamp =
    { year : Int
    , month : Int
    , day : Int
    , hour : Int
    , minute : Int
    , second : Int
    }


encode : TimeStamp -> Encode.Value
encode timestamp =
    Encode.object
        [ ( "test_time_stamp"
          , Encode.object
                [ ( "year", Encode.int timestamp.year )
                , ( "month", Encode.int timestamp.month )
                , ( "day", Encode.int timestamp.day )
                , ( "hour", Encode.int timestamp.hour )
                , ( "minute", Encode.int timestamp.minute )
                , ( "second", Encode.int timestamp.second )
                ]
          )
        ]


decode : Decode.Decoder TimeStamp
decode =
    Decode.succeed TimeStamp
        |> required "year" Decode.int
        |> required "month" Decode.int
        |> required "day" Decode.int
        |> required "hour" Decode.int
        |> required "minute" Decode.int
        |> required "second" Decode.int


toString : TimeStamp -> String
toString timestamp =
    String.padLeft 4 '0' (String.fromInt timestamp.year) ++ "/" ++ String.padLeft 2 '0' (String.fromInt timestamp.month) ++ "/" ++ String.padLeft 2 '0' (String.fromInt timestamp.day) ++ " " ++ String.padLeft 2 '0' (String.fromInt timestamp.hour) ++ ":" ++ String.padLeft 2 '0' (String.fromInt timestamp.minute) ++ ":" ++ String.padLeft 2 '0' (String.fromInt timestamp.second)
