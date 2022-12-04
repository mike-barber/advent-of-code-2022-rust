module Day1
    ( run
    ) where

import qualified Common
import Data.List (sort)
import Data.List.Split (splitOn)
import Text.Read (readMaybe)
import Data.Maybe (catMaybes)

run :: IO ()
run = do
    part1
    part2

readInput :: IO [String]
readInput = Common.readLines "../day1/input1.txt"

parseGroups :: [String] -> [[Integer]]
parseGroups ll =
    let
        values = map readMaybe ll
        splits = splitOn [Nothing] values
    in map catMaybes splits

part1 :: IO ()
part1 = do
    inputLines <- readInput
    let groupSums = map sum $ parseGroups inputLines
    let solution = maximum groupSums
    putStrLn ("day1 / part1 = " ++ show solution)

part2 :: IO ()
part2 = do
    inputLines <- readInput
    let groupSums = map sum $ parseGroups inputLines
    let top3 = take 3 $ reverse $ sort groupSums
    let solution = sum top3
    putStrLn ("day1 / part2 = " ++ show solution)


