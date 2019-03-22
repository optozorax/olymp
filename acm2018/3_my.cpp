#include <iostream>
#include <vector>
#include <string>
#include <bitset>
#include <sstream>
#include <algorithm>

using namespace std;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}


int main() {
	//test("ababa");
	//test("aaa");
	//test("abcdefigazwvmb");

	string str;
	cin >> str;

	bitset<'z' - 'a' + 1> used;
	size_t smax = 0;
	for (int i = 0; i < str.size(); ++i) {
		int j = i;
		for (; j < str.size(); ++j) {
			if (!used.test(str[j]-'a')) {
				used.set(str[j]-'a', true);
			} else {
				smax = max(smax, used.count());
				used.reset();
				break;
			}
		}
		if (j == str.size()) {
			smax = max(smax, used.count());
		}
	}

	cout << smax;
}