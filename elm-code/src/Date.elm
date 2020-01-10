module Date exposing (Date, encode, fromString, toString)

import Json.Encode as Encode


type alias Date =
    { year : Int
    , month : Int
    , day : Int
    }


encode : Date -> Encode.Value
encode date =
    Encode.object
        [ ( "test_time_stamp"
          , Encode.object
                [ ( "year", Encode.int date.year )
                , ( "month", Encode.int date.month )
                , ( "day", Encode.int date.day )
                ]
          )
        ]


toString : Date -> String
toString date =
    String.fromInt date.day ++ String.fromInt (date.month * 100) ++ String.fromInt (date.year * 10000)


fromString : String -> Date
fromString string =
    let
        y =
            string |> String.slice 0 3 |> String.toInt |> Maybe.withDefault 0

        m =
            string |> String.slice 4 5 |> String.toInt |> Maybe.withDefault 0

        d =
            string |> String.slice 6 7 |> String.toInt |> Maybe.withDefault 0
    in
    Date y m d
