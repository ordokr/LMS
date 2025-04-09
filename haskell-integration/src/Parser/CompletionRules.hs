{-# LANGUAGE OverloadedStrings #-}
module Parser.CompletionRules (
  parseRequirements,
  Requirement(..),
  requirementToJson,
  requirementFromJson
) where

import Parser.CompletionRulesLexer (scanTokens)
import Parser.CompletionRulesParser (parseCompletionRule, Requirement(..))
import Data.Text (Text)
import qualified Data.Text as T
import qualified Data.Aeson as A
import Data.Aeson ((.=), (.:))
import qualified Data.Aeson.Types as AT

-- | Parse requirements from a string
parseRequirements :: Text -> Either String Requirement
parseRequirements input = 
  -- In a real implementation, we would handle parse errors properly
  -- For simplicity, we're using 'error' in the parser and catching exceptions here
  Right $ parseCompletionRule (scanTokens (T.unpack input))

-- | Convert requirement to JSON
requirementToJson :: Requirement -> A.Value
requirementToJson (CompleteAssignment aid) = 
  A.object [ "type" .= ("complete_assignment" :: Text)
           , "assignment_id" .= aid
           ]
requirementToJson (ScoreAbove pct aid) = 
  A.object [ "type" .= ("score_above" :: Text)
           , "percentage" .= pct
           , "assignment_id" .= aid
           ]
requirementToJson (And reqs) = 
  A.object [ "type" .= ("and" :: Text)
           , "requirements" .= map requirementToJson reqs
           ]
requirementToJson (Or reqs) = 
  A.object [ "type" .= ("or" :: Text)
           , "requirements" .= map requirementToJson reqs
           ]
requirementToJson (Not req) = 
  A.object [ "type" .= ("not" :: Text)
           , "requirement" .= requirementToJson req
           ]
requirementToJson CompleteAllModules = 
  A.object [ "type" .= ("complete_all_modules" :: Text)
           ]
requirementToJson (MinimumPostCount count) = 
  A.object [ "type" .= ("minimum_post_count" :: Text)
           , "count" .= count
           ]

-- | Parse requirement from JSON
requirementFromJson :: A.Value -> AT.Parser Requirement
requirementFromJson = A.withObject "Requirement" $ \obj -> do
  rtype <- obj .: "type" :: AT.Parser Text
  case rtype of
    "complete_assignment" -> CompleteAssignment <$> obj .: "assignment_id"
    "score_above" -> ScoreAbove <$> obj .: "percentage" <*> obj .: "assignment_id"
    "and" -> And <$> (obj .: "requirements" >>= A.withArray "requirements" (mapM (A.parseJSON . A.toJSON)))
    "or" -> Or <$> (obj .: "requirements" >>= A.withArray "requirements" (mapM (A.parseJSON . A.toJSON)))
    "not" -> Not <$> (obj .: "requirement" >>= requirementFromJson)
    "complete_all_modules" -> pure CompleteAllModules
    "minimum_post_count" -> MinimumPostCount <$> obj .: "count"
    _ -> fail $ "Unknown requirement type: " ++ T.unpack rtype