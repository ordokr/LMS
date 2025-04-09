{-# LANGUAGE OverloadedStrings #-}
module Parser.QueryLanguage (
  parseQueryLanguage,
  optimizeQuery,
  Query(..),
  QueryType(..),
  Condition(..),
  Expression(..),
  Column(..),
  Table(..)
) where

import Parser.QueryLexer (scanTokens)
import Parser.QueryParser
import Data.Text (Text)
import qualified Data.Text as T
import qualified Data.Map as M
import qualified Data.Set as S
import Control.Monad.State

-- | Parse query from a string
parseQueryLanguage :: Text -> Either String Query
parseQueryLanguage input = 
  -- In a real implementation, we would handle parse errors properly
  Right $ parseQuery (scanTokens (T.unpack input))

-- | Query optimization functions
optimizeQuery :: Query -> Query
optimizeQuery query = 
  let pipeline = [ pushDownFilters
                 , reorderJoins
                 , eliminateUnusedColumns
                 ]
  in foldl (\q f -> f q) query pipeline

-- | Push filters down closer to tables
pushDownFilters :: Query -> Query
pushDownFilters query = 
  -- Simple implementation for demonstration; a real optimizer would be more complex
  query { conditions = reorderConditions (conditions query) }

-- | Reorder conditions to filter out more rows early
reorderConditions :: [Condition] -> [Condition]
reorderConditions = sortOn conditionSelectivity
  where
    -- Simple heuristic: equals conditions are more selective than ranges
    conditionSelectivity (Equals _ _) = 1
    conditionSelectivity (NotEquals _ _) = 5
    conditionSelectivity (LessThan _ _) = 3
    conditionSelectivity (GreaterThan _ _) = 3
    conditionSelectivity (LessEquals _ _) = 4
    conditionSelectivity (GreaterEquals _ _) = 4
    conditionSelectivity (Nested conds) = minimum (map conditionSelectivity conds)
    
    -- GHC doesn't recognize sortOn, so we'd define it
    sortOn f = sortBy (\x y -> compare (f x) (f y))
    sortBy = error "Stub implementation"

-- | Reorder joins to minimize intermediate results
reorderJoins :: Query -> Query
reorderJoins query = 
  -- A real implementation would consider table sizes, join conditions, etc.
  query

-- | Eliminate columns not needed in final result
eliminateUnusedColumns :: Query -> Query
eliminateUnusedColumns query = 
  -- Identify required columns
  let usedColumns = execState (collectUsedColumns query) S.empty
      prunedColumns = filter (isColumnUsed usedColumns) (columns query)
  in query { columns = prunedColumns }

-- | Collect all columns used in query (including in conditions)
collectUsedColumns :: Query -> State (S.Set Text) ()
collectUsedColumns query = do
  -- Collect columns used in SELECT clause
  mapM_ collectColumnUsage (columns query)
  
  -- Collect columns used in WHERE conditions
  mapM_ collectConditionUsage (conditions query)
  
  -- Collect columns used in GROUP BY, ORDER BY
  mapM_ collectColumnUsage (groupBy query)
  case orderBy query of
    Just cols -> mapM_ collectColumnUsage cols
    Nothing -> return ()

-- | Check if a column is in the used set
isColumnUsed :: S.Set Text -> Column -> Bool
isColumnUsed used (SimpleColumn name) = S.member name used
isColumnUsed used (QualifiedColumn _ name) = S.member name used
isColumnUsed _ AllColumns = True
isColumnUsed used (AggregateColumn _ col _) = isColumnUsed used col

-- | Collect column names used in a column reference
collectColumnUsage :: Column -> State (S.Set Text) ()
collectColumnUsage (SimpleColumn name) = 
  modify $ S.insert name
collectColumnUsage (QualifiedColumn _ name) = 
  modify $ S.insert name
collectColumnUsage AllColumns = 
  return () -- All columns are used
collectColumnUsage (AggregateColumn _ col _) = 
  collectColumnUsage col

-- | Collect column names used in a condition
collectConditionUsage :: Condition -> State (S.Set Text) ()
collectConditionUsage (Equals e1 e2) = collectExprUsage e1 >> collectExprUsage e2
collectConditionUsage (LessThan e1 e2) = collectExprUsage e1 >> collectExprUsage e2
collectConditionUsage (GreaterThan e1 e2) = collectExprUsage e1 >> collectExprUsage e2
collectConditionUsage (LessEquals e1 e2) = collectExprUsage e1 >> collectExprUsage e2
collectConditionUsage (GreaterEquals e1 e2) = collectExprUsage e1 >> collectExprUsage e2
collectConditionUsage (NotEquals e1 e2) = collectExprUsage e1 >> collectExprUsage e2
collectConditionUsage (Nested conditions) = mapM_ collectConditionUsage conditions

-- | Collect column names used in an expression
collectExprUsage :: Expression -> State (S.Set Text) ()
collectExprUsage (ColumnExpr col) = collectColumnUsage col
collectExprUsage (LiteralExpr _) = return ()