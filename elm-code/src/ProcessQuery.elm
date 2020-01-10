module ProcessQuery exposing (ProcessQuery, encode)

import Json.Encode as Encode
import ProcessData exposing (ProcessData)
import ProcessingType exposing (ProcessingType)


type alias ProcessQuery =
    { what : ProcessingType
    , combined : Bool
    , from : List ProcessData
    }


encode : ProcessQuery -> Encode.Value
encode query =
    Encode.object
        [ ( "what", ProcessingType.encode query.what )
        , ( "combined", Encode.bool query.combined )
        , ( "from", Encode.list ProcessData.encode query.from )
        ]
