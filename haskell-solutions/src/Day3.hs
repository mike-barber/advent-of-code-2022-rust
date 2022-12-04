{-# LANGUAGE InstanceSigs #-}

module Day3
  ( run,
  )
where

import qualified Common
import qualified Data.Char
import qualified Data.HashSet as HashSet
import Data.Hashable (Hashable, hashWithSalt)
import Data.Maybe (catMaybes, listToMaybe)

newtype Item = Item Char deriving (Eq)

-- is is possible to derive this automatically too:
-- see https://hackage.haskell.org/package/hashable-1.4.0.2/docs/Data-Hashable.html#g:4
instance Hashable Item where
  hashWithSalt :: Int -> Item -> Int
  hashWithSalt salt (Item v) = hashWithSalt salt v

-- prettier show
instance Show Item where
  show :: Item -> String
  show (Item x) = show x

run :: IO ()
run = do
  inputs <- readInput
  putStrLn ("day3 / part1 = " ++ show (part1 inputs))
  putStrLn ("day3 / part1 = " ++ show (part2 inputs))

value :: Item -> Int
value (Item v)
  | v `elem` ['a' .. 'z'] = 1 + Data.Char.ord v - Data.Char.ord 'a'
  | v `elem` ['A' .. 'Z'] = 27 + Data.Char.ord v - Data.Char.ord 'A'
  | otherwise = 0

-- find common items in multiple sets
commonItemSets :: [HashSet.HashSet Item] -> Maybe Item
commonItemSets (s : xs) =
  let inter = foldr HashSet.intersection s xs
   in listToMaybe $ HashSet.toList inter
commonItemSets _ = Nothing

-- group list items into groups of 3
group3 :: [t] -> [Maybe [t]]
group3 [a, b, c] = [Just [a, b, c]]
group3 (a : b : c : xs) = Just [a, b, c] : group3 xs
group3 _ = [Nothing]

readInput :: IO [String]
readInput = Common.readLines "../day3/input1.txt"

parseSet :: String -> HashSet.HashSet Item
parseSet s = HashSet.fromList $ map Item s

splitLine :: String -> (String, String)
splitLine line =
  let len = length line
   in splitAt (len `div` 2) line

part1 :: [String] -> Int
part1 inputLines =
  let val line =
        let (a, b) = splitLine line
            sa = parseSet a
            sb = parseSet b
         in value <$> commonItemSets [sa,sb]
      lineValues = map val inputLines
   in sum $ catMaybes lineValues

part2 :: [String] -> Int
part2 inputLines =
  let groups = group3 inputLines
      common grp =
        let sets = map parseSet grp
         in value <$> commonItemSets sets
      values = map common (catMaybes groups)
   in sum $ catMaybes values
