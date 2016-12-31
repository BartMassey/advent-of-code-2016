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

case "$1" in
    [1-9]) DAY=day"0$1" ;;
    [1-3][0-9]) DAY=day"$1" ;;
    *) echo "mkday: bad day $1" >&2; exit 1 ;;
esac

mkdir $DAY
if [ $? -ne 0 ]
then
    echo "mkday: directory cannot be created, giving up" >&2
    exit 1
fi

cd template
for f in *
do
    sed "s=<day>=$1=g" <$f >../$DAY/$f
done

cd ../$DAY
echo $1 >.day
