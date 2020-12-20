#include <iostream>
#include <cmath>
using namespace std;
typedef int64_t bint;

bool is_p(bint n) {
	if (n == 1) return false;
	if (n == 2) return true;
	if (n == 3) return true;

	for (int i = 2; i <= sqrt(n)+1; ++i) {
		if (n % i == 0) {
			return false;
		}
	}
	return true;
}

int main() {
	bint n;
	cin >> n;
	if (is_p(n))
		cout << "YES";
	else
		cout << "NO";

	//system("pause");
}