#include <iostream>
#include <vector>
#include <algorithm>
#include <sstream>
#include <fstream>

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

int countWalls(vector<vector<int>>& p) {
	vector<pair<int, int>> m;
	for (int i = 0; i < p.size(); ++i) {
		if (p[i].size() > 0)
			for (int j = 0; j < p[i].size()-1; ++j) {
				m.push_back({p[i][j], p[i][j+1]});
			}
	}

	sort(m.begin(), m.end(), [] (auto a, auto b) -> bool {
		return a.first > b.first;
	});

	int currentWall = -1;
	int result = 0;
	for (auto& i : m) {
		if (currentWall >= i.first && currentWall < i.second) {

		} else {
			currentWall = i.first;
			result++;
		}
	}
	return result;
}

int main() {
	//string number = "27";
	//test(readFromFile("testacm2018/7/input" + number + ".txt"));

	int n, m;
	cin >> n >> m;
	char c;
	vector<vector<int>> rows(n), cols(m);
	for (int i = 0; i < n; ++i) {
		for (int j = 0; j < m; ++j) {
			cin >> c;
			if (c == '*') {
				rows[i].push_back(j);
				cols[j].push_back(i);
			}
		}
	}

	//cout << readFromFile("Testacm2018/7/output" + number + ".txt");
	cout << countWalls(rows) + countWalls(cols);
}