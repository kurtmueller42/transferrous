#!/bin/bash
pushd test_directory

rm ./*.out

./transferrous-server >/dev/null 2>&1 &
PID=$!

sleep 0.2

files=`ls *.in`

RED='\u001b[31;1m'
GREEN='\u001b[32;1m'
WHITE='\u001b[37;1m'
NC='\033[0m' # No Color
BLK_BCK='\u001b[40m'

for f in $files
do
  printf "${WHITE}${BLK_BACK}Running test on $f\n"
  ./transferrous-client $f >/dev/null 2>&1
  MD5IN=`cat $f | md5sum`
  MD5OUT=`cat $f.out | md5sum`
  if [ "$MD5IN" == "$MD5OUT" ]; then
    printf "${GREEN}PASSED\n${WHITE}"
  else
    printf "${RED}FAILED\n${WHITE}"
  fi
done

kill $PID

popd
