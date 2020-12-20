#include <iostream>
#include <vector>
#include <map>
#include <cmath>

using namespace std;
typedef int64_t bint;

int main() {
	bint n;
	//n = 5;
	//n = 50;
	//n = 8;
	cin >> n;
	bint m = log(double(n)) / log(2.0);
	n -= pow(2, m);
	bint result = 0;
	vector<bint> nextmany(m+1);
	for (bint i = 0; i < m; i++)
		nextmany[i] = pow(2, i);
	for (int i = 0; i < m; i++) {
		bint count = pow(2, m-i-1);
		bint many = nextmany[i];
		while (n > 0 && many != 0 && n >= count) {
			n -= count;
			many--;
			result++;
			for (bint j = i+1; j < m; j++)
				nextmany[j] += pow(2, j-1);
		}
		if (n == 0)
			break;
	}

	cout << result;

	//system("pause");
}