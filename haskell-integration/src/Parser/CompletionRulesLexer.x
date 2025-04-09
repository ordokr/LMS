{
module Parser.CompletionRulesLexer (
  Token(..),
  scanTokens
) where
}

%wrapper "basic"

$digit = 0-9
$alpha = [a-zA-Z]
$ident = [$alpha $digit \_ \-]

tokens :-
  $white+                       ;
  "//".*                        ;
  complete                      { \_ -> TComplete }
  score                         { \_ -> TScore }
  above                         { \_ -> TAbove }
  and                           { \_ -> TAnd }
  or                            { \_ -> TOr }
  not                           { \_ -> TNot }
  all                           { \_ -> TAll }
  modules                       { \_ -> TModules }
  minimum                       { \_ -> TMinimum }
  posts                         { \_ -> TPosts }
  \(                            { \_ -> TLeftParen }
  \)                            { \_ -> TRightParen }
  \{                            { \_ -> TLeftBrace }
  \}                            { \_ -> TRightBrace }
  \,                            { \_ -> TComma }
  \:                            { \_ -> TColon }
  \%                            { \_ -> TPercent }
  $digit+\.$digit+              { \s -> TFloat (read s) }
  $digit+                       { \s -> TInt (read s) }
  \" [^\"]* \"                  { \s -> TString (init (tail s)) }
  $alpha$ident*                 { \s -> TIdentifier s }

{
data Token = 
    TComplete
  | TScore
  | TAbove
  | TAnd
  | TOr
  | TNot
  | TAll
  | TModules
  | TMinimum
  | TPosts
  | TLeftParen
  | TRightParen
  | TLeftBrace
  | TRightBrace
  | TComma
  | TColon
  | TPercent
  | TFloat Double
  | TInt Int
  | TString String
  | TIdentifier String
  deriving (Eq, Show)

scanTokens :: String -> [Token]
scanTokens = alexScanTokens
}