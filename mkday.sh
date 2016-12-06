#!/bin/sh
# Copyright © 2016 Bart Massey

# Set up a template directory for a new day.

if [ $# -ne 1 ]
then
    echo "mkday: usage: mkday <day>" >&2
    exit 1
fi

if ! [ -f mkday.sh ]
then
    echo "mkday: not run from base directory" >&2
    exit 1
fi

DAY=day"$1"

mkdir $DAY
if [ $? -ne 0 ]
then
    echo "mkday: directory cannot be created, giving up" >&2
    exit 1
fi
cd $DAY
cp ../template/*.rs ../template/*.md ../template/Cargo.toml .
for i in 1 2
do
    ln -s target/debug/part${i} part${i}
done

