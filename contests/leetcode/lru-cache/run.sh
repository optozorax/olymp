clear
g++ $1.cpp -O3 -s -DNDEBUG && (echo "Compiled"; ./a.out)