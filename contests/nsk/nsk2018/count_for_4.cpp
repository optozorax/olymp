#include <iostream>
#include <vector>
#include <numeric>
#include <algorithm>

using namespace std;

typedef int64_t bint;

int main() {
	bint a, b;

	//cin >> a >> b;
	//a = 31275; b = 13575;
	a = 9971; b = 9863;

	bint size = 2*a*b;
	vector<char> used(size, false);
	bint x = 0, y = 0;
	while (true) {
		x = 0;
		while (true) {
			bint sum = a * x + b * y;
			if (sum >= size)
				break;
			else 
				used[sum] = true;
			x++;
		}
		y++;
		if (b * y >= size)
			break;
	}

	bint g = gcd(a, b), l = lcm(a, b);
	cout << "gcd(a, b) = " << g << ", lcm(a, b) = " << l << ", a*b = " << a * b << endl; 

	for (int i = 0; i < size; ++i) if (used[i]) {
		bool is_start = true;
		for (int j = 0; j < size; ++j) {
			if (i + j * g >= size)
				break; 
			if (!used[i + j * g]) {
				is_start = false;
				break;
			}
		}
		if (is_start) {
			cout << "min start: " << i << endl;
			break;
		}
	}

	system("pause");
}

/*

1524 531
gcd(a, b) = 3, lcm(a, b) = 269748, a*b = 809244
min start: 267696

9971 9863
gcd(a, b) = 1, lcm(a, b) = 98343973, a*b = 98343973
min start: 98324140




*/