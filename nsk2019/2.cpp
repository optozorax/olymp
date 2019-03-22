#include <iostream>
#include <cmath>

using namespace std;

void writeAnswer(int64_t x) {
	int64_t n = (sqrt(1 + 8 * x) - 1) / 2;
	if (n*(n + 1) / 2 == x)
		cout << "YES";
	else
		cout << "NO";
}

int main() {
	int64_t x;
	cin >> x;
	writeAnswer(x);
	/*writeAnswer(2);
	writeAnswer(3);
	writeAnswer(4);
	writeAnswer(5);
	writeAnswer(6);
	writeAnswer(7);
	writeAnswer(8);
	writeAnswer(9);
	writeAnswer(10);*/
}