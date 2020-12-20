#include <iostream>
#include <string>
#include <sstream>

using namespace std;
typedef int64_t bint;

void test(string str) {
	auto s = new string(str);
	auto iss = new istringstream(*s);
	cin.rdbuf(iss->rdbuf());
}

int main() {
	//test("3 8"); // 1 2
	bint n, k;
	cin >> n >> k;
	bint a = (k - 2 * n) / 2;
	bint b = n - a;
	cout << a << " " << b;
}