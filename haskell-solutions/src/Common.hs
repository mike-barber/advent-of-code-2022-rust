module Common
  ( readLines,
  )
where

-- add common functions here
readLines :: String -> IO [String]
readLines fileName = do
  contents <- readFile fileName
  return (lines contents)
