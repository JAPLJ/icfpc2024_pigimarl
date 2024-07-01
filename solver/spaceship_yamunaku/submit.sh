#! /bin/bash

cd `dirname $0`

g++ main.cpp -O2 -std=c++17 -o main
echo -n "solve spaceship$1 " > solution
./main < $(printf "%02d" $1).txt >> solution
python3 ../main.py < solution 
