module Day2
  ( run,
  )
where

import qualified Common
import Data.List.Split (splitOn)
import Data.Maybe (mapMaybe)

data Play = Rock | Paper | Scissors deriving (Show, Eq)

data Outcome = Lose | Draw | Win deriving (Show, Eq)

run :: IO ()
run = do
  inputs <- readInput
  putStrLn ("day2 / part1 = " ++ show (part1 inputs))
  putStrLn ("day2 / part2 = " ++ show (part2 inputs))

readInput :: IO [String]
readInput = Common.readLines "../day2/input1.txt"

parseOpp :: Char -> Maybe Play
parseOpp x = case x of
  'A' -> Just Rock
  'B' -> Just Paper
  'C' -> Just Scissors
  _ -> Nothing

parseMe1 :: Char -> Maybe Play
parseMe1 x = case x of
  'X' -> Just Rock
  'Y' -> Just Paper
  'Z' -> Just Scissors
  _ -> Nothing

parseOutcome :: Char -> Maybe Outcome
parseOutcome x = case x of
  'X' -> Just Lose
  'Y' -> Just Draw
  'Z' -> Just Win
  _ -> Nothing

outcome :: Play -> Play -> Outcome
outcome opp me =
  if opp == me
    then Draw
    else case (opp, me) of
      (Rock, Paper) -> Win
      (Scissors, Rock) -> Win
      (Paper, Scissors) -> Win
      _ -> Lose

playGivenOutcome :: Play -> Outcome -> Play
playGivenOutcome opp outc =
  case (opp, outc) of
    (Rock, Win) -> Paper
    (Rock, Lose) -> Scissors
    (Paper, Win) -> Scissors
    (Paper, Lose) -> Rock
    (Scissors, Win) -> Rock
    (Scissors, Lose) -> Paper
    _ -> opp

scoreOutcome :: Outcome -> Int
scoreOutcome Lose = 0
scoreOutcome Draw = 3
scoreOutcome Win = 6

scoreMe :: Play -> Int
scoreMe Rock = 1
scoreMe Paper = 2
scoreMe Scissors = 3

score :: Play -> Outcome -> Int
score me res = scoreMe me + scoreOutcome res

maybeSplit :: String -> Maybe (Char, Char)
maybeSplit s =
  let spl [[a], [b]] = Just (a, b)
      spl _ = Nothing
   in spl $ splitOn " " s

calcPlay1 :: String -> Maybe (Play, Play)
calcPlay1 inp = do
  (a, b) <- maybeSplit inp
  opp <- parseOpp a
  me <- parseMe1 b
  Just (opp, me)

part1 :: [String] -> Int
part1 inputLines =
  let parsed = mapMaybe calcPlay1 inputLines
      scores = map (\(opp, me) -> score me (outcome opp me)) parsed
   in sum scores

calcPlay2 :: String -> Maybe (Play, Play)
calcPlay2 inp = do
  (a, b) <- maybeSplit inp
  opp <- parseOpp a
  outc <- parseOutcome b
  let play = playGivenOutcome opp outc
  Just (opp, play)

part2 :: [String] -> Int
part2 inputLines =
  let parsed = mapMaybe calcPlay2 inputLines
      scores = map (\(opp, me) -> score me (outcome opp me)) parsed
   in sum scores
