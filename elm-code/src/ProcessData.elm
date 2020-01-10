module ProcessData exposing (ProcessData, encode)

import Json.Encode as Encode
import TestDataCompact exposing (TestDataCompact, encode)


type alias ProcessData =
    { id : String
    , data : List TestDataCompact
    }


encode : ProcessData -> Encode.Value
encode pdata =
    Encode.object
        [ ( "id", Encode.string pdata.id )
        , ( "data", Encode.list TestDataCompact.encode pdata.data )
        ]
