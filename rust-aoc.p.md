Use Of Rust To Construct A Suite Of Small Algorithm Design
and Validation Executables and Supporting Infrastructure

Bart Massey

Rust for Advent Of Code 2016

Bart Massey

# Advent Of Code 2016

* Eric Wastl, http://adventofcode.com/2016 (also 2015)

* Series of 25 small programming challenge problems

* "Advent Calendar" format

* Roughly increasing difficulty

* Usual single-answer 

* Great sense of humor, statistics, competition

# My History: Advent of Code 2015 In Haskell

* http://github.com/BartMassey/advent-of-code-2015

* Used it as an opportunity to learn some more Haskell

* Got on the leaderboard a couple of times

* Well-liked, well-reviewed

# Advent of Code 2016 In Rust

* http://github.com/BartMassey/advent-of-code-2016

* Learn some more Rust

* Better performance + easier coding

* Informational / educational

# Programs + Infrastructure = Solution

* Bunch of related problems means common code

  * Maps and directions, A* search, RTL assembler

* Standard programming-puzzle means common code

  * Argument processing, primes and factoring, integer
    square root, popcount, combinatorics

* Rust limitations mean common code

  * Hexadecimal numbers, iterator over lines of a file

# Special Setup and Build

* Not written in Rust but in Bourne Shell

  * `mkday.sh`: Set up a new day's subdirectory
  * `process-aoc.sh`: Mangle copy-paste of problem
    into archival format (using `Pandoc`)
  
* Makes each day's setup fast because muh leaderboard

* Hopefully reusable for 2017


# Rust Issues: Binaries Are Second-Class Citizens

* Cargo support for binaries is limited

  * Binaries sharing a library does not work well

  * Cannot install arbitrary dependencies of binary
  
* Rustdoc does not binary much

  * No support for things like argument descriptions
  
  * `cargo rustdoc --open -- --no-defaults --passes collapse-docs \
      --passes unindent-comments --passes strip-priv-imports`

* Not great support for end-to-end binary tests
