#include <iostream>
#include <vector>
#include <sstream>
#include <algorithm>
#include <fstream>
#include <map>

using namespace std;
typedef int64_t bint;

void test(string str) {
	// Функция для передачи строки на потоковый ввод, чтобы легко тестировать программу
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

bint n;
vector<pair<bint, vector<bint>>> build; // nomer etaja i ludi kotorie na nem projivaut
void readData(void) {
	cin >> n;
	vector<pair<bint, bint>> man;
	man.resize(n);
	for (bint i = 0; i < n; ++i) {
		bint k;
		cin >> k;
		man[i] = {i+1, k};
	}
	sort(man.begin(), man.end(), [](auto a, auto b) -> bool {
		return a.second < b.second;
	});
	for (auto i : man) {
		if (build.size() != 0) {
			if (build.back().first == i.second) {
				build.back().second.push_back(i.first);
			} else {
				build.push_back({i.second, {i.first}});
			}
		} else {
			build.push_back({ i.second, {i.first} });
		}
	}
}

bint calctime(bint i) {
	bint sum = 0;
	for (int j = 0; j < build.size(); ++j) {
		bint res = (build[i].first - build[j].first) * build[j].second.size();
		if (res < 0)
			sum += -res;
		else
			sum += res;
	}
	return sum;
}

bool isGrow(bint i) {
	bint time = calctime(i);
	if (i == build.size() - 1) {
		bint time1 = calctime(i - 1);
		return time1 < time;
	} else {
		bint time1 = calctime(i + 1);
		return time1 > time;
	}
}

bool isMin(bint i) {
	if (i == 0) {
		return isGrow(i);
	} else if(i == build.size()) {
		return !isGrow(i);
	} else {
		return !isGrow(i-1) && isGrow(i);
	}
}

bint findMinTime(void) {
	bint a = 0, b = build.size()-1;
	while (true) {
		if (isMin(a)) return a;
		if (isMin(b)) return b;
		bint c = (a + b) / 2;
		if (c == a || c ==b)
			return c;
		if (!isGrow(c))
			a = c;
		else
			b = c;
	}
}

int main() {
	test(readFromFile("testacm2017/10/input11.txt"));

	readData();

	// ofstream fout("out.txt");
	// for (int i = 0; i < build.size(); i++)
	// 	fout << calctime(i) << endl;
	// fout.close();
	
	bint res = findMinTime();
	while (res > 0) {
		if (calctime(res - 1) < calctime(res)) {
			res--;
		}
		else break;
	}

	auto rescopy = res;

	vector<int> humans;
	humans.insert(humans.end(), build[res].second.begin(), build[res].second.end());
	while (res > 0) {
		if (calctime(res - 1) == calctime(res)) {
			res--;
			humans.insert(humans.end(), build[res].second.begin(), build[res].second.end());
		}
		else break;
	}

	res = rescopy;
	while (res < build.size()-1) {
		if (calctime(res + 1) == calctime(res)) {
			res++;
			humans.insert(humans.end(), build[res].second.begin(), build[res].second.end());
		}
		else break;
	}

	sort(humans.begin(), humans.end(), [](auto a, auto b) -> bool {
		return a < b;
	});

	cout << humans[0];

	//system("pause");
}