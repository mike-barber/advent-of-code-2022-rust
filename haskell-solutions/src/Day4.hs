module Day4
  ( run,
  )
where

import qualified Common
import Data.List.Split (splitOn)
import Data.Maybe (mapMaybe)
import Text.Read (readMaybe)

-- quite an inefficient implementation for a range, but
-- it's fairly straightforward
newtype Range = Range [Int] deriving (Show)

data AssignmentPair = AssignmentPair Range Range deriving (Show)

run :: IO ()
run = do
  inputs <- readInput
  putStrLn ("day4 / part1 = " ++ show (part1 inputs))
  putStrLn ("day4 / part1 = " ++ show (part2 inputs))

readInput :: IO [String]
readInput = Common.readLines "../day4/input1.txt"

split :: String -> String -> Maybe (String, String)
split delimiter s =
  let splMatch [a, b] = Just (a, b)
      splMatch _ = Nothing
   in splMatch $ splitOn delimiter s

parseRange :: String -> Maybe Range
parseRange s = do
  (sa, sb) <- split "-" s
  a <- readMaybe sa
  b <- readMaybe sb
  Just $ Range [a .. b]

parseAssignment :: String -> Maybe AssignmentPair
parseAssignment s = do
  (sa, sb) <- split "," s
  a <- parseRange sa
  b <- parseRange sb
  Just $ AssignmentPair a b

assignmentHasSubset :: AssignmentPair -> Bool
assignmentHasSubset (AssignmentPair ra rb) =
  let subsetOf (Range aa) (Range bb) = all (`elem` bb) aa
   in subsetOf ra rb || subsetOf rb ra

part1 :: [String] -> Int
part1 input =
  let assignments = mapMaybe parseAssignment input
   in length $ filter assignmentHasSubset assignments

assignmentHasOverlap :: AssignmentPair -> Bool
assignmentHasOverlap (AssignmentPair ra rb) =
  let anyIn (Range aa) (Range bb) = any (`elem` bb) aa
   in anyIn ra rb || anyIn rb ra

part2 :: [String] -> Int
part2 input =
  let assignments = mapMaybe parseAssignment input
   in length $ filter assignmentHasOverlap assignments