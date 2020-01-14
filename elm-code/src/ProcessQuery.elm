module ProcessQuery exposing (ProcessQuery, encode)

import Json.Encode as Encode
import ProcessData exposing (ProcessData)
import ProcessingType exposing (ProcessingType)


type alias ProcessQuery =
    { what : List ProcessingType
    , combined : Bool
    , from : List ProcessData
    }


encode : ProcessQuery -> Encode.Value
encode query =
    Encode.object
        [ ( "what", Encode.list ProcessingType.encode query.what )
        , ( "combined", Encode.bool query.combined )
        , ( "from", Encode.list ProcessData.encode query.from )
        ]
