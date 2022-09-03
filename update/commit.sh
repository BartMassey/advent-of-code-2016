#!/bin/sh
DAY=`basename \`pwd\``
sed "s/DAY/$DAY/" ../update/commit.msg >commit.msg
git commit -a -F commit.msg
rm commit.msg
