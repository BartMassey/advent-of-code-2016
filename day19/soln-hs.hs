-- Copyright © 2016 Bart Massey

-- This program is licensed under the "MIT License".
-- Please see the file COPYING in this distribution
-- for license terms.

-- Advent of Code Day 19 in Haskell.

import System.Environment
import Data.Sequence as S hiding (length)

del :: Int -> S.Seq Int -> S.Seq Int
del i s
    | i < 0 = error "index must be positive"
    | i >= length s = error "index off end"
    | otherwise = deleteAt i s

exchange :: Int -> (Int -> Int -> Int) -> S.Seq Int -> Int
exchange posn nextElf elves
    | length elves == 1 = index elves 0
    | otherwise =
        let victor_serial = index elves posn
            victim = nextElf (length elves) posn
            victim_serial = index elves victim
            elves' = del victim elves
            successor = (posn + 1) `mod` length elves in
        exchange successor nextElf elves'

main:: IO ()
main = do
  [partStr, countStr] <- getArgs
  let part = read partStr :: Int
  let n = read countStr :: Int
  let elves = S.fromList [1..n]
  let survivor =
          case part of
            1 -> exchange 0 (\n i -> (i + 1) `mod` n) elves
            2 -> exchange 0 (\n i -> (i + (n `div` 2)) `mod` n) elves
            _ -> error "unknown part"
  print $ survivor
