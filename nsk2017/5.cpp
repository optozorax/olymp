#include <iostream>
#include <sstream>
#include <algorithm>
#include <bitset>
using namespace std;
typedef int64_t bint;
void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

int main() {
	//test("5\nvasya\n");
	bint n;
	cin >> n;
	bitset<256> alphabet(0);
	char c;
	bint count = 1;
	for (int i = 0; i < n; ++i) {
		cin >> c;
		if (alphabet[c]) {
			count++;
			alphabet.reset();
		}
		alphabet[c] = true;
	}
	cout << count
		;
	system("pause");
}