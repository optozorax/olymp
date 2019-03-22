#include <iostream>
#include <list>
#include <sstream>
#include <string>

using namespace std;

typedef int64_t bint;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

int main() {
	//test("50\n1");
	//test("702232740505518990545957699696\n20");

	bint n, k;
	string num;
	cin >> num;
	cin >> k;

	bint posi = 0;
	auto i = num.begin();
	list<char> result;
	while (posi < num.size() - k) {
		if (*i == '9')
			result.push_back(*i);
		else {
			int posmax = posi;
			auto imax = i;
			auto j = i;
			auto posj = posi;
			for (; posj <= posi + k; ++posj, ++j) {
				if (*j == '9') {
					posmax = posj;
					imax = j;
					break;
				} else {
					if (*j > *imax) {
						posmax = posj;
						imax = j;
					} else {

					}
				}
			}
			result.push_back(*imax);
			if (posmax != posi) {
				k -= posmax - posi;
				posi = posmax;
				i = imax;
			}
		}
		posi++;
		i++;
	}

	for (auto& i : result)
		cout << i;
}