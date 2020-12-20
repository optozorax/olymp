#include <iostream>
#include <vector>
#include <map>
#include <cmath>
#include <functional>
#include <numeric>
#include <algorithm>

using namespace std;
typedef int64_t bint;

int main() {
	bint a, b;
	cin >> a >> b;
	//a = 10; b = 10;
	bint i = 1;
	while (a + b > 0) {
		int count = i;
		if (b * 2 <= count) {
			count -= b * 2;
			b = 0;
		} else {
			b -= count / 2;
			count -= (count / 2)*2;
		}
		if (a <= count) {
			count -= a;
			a = 0;
		} else {
			a -= count;
			count -= count;
		}
		i++;
	}
	cout << i-1;

	//system("pause");
}