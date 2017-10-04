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

# Let's Do Day 1!

* Part 1 is really straightforward

* Note that there's a lot of fairly ugly boilerplate

  * Reading a line from a file

  * String manipulation and number parsing

# Day 1 Part 2

* Need to detect self-intersection of path

* Rust HashSet is perfect for this

* Note that memory management really isn't an issue here

# The Rest Of The Days

* Kinds of problems: straight puzzles, simulations,
  brute-force calculations

* Difficulty from a few minutes to "I don't think I can do
  this"

* The theming is nice and keeps it interesting

* Nice endgame (on Christmas Day!)

# Programs + Infrastructure = Solution

* Bunch of related problems means common code

  * Maps and directions, A* search, RTL assembler

* Standard programming-puzzle means common code

  * Argument processing, primes and factoring, integer
    square root, popcount, combinatorics

* Rust limitations mean common code

  * Hexadecimal numbers, iterator over lines of a file

# Assembunny

* Several of the problems require parsing and executing
  assembly code for a hypothetical machine
  
* Extracting the assembler is a big clarity win

* Also allows performance improvements: 318 MAIPS

# Special Setup and Build

* Not written in Rust but in Bourne Shell

  * `mkday.sh`: Set up a new day's subdirectory
  * `process-aoc.sh`: Mangle copy-paste of problem
    into archival format (using `Pandoc`)
  
* Makes each day's setup fast because muh leaderboard

* Hopefully reusable for 2017


# The Organization Is Partly A Lie

* I've presented the organization of this stuff as though I
  chose it in advance
  
* In truth, it all grew fairly organically

* In particular, a lot of stuff was split out and cleaned up
  after the contest was over, in prep for publication

# Rust Issue: Binaries Are Second-Class Citizens

* Cargo support for binaries is limited

  * Binaries sharing a library does not work well

  * Cannot install arbitrary dependencies of binary
  
* Rustdoc does not binary much

  * No support for things like argument descriptions
  
  * `cargo rustdoc --open -- --no-defaults --passes collapse-docs \
      --passes unindent-comments --passes strip-priv-imports`

* Not great support for end-to-end binary tests

# Final Thoughts

* Rust is ready for this kind of task

* A reasonable person with reasonable Rust knowledge can
  build quickly and cleanly

* It would be good to have Rust developers sit with newer
  users and watch them work through an exercise like this
