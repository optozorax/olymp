#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <cassert>

using namespace std;
typedef int64_t bint; 
#define int int64_t

void test(string str) {
	auto s = new string(str);
	auto iss = new istringstream(*s);
	cin.rdbuf(iss->rdbuf());
}

pair<bint, bint> get_reverse(bint n) {
	if (n == 0) {
		return { n, 1 };
	}

	auto res = get_reverse(n / 10);
	return { n % 10 * res.second + res.first, res.second * 10 };
}

bint reverse(bint n) {
	return get_reverse(n).first;
}

bool is_even(bint n) {
	if (n == 0)
		return true;
	return (n % 10) % 2 == 1 && is_even(n / 10);
}

bool is(bint n) {
	return is_even(n + reverse(n));
}

int32_t main() {
	/*assert(reverse(100) == 1);
	assert(reverse(101) == 101);
	assert(reverse(987) == 789);
	assert(reverse(5) == 5);
	assert(reverse(54) == 45);

	assert(is_even(0) == true);
	assert(is_even(1) == true);
	assert(is_even(2) == false);
	assert(is_even(3) == true);
	assert(is_even(11) == true);
	assert(is_even(10) == false);
	assert(is_even(111) == true);
	assert(is_even(121) == false);
	assert(is_even(1110) == false);
	assert(is_even(11333231) == false);

	assert(is(12) == true);
	assert(is(807) == true);
	assert(is(99) == false);*/

	//test("12"); // 12
	//test("100000000");
	
	bint n;
	cin >> n;
	for (int i = n; i >= 0; i--) {
		if (i == 0) {
			cout << -1;
			break;
		}
		if (is(i)) {
			cout << i;
			break;
		}
	}
}