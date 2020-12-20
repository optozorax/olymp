/** Данная задача не решена!!! */

#include <iostream>
#include <algorithm>
#include <string>
#include <algorithm>
#include <sstream>
#include <vector>
#include <numeric>

using namespace std;

typedef int64_t bint;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

bint get_unique_count(bint max, bint mass) {
	if (mass == 1)
		return max;
	else
		return max/mass;
}

int main() {
	//test("2 10\n3 5");
	//test("3 100\n6 15 14");
	//test("5 100\n1 158 65 19 15");
	test("5 1000\n39 80 25 78 94"); // ответ: 819

	bint k, n;
	cin >> k >> n;
	vector<bint> g(k, 0);
	for (bint i = 0; i < k; ++i)
		cin >> g[i];

	bint sum = 0;
	for (bint i = 0; i < k; ++i) {
		sum += get_unique_count(n, g[i]);
		for (bint j = i+1; j < k; ++j) {
			sum -= get_unique_count(n, g[i]*g[j]/gcd(g[i], g[j]));
		}
	}

	cout << sum;
}