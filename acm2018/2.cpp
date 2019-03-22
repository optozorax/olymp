#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <map>
#include <sstream>

using namespace std;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

string readFromFile(string fileName) {
	std::ifstream fin(fileName);
	std::string result;
	result.assign(std::istreambuf_iterator<char>(fin), std::istreambuf_iterator<char>());
	fin.close();

	return result;
}

void merge(vector<string>& a, const vector<string>& b) {
	auto i = 0;
	for (int j = 0; j < b.size(); ++j) {
		auto finded = find(a.begin() + i, a.end(), b[j]);
		if (finded != a.end()) {
			i = distance(finded, a.begin())+1;
		} else {
			while (i != a.size() && a[i] < b[j]) 
				i++;
			a.insert(a.begin() + i, b[j]);
			i++;
		}
	}
}

int main() {
	test(readFromFile("input05.txt"));

	int n;
	cin >> n;
	vector<string> res;
	int nk;
	for (int i = 0; i < n; ++i) {
		cin >> nk;
		vector<string> rep(nk);
		string str;
		for (int j = 0; j < nk; ++j)
			cin >> rep[j];

		merge(res, rep);
	}
	
	cout << res.size() << endl;
	for (auto& i : res)
		cout << i << endl;
}