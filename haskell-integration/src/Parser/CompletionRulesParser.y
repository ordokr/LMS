{
module Parser.CompletionRulesParser (
  parseCompletionRule,
  Requirement(..)
) where

import Parser.CompletionRulesLexer
import Data.Text (Text)
import qualified Data.Text as T
}

%name parseCompletionRule
%tokentype { Token }
%error { parseError }

%token
  complete     { TComplete }
  score        { TScore }
  above        { TAbove }
  and          { TAnd }
  or           { TOr }
  not          { TNot }
  all          { TAll }
  modules      { TModules }
  minimum      { TMinimum }
  posts        { TPosts }
  '('          { TLeftParen }
  ')'          { TRightParen }
  '{'          { TLeftBrace }
  '}'          { TRightBrace }
  ','          { TComma }
  ':'          { TColon }
  '%'          { TPercent }
  float        { TFloat $$ }
  int          { TInt $$ }
  string       { TString $$ }
  identifier   { TIdentifier $$ }

%%

Requirement : CompleteRule                 { $1 }
            | ScoreRule                    { $1 }
            | AndRule                      { $1 }
            | OrRule                       { $1 }
            | NotRule                      { $1 }
            | AllModulesRule               { $1 }
            | MinimumPostsRule             { $1 }

CompleteRule : complete identifier         { CompleteAssignment (T.pack $2) }

ScoreRule : score above float '%' identifier { ScoreAbove $3 (T.pack $5) }

AndRule : and '{' Requirements '}'         { And $3 }

OrRule : or '{' Requirements '}'           { Or $3 }

NotRule : not Requirement                  { Not $2 }

AllModulesRule : complete all modules      { CompleteAllModules }

MinimumPostsRule : minimum int posts       { MinimumPostCount $2 }

Requirements : Requirement                 { [$1] }
             | Requirement ',' Requirements { $1 : $3 }

{
data Requirement = 
    CompleteAssignment Text
  | ScoreAbove Double Text
  | And [Requirement] 
  | Or [Requirement]
  | Not Requirement
  | CompleteAllModules
  | MinimumPostCount Int
  deriving (Show, Eq)

parseError :: [Token] -> a
parseError tokens = error $ "Parse error: " ++ show tokens
}