#!/bin/sh
DAY=`basename \`pwd\``
sed "s/DAY/$DAY/" ../update/commit.msg >commit.msg
case "$1" in
    "-c") echo "* added libcapturesat for compat" >>commit.msg ;;
esac
git commit -a -F commit.msg
rm commit.msg
