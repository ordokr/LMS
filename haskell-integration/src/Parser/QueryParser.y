{
module Parser.QueryParser (
  parseQuery,
  Query(..),
  QueryType(..),
  Condition(..),
  Expression(..),
  Column(..),
  Table(..)
) where

import Parser.QueryLexer
import qualified Data.Text as T
}

%name parseQuery
%tokentype { Token }
%error { parseError }

%token
  select       { TSelect }
  from         { TFrom }
  where        { TWhere }
  join         { TJoin }
  on           { TOn }
  and          { TAnd }
  or           { TOr }
  not          { TNot }
  group        { TGroup }
  by           { TBy }
  order        { TOrder }
  limit        { TLimit }
  offset       { TOffset }
  count        { TCount }
  sum          { TSum }
  avg          { TAvg }
  min          { TMin }
  max          { TMax }
  as           { TAs }
  '='          { TEquals }
  '<'          { TLessThan }
  '>'          { TGreaterThan }
  '<='         { TLessEquals }
  '>='         { TGreaterEquals }
  '!='         { TNotEquals }
  '('          { TLeftParen }
  ')'          { TRightParen }
  ','          { TComma }
  '.'          { TDot }
  '*'          { TStar }
  float        { TFloat $$ }
  int          { TInt $$ }
  string       { TString $$ }
  identifier   { TIdentifier $$ }

%%

Query : SelectQuery                        { $1 }

SelectQuery : select SelectColumns from Tables WhereClause 
                                          { Query Select $2 $4 $5 [] Nothing Nothing Nothing }
            | select SelectColumns from Tables WhereClause GroupByClause
                                          { Query Select $2 $4 $5 $6 Nothing Nothing Nothing }
            | select SelectColumns from Tables WhereClause OrderByClause
                                          { Query Select $2 $4 $5 [] $6 Nothing Nothing }
            | select SelectColumns from Tables WhereClause LimitClause
                                          { Query Select $2 $4 $5 [] Nothing $6 Nothing }
            | select SelectColumns from Tables WhereClause OffsetClause
                                          { Query Select $2 $4 $5 [] Nothing Nothing $6 }

SelectColumns : '*'                        { [AllColumns] }
              | ColumnList                 { $1 }

ColumnList : Column                        { [$1] }
           | Column ',' ColumnList         { $1 : $3 }

Column : identifier                        { SimpleColumn (T.pack $1) }
       | identifier '.' identifier         { QualifiedColumn (T.pack $1) (T.pack $3) }
       | AggregateFunction '(' Column ')' as identifier
                                          { AggregateColumn $1 $3 (T.pack $6) }

AggregateFunction : count                  { Count }
                  | sum                    { Sum }
                  | avg                    { Avg }
                  | min                    { Min }
                  | max                    { Max }

Tables : Table                             { [$1] }
       | Table ',' Tables                  { $1 : $3 }
       | Table JoinClause                  { [$1, $2] }

Table : identifier                         { Table (T.pack $1) Nothing }
      | identifier as identifier           { Table (T.pack $1) (Just (T.pack $3)) }

JoinClause : join Table on Condition       { $2 }

WhereClause : {- empty -}                  { [] }
            | where Conditions             { $2 }

Conditions : Condition                     { [$1] }
           | Condition and Conditions      { $1 : $3 }

Condition : Expression '=' Expression      { Equals $1 $3 }
          | Expression '<' Expression      { LessThan $1 $3 }
          | Expression '>' Expression      { GreaterThan $1 $3 }
          | Expression '<=' Expression     { LessEquals $1 $3 }
          | Expression '>=' Expression     { GreaterEquals $1 $3 }
          | Expression '!=' Expression     { NotEquals $1 $3 }
          | '(' Conditions ')'             { Nested $2 }

Expression : Column                        { ColumnExpr $1 }
           | Literal                       { LiteralExpr $1 }

Literal : int                              { IntLit $1 }
        | float                            { FloatLit $1 }
        | string                           { StringLit (T.pack $1) }

GroupByClause : group by ColumnList        { $3 }

OrderByClause : order by ColumnList        { $3 }

LimitClause : limit int                    { $2 }

OffsetClause : offset int                  { $2 }

{
data Query = Query {
    queryType :: QueryType,
    columns :: [Column],
    tables :: [Table],
    conditions :: [Condition],
    groupBy :: [Column],
    orderBy :: Maybe [Column],
    limit :: Maybe Int,
    offset :: Maybe Int
} deriving (Show, Eq)

data QueryType = Select | Join | Aggregate
  deriving (Show, Eq)

data Condition = 
    Equals Expression Expression
  | LessThan Expression Expression
  | GreaterThan Expression Expression
  | LessEquals Expression Expression
  | GreaterEquals Expression Expression
  | NotEquals Expression Expression
  | Nested [Condition]
  deriving (Show, Eq)

data Expression = 
    ColumnExpr Column
  | LiteralExpr Literal
  deriving (Show, Eq)

data Literal =
    IntLit Int
  | FloatLit Double
  | StringLit T.Text
  deriving (Show, Eq)

data Column = 
    SimpleColumn T.Text
  | QualifiedColumn T.Text T.Text
  | AggregateColumn AggregateFunction Column T.Text
  | AllColumns
  deriving (Show, Eq)

data AggregateFunction = Count | Sum | Avg | Min | Max
  deriving (Show, Eq)

data Table = Table {
    tableName :: T.Text,
    tableAlias :: Maybe T.Text
} deriving (Show, Eq)

parseError :: [Token] -> a
parseError tokens = error $ "Parse error: " ++ show tokens
}