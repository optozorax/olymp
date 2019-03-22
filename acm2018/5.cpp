#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <map>
#include <list>
#include <forward_list>
#include <sstream>
#include <fstream>

using namespace std;

typedef int64_t bint;

bool is_palindrome(string &line, bint l, bint r) {
	for (bint i = l, j = r; i <= (l + r) / 2; ++i, --j) {
		if (line[i] != line[j])
			return false;
	}

	return true;
}

long min1 = 1000000;
bool no_recursion = false;

void delete_palindrome(string& str) {
	vector<vector<int>> map('z'-'a'+1);
	for (int i = 0; i < str.size(); i++)
		map[str[i]-'a'].push_back(i);

	for (auto& i : map) {
		for (int j = 0; j < i.size(); j++) {
			for (int k = i.size()-1; k >= j+1; k--) {
				if (is_palindrome(str, i[j], i[k])) {
					string str2 = str;
					str2.erase(str2.begin() + i[j], str2.begin() + i[k]);
					str2.erase(str2.begin() + i[j]);
					if (str2.size() < min1) min1 = str2.size();
					if (min1 == 0 || min1 == 1) {
						no_recursion = true;
						return;
					}
					if (!no_recursion)
						delete_palindrome(str2);
				}
			}
		}
	}
}

//-----------------------------------------------------------------------------
void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}


int main() {
	//test("hhaghaeahhhibihcffhecehcgahdabaahcdfghgfdbgadjdh");
	test("hhaghaeahhhibihcffhecehcgahdabaah");

	string str;
	cin >> str;
	delete_palindrome(str);
	cout << min1;
}