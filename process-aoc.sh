#!/bin/sh
# Copyright © 2016 Bart Massey
# Cut up and process an Advent of Code day for problem
# documentation.
TMPDIR=/tmp/process-aoc.$$
trap "rm -rf $TMPDIR" 0 1 2 3 15
mkdir $TMPDIR
DAY=`cat .day`
tidy -q -asxml 2>/dev/null | awk '
  /^<article class="day-desc">$/ {
      part += 1;
      outfile = tmpdir "/part" part ".xhtml";
      printf "" >outfile;
      next;
  }
  /^<\/article>$/ {
      answer_par = 1;
      next;
  }
  answer_par == 1 && /^<p>/ {
      print $0 >>outfile;
      answer_par = 0;
      outfile = "";
      next;
  }
  {
      if (outfile != "")
          print $0 >> outfile;
      next;
  }' tmpdir=$TMPDIR &&
for f in $TMPDIR/part*.xhtml
do
    OUTFILE=`basename $f .xhtml`.md
    pandoc -f html -t markdown <$f |
    sed -e "1s/^--- Day/# Day/" \
        -e "1s/^--- /# Day $DAY: /" \
        -e '1s/ ---$//' \
        -e 2d >$OUTFILE
done
