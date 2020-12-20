// Решение на основе идеи Ступакова о том, что функция, дающая i-й палиндром монотонна, и что можно найти решение по помощи двоичного поиска

#include <iostream>
#include <bitset>
#include <iomanip>
#include <string>
#include <cmath>

using namespace std;

typedef int64_t bint;

bint getIPalindrome(bint i) {
	if (i == 0) return 1;
	if (i == 1) return 3;
	i -= 2;

	bint size = floor(log2(i/4 + 1)) + 1;
	bint old_size = floor(4 * pow(2.0, size-2));
	bint sub = 4.0 * (pow(2.0, size-1)-1);
	bint inside = (i - sub) % old_size;
	bool odd = ((i - sub) / old_size) == 0;

	bitset<64> to(inside); 
	bint end = size*2-odd + 1;

	bitset<64> r;
	r[0] = 1;
	r[end] = 1;

	for (int i = 0; i < size; i++) {
		r[i+1] = to[size-i-1];
		r[end-1-i] = to[size-i-1];
	}

	return r.to_ullong();
}

int findI(bint n) {
	if (n == 0) return -1;
	if (n == 1) return 0;

	bint a = 0;
	bint b = 4*(pow(2.0, 31)-1)+1;

	while (true) {
		bint c = (a + b)/2;
		if (getIPalindrome(c) > n) {
			b = c;
		} else {
			a = c;
		}

		if (getIPalindrome(c) == n) return c;
		if (a == b) return a;
		if (a == b-1) return a;
	}

	return 0;
}

bint getFastAnswer(bint n) {
	return findI(n)+1;
}

int main() {
	bint n;
	cin >> n;
	cout << getFastAnswer(n);
}