#include <iostream>
#include <vector>
#include <iomanip>
#include <map>
#include <fstream>
#include <algorithm>
#include <cmath>

using namespace std;

typedef int64_t bint;

//-----------------------------------------------------------------------------
double p(int a, int b) {
	if (a < b) swap(a, b);
	double up = 1;
	double down = 1;
	for (int i = 0; i < b; ++i) {
		up *= a + 1 + i;
		down *= 1 + i;
	}

	return up/down;
}

//-----------------------------------------------------------------------------
double getCount(int n, int k, int one) {
	if (one == 0)
		return 0;

	double sum = 0;
	for (int i = 0; i <= one; ++i) if((2*i + 1 <= one) && (k - 2*i-1 >= 0) && (n - one - k + 2*i+1 >= 0)) {
		int j = 2*i + 1;
		sum += p(j, k-j) * p(one - j, n - one - k + j);
	}

	return sum;
}

//-----------------------------------------------------------------------------
int getCountOfOne(const std::vector<bint>& mas, int bit) {
	int result = 0;
	for (const auto& i : mas)
		result += (i & (1 << bit)) >> bit;
	return result;
}

//-----------------------------------------------------------------------------
double calcExpected(int n, int k, const std::vector<bint>& mas) {
	double sum = 0;
	bint twoPow = 1;
	for (int i = 0; i < 32; i++, twoPow *= 2) {
		int ones = getCountOfOne(mas, i);
		sum += twoPow * getCount(n, k, ones) / double(p(ones, n - ones));
	}

	return sum;
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

int main() {
	int k, n;

	cin >> n >> k;
	std::vector<bint> mas(n, 0);
	for (int i = 0; i < n; i++)
		cin >> mas[i];

	cout << std::fixed << std::setprecision(3) << calcExpected(n, k, mas) << endl;
}