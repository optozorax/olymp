#include <iostream>
#include <vector>
#include <map>
#include <string>
#include <algorithm>
#include <fstream>
#include <sstream>

using namespace std;

typedef int64_t bint;

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

/* Класс id для строк. Каждой уникальной строке выдается уникальный айди. Необходимо, чтобы использование строк заменить на использование чисел. */
class string_id
{
public:
	string_id() {
		count = 0;
	}

	int get_id(const string& str) {
		auto& pos = id[str];
		if (pos == 0) {
			count++;
			pos = count;
			id_reverse.push_back(str);
			return pos-1;
		} else
			return pos-1;
	}

	const string& get_str(int id) const {
		return id_reverse[id];
	}

	int size(void) const {
		return count;
	}
private:
	map<string, int> id;
	vector<string> id_reverse;
	int count;
};

int main() {
	//string testNo = "02";
	//test(readFromFile("testacm2018/2/input" + testNo + ".txt"));
	//cout << "Answer: " << readFromFile("testacm2018/2/output" + testNo + ".txt") << "END ANSWER" << endl << endl;

	// Считывание данных
	string_id id;
	int n;
	cin >> n;
	vector<vector<int>> reps(n, vector<int>());
	int nk;
	for (int i = 0; i < n; ++i) {
		cin >> nk;
		reps[i].resize(nk, 0);
		string str;
		for (int j = 0; j < nk; ++j) {
			cin >> str;
			reps[i][j] = id.get_id(str);
		}
	}

	vector<vector<bint>> pos(id.size());
	for (auto& rep : reps) {
		for (bint j = 1; j < rep.size(); ++j) {
			pos[rep[j]].push_back(rep[j-1]);
		}
	}

	vector<bint> ids;
	for (int i = 0; i < id.size(); ++i)
		ids.push_back(i);
	sort(ids.begin(), ids.end(), [&] (auto a, auto b) {
		return id.get_str(a) < id.get_str(b);
	});

	vector<bint> result;
	vector<bool> used(id.size(), false);
	while (result.size() != id.size()) {
		for (auto& i : ids) {
			if (!used[i]) {
				bool flag = true;
				for (bint j = 0; j < pos[i].size() && flag; ++j)
					flag &= used[pos[i][j]];
				if (flag) {
					result.push_back(i);
					used[i] = true;
					break;
				}
			}
		}
	}

	cout << result.size() << endl;
	for (bint i = 0; i < result.size(); ++i)
		cout << id.get_str(result[i]) << endl;

	return 0;
}