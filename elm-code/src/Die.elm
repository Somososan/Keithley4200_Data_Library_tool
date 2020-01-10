module Die exposing (toString)


toString : ( String, Int ) -> String
toString ( a, b ) =
    a ++ String.fromInt b
