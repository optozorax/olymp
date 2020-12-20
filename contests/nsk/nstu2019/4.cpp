#include <iostream>
#include <string>
#include <vector>
#include <map>
using namespace std;

typedef int64_t bint;

string tobin(bint a) {
	if (a == 0) return "0";
	if (a == 1) return "1";

	string result;
	while (a != 0) {
		result = std::to_string(bint(a % 2)) + result;
		a /= 2;
	}

	return result;
}

bool equal(string a, string b) {
	return a == b;
}

bool myless(string a, string b) { // a <= b
	if (b.size() > a.size())
		return true;
	if (b.size() < a.size())
		return false;
	if (b.size() == a.size()) {
		for (int i = 0; i < a.size(); i++) {
			if (b[i] > a[i])
				return true;
			if (a[i] > b[i])
				return false;
		}

		return true;
	}
}

string makePalindrom(string a, bool isOddSize) {
	string result = a;
	for (int i = 0; i < a.size() - isOddSize; i++)
		result += a[a.size() - 1 - i - isOddSize];

	return result;
}

string findNearestPalindrom(string a) {
	bool odd = a.size() % 2 == 1;
	string b;
	for (int i = 0; i < a.size() / 2 + odd; i++) b += '0';
	b[0] = '1';
	for (int i = 1; i < b.size(); i++) {
		b[i] = '1';
		if (!myless(makePalindrom(b, odd), a)) b[i] = '0';
	}

	return makePalindrom(b, odd);
}

bint palindromsPartCount(bint size) {
	if (size < 0) return 0;
	if (size == 0) return 0;
	if (size == 1) return 2;
	if (size == 2) return 2;

	static map<bint, bint> cache;
	auto& c = cache[size];
	if (c != 0) return c;
	else {
		c = 2 * palindromsPartCount(size - 2);
		return c;
	}
}

bint palindromsCount(bint size) {
	if (size < 0) return 0;
	if (size == 0) return 0;
	if (size == 1) return 1;
	if (size == 2) return 2;

	static map<bint, bint> cache;
	auto& c = cache[size];
	if (c != 0) return c;
	else {
		bint sum = 0;
		for (int i = 1; i <= size; i++)
			sum += palindromsPartCount(i) / 2.0;

		c = sum;
		return sum;
	}
}

bool isPalindrom(string a) {
	if (a == "0") return false;
	if (a == "1") return true;

	bool odd = a.size() % 2 == 1;
	return makePalindrom(a.substr(0, a.size() / 2 + odd), odd) == a;
}

bint findPalindromsCountWithEqualSize(string a) {
	auto p = findNearestPalindrom(a);
	if (!myless(p, a)) return 0;

	bool odd = a.size() % 2 == 1;
	int size = a.size() / 2 + odd;

	bint sum = 1;
	for (int i = 1; i < size; i++) {
		if (p[i] == '1') {
			sum += palindromsPartCount((size - 1 - i) * 2 - odd);
			if (i == size - 1)
				sum++;
		}
	}

	return sum;
}

bint getFastAnswer(bint n) {
	if (n == 1) return 1;
	int count1 = findPalindromsCountWithEqualSize(tobin(n));
	int count2 = palindromsCount(tobin(n).size() - 1);
	return count1 + count2;
}

bint getPrecisionAnswer(bint n) {
	bint sum = 0;
	for (int i = 1; i <= n; i++)
		sum += isPalindrom(tobin(i));

	return sum;
}

int main() {
	bint n;
	cin >> n;
	cout << getFastAnswer(n);

	/*cout << getFastAnswer(1000000000000000000);

	for (int i = 0; i < 10000; i++) {
		if (getFastAnswer(i) != getPrecisionAnswer(i)) {
			cout << i << " = " << tobin(i) << ", fast: " << getFastAnswer(i) << ", precision: " << getPrecisionAnswer(i) << endl;
			break;
		} else {
			cout << i << " OK" << endl;
		}
	}*/

	//system("pause");
}