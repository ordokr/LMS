{
module Parser.QueryLexer (
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
  "--".*                        ;
  select                        { \_ -> TSelect }
  from                          { \_ -> TFrom }
  where                         { \_ -> TWhere }
  join                          { \_ -> TJoin }
  on                            { \_ -> TOn }
  and                           { \_ -> TAnd }
  or                            { \_ -> TOr }
  not                           { \_ -> TNot }
  group                         { \_ -> TGroup }
  by                            { \_ -> TBy }
  order                         { \_ -> TOrder }
  limit                         { \_ -> TLimit }
  offset                        { \_ -> TOffset }
  count                         { \_ -> TCount }
  sum                           { \_ -> TSum }
  avg                           { \_ -> TAvg }
  min                           { \_ -> TMin }
  max                           { \_ -> TMax }
  as                            { \_ -> TAs }
  \=                            { \_ -> TEquals }
  \<                            { \_ -> TLessThan }
  \>                            { \_ -> TGreaterThan }
  \<\=                          { \_ -> TLessEquals }
  \>\=                          { \_ -> TGreaterEquals }
  \!\=                          { \_ -> TNotEquals }
  \(                            { \_ -> TLeftParen }
  \)                            { \_ -> TRightParen }
  \,                            { \_ -> TComma }
  \.                            { \_ -> TDot }
  \*                            { \_ -> TStar }
  $digit+\.$digit+              { \s -> TFloat (read s) }
  $digit+                       { \s -> TInt (read s) }
  \" [^\"]* \"                  { \s -> TString (init (tail s)) }
  \' [^\']* \'                  { \s -> TString (init (tail s)) }
  $alpha$ident*                 { \s -> TIdentifier s }

{
data Token = 
    TSelect
  | TFrom
  | TWhere
  | TJoin
  | TOn
  | TAnd
  | TOr
  | TNot
  | TGroup
  | TBy
  | TOrder
  | TLimit
  | TOffset
  | TCount
  | TSum
  | TAvg
  | TMin
  | TMax
  | TAs
  | TEquals
  | TLessThan
  | TGreaterThan
  | TLessEquals
  | TGreaterEquals
  | TNotEquals
  | TLeftParen
  | TRightParen
  | TComma
  | TDot
  | TStar
  | TFloat Double
  | TInt Int
  | TString String
  | TIdentifier String
  deriving (Eq, Show)

scanTokens :: String -> [Token]
scanTokens = alexScanTokens
}