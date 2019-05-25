#include <iostream>
#include <vector>
#include <map>
#include <cmath>
#include <functional>
#include <numeric>
#include <algorithm>
#include <bitset>
#include <cassert>

using namespace std;
typedef int64_t bint;

void mul(bint count, bint m, bint& n) {
	bitset<64> c(count);
	bint n1 = n;
	bint m1 = m;
	for (int i = 0; i < 64; i++) {
		if (c[i])
			n = (n*m) % (1000000007);
		m = (m*m) % (1000000007);
	}

	/*for (int i = 0; i < count; i++)
		n1 = (n1*m1) % (1000000007);

	assert(n1 == n);*/
}

bint getAnswer(bint n) {
	if (n == 2) return 1;
	if (n == 3) return 2;
	if (n == 4) return 3;
	if (n == 5) return 4;
	n--;
	
	bint n4 = 0;
	bint n3 = n / 3;
	bint n2 = n - n3 * 3;
	if (n2 == 1) {
		if (n3 > 0) {
			n3--;
			n4++;
			n2 = 0;
		}
	}
	n2 /= 2;

	bint result = 1;
	mul(n4, 4, result);
	mul(n3, 3, result);
	mul(n2, 2, result);

	return result;
}

int main() {
	bint n;
	cin >> n;
	//n = 64;
	/*n = 2;
	cout << getAnswer(5) << endl;
	cout << getAnswer(6) << endl;
	cout << getAnswer(7) << endl;
	cout << getAnswer(8) << endl;
	cout << getAnswer(9) << endl;*/
	cout << getAnswer(n);

	//system("pause");
}