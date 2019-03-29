#define _USE_MATH_DEFINES

#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <functional>
#include <sstream>
#include <string>
#include <bitset>
#include <numeric>
#include <algorithm>
#include <iomanip>
#include <cctype>
#include <string>
using namespace std;
typedef int64_t bint;
void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

vector<bint> letters(26, 0);
bint toNum(char c) {
	return tolower(c)-'a';
}

string name; 

bool check_impossible() {
	auto m = max_element(letters.begin(), letters.end());
	int sum = 0;
	for (auto i = letters.begin(); i != letters.end(); ++i) {
		if (i != m)
			sum += *i;
	}

	return sum < (*m-1);
}

bool make_name(bint pos, bint last) {
	if (check_impossible())
		return false;
	if (pos < name.size()) {
		for (int i = 0; i < letters.size(); ++i) {
			if (letters[i] != 0 && i != last) {
				name[pos] = 'a' + i;
				letters[i]--;
				if (make_name(pos+1, i))
					return true;
				letters[i]++;
			}
		}
		return false;
	}
	return true;
}

int main() {
	//test("6\ncbacba\n");
	//test("4\naaaa\n");
	//test("12\naaaaaaabbbcc\n");

	bint n;
	cin >> n;
	name.resize(n, ' ');
	for (int i = 0; i < n; ++i) {
		char c;
		cin >> c;
		letters[toNum(c)]++;
	}

	if (check_impossible())
		cout << "IMPOSSIBLE";
	else {
		if (make_name(0, -1)) 
			cout << name;
		else
			throw std::exception();
	}

	//system("pause");
}