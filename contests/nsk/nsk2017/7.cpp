#include <iostream>
#include <sstream>
#include <iterator>
#include <fstream>
#include <map>
using namespace std;
typedef int64_t bint;
void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

int main() {
	//test("3\na 10\nb 10\na 100\n4\na 10\nb 100\na 1000\na 900\n");
	//test("3\na 10\nb 10\na 100\n4\na 10\nb 100\na 10\na 9\n");

	map<char, map<int, int>> objects;
	map<char, map<int, int>> boxes;

	bint n, m;
	cin >> n;
	for (int i = 0; i < n; ++i) {
		char c;
		bint count;
		cin >> c >> count;
		objects[c][count]++;
	}

	cin >> m;
	for (int i = 0; i < m; ++i) {
		char c;
		bint count;
		cin >> c >> count;
		boxes[c][count]++;
	}

	auto check = [&] () -> bool {
		for (auto& i : objects) {
			auto& object = i.second;
			auto& box = boxes[i.first];

			auto oi = object.rbegin();
			auto bi = box.rbegin();
			for (; oi != object.rend(); ++oi) {
				start:
				if (oi->first <= bi->first) {
					if (oi->second <= bi->second) {
						bi->second -= oi->second;
						if (bi->second == 0)
							++bi;
					} else {
						oi->second -= bi->second;
						bi->second = 0;
						++bi;
						goto start;
					}
				} else {
					return false;
				}
			}
		}

		return true;
	};

	if (check()) {
		cout << "YES";
	} else {
		cout << "NO";
	}

	//system("pause");
}