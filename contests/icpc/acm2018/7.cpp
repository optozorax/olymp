#include <iostream>
#include <vector>
#include <sstream>
#include <algorithm>
#include <fstream>
#include <map>

using namespace std;

int n, m;
vector<vector<int>> rows; // строки
vector<vector<int>> cols; // столбцы
vector<vector<int>> alls; // Массив пар, там пишется какая стена защитит эту пару
int result = 0;

vector<vector<int>> wall; // В этом массиве показывается для какой стены каким парам она принадлежит

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

void readData(void) {
	cin >> n >> m;
	rows.resize(n);
	cols.resize(m);
	char c;
	for (int i = 0; i < n; ++i) {
		for (int j = 0; j < m; ++j) {
			cin >> c;
			if (c == '*') {
				rows[i].push_back(j);
				cols[j].push_back(i);
			}
		}
	}
}

bool is_min(const vector<int>& walls) {
	int count = 0;
	for (const auto& i : walls)
		count += wall[i].size();
	if (count >= alls.size()) {
		vector<char> covered(alls.size(), 0);
		for (const auto& i : walls) {
			for (int j = 0; j < wall[i].size(); ++j) {
				covered[wall[i][j]] = 1;
			}
		}
		for (int i = 0; i < covered.size(); ++i) {
			if (!covered[i])
				return false;
		}
		return true;
	} else 
		return false;
}

bool forall(const vector<int>& a, vector<int>& mas, int i, int last, int n) {
	if (i >= n - 1) {
		for (int j = last; j < a.size(); ++j) {
			mas[i] = a[j];
			if (is_min(mas))
				return true;
		}
	} else {
		for (int j = last; j < a.size(); ++j) {
			mas[i] = a[j];
			if (forall(a, mas, i+1, j+1, n))
				return true;
		}
	}
	return false;
}

vector<int> covered;
int covered_count = 0;
int min_depth = 3000;

void travel(int start, int depth) {
	if (depth > min_depth)
		return;
	for (int i = start; i < alls.size(); i++) if (covered[i] == 0) {
		for (int j = 0; j < alls[i].size(); j++) {
			int wall_no = alls[i][j];
			for (int k = 0; k < wall[wall_no].size(); ++k) {
				auto& ref = covered[wall[wall_no][k]];
				ref++;
				if (ref == 1)
					covered_count++;
			}
			if (covered_count == alls.size()) {
				if (depth < min_depth) {
					min_depth = depth;
					cout << depth << endl;
				}
			} else
				travel(i + 1, depth + 1);
			for (int k = 0; k < wall[wall_no].size(); ++k) {
				auto& ref = covered[wall[wall_no][k]];
				if (ref == 1)
					covered_count--;
				ref--;
			}
		}
	}
}

int main() {
	test(readFromFile("Testacm2018/7/input18.txt"));
	readData();
	cout << readFromFile("Testacm2018/7/output18.txt");

	auto remake_wall = [] () {
		wall.clear();
		wall.resize(n+m-2);
		for (int i = 0; i < alls.size(); i++) {
			for (int j = 0; j < alls[i].size(); j++) {
				wall[alls[i][j]].push_back(i);
			}
		}
	};

	// Собираем все возможные стены, которые защитят от удара бомб по горизонтали
	for (int i = 0; i < rows.size(); ++i) {
		if (rows[i].size() > 0)
		for (int j = 0; j < rows[i].size()-1; ++j) {
			alls.push_back({});
			int a = rows[i][j];
			int b = rows[i][j+1];
			for (int k = a; k < b; ++k) {
				alls.back().push_back(m-1 + k);
			}
		}
	}

	// Собираем все возможные стены, которые защитят от удара бомб по вертикали
	for (int i = 0; i < cols.size(); ++i) {
		if (cols[i].size() > 0)
		for (int j = 0; j < cols[i].size()-1; ++j) {
			alls.push_back({});
			int a = cols[i][j];
			int b = cols[i][j+1];
			for (int k = a; k < b; ++k) {
				alls.back().push_back(k);
			}
		}
	}

	remake_wall();

	// Проходимся по всем стенам, и смотрим те, которые защищают только одну пару, удаляем их из списка, если там есть еще стены, которые защищают эту пару
	for (int i = 0; i < wall.size(); ++i) {
		if (wall[i].size() == 1) {
			auto& current_pair = alls[wall[i][0]];
			if (current_pair.size() > 1)
				current_pair.erase(find(current_pair.begin(), current_pair.end(), i));
		}
	}

	remake_wall();

	// Перебираем все стены со всеми и находим те, которые полностью входят в другие, удаляем те, что имеют меньший размер
	int deleted_count = 0;
	auto check_walls = [&] (int w1n, int w2n) {
		if (wall[w1n].size() > wall[w2n].size())
			swap(w1n, w2n);

		if (wall[w1n].size() == 0)
			return;

		auto w1 = wall[w1n];
		auto w2 = wall[w2n];

		for (int i = 0; i < w1.size(); ++i) {
			auto finded = find(w2.begin(), w2.end(), w1[i]);
			if (finded == w2.end())
				return;
		}

		deleted_count++;

		for (int i = 0; i < w1.size(); ++i) {
			int para = w1[i];
			auto finded = find(alls[para].begin(), alls[para].end(), w1n);
			alls[para].erase(finded);
		}

		wall[w1n].clear();
	};

	for (int i = 0; i < wall.size(); ++i) {
		for (int j = i+1; j < wall.size(); ++j) {
			check_walls(i, j);
		}
	}

	cout << "deleted: " << deleted_count << endl;

	remake_wall();

	// Проходим по всем парам бомб, что защищают только одна стена, и сразу считаем эту стену как обязательную к использованию, и удаляем её из других мест
	auto copy = alls;
	vector<int> to_delete;
	for (int i = 0; i < alls.size(); ++i) {
		if (copy[i].size() == 1 && alls[i].size() == 1) {
			result++;
			int wall_no = alls[i][0];
			auto& current_wall = wall[wall_no];
			for (int j = 0; j < current_wall.size(); j++) {
				auto& current_pair = alls[current_wall[j]];
				current_pair.clear();
			}
		}
	}

	// Удаляем пары с 0 стен
	copy = alls;
	alls.clear();
	start_delete:
	for (int i = 0; i < copy.size(); i++) {
		if (copy[i].size() != 0) {
			alls.push_back(copy[i]);
		}
	}
	remake_wall();

	sort(alls.begin(), alls.end(), [] (auto a, auto b) -> bool {
		return a.size() < b.size();
	});

	remake_wall();

	for (int i = 0; i < wall.size(); ++i) {
		for (int j = i + 1; j < wall.size(); ++j) {
			check_walls(i, j);
		}
	}

	cout << "deleted: " << deleted_count << endl;

	sort(alls.begin(), alls.end(), [](auto a, auto b) -> bool {
		return a.size() < b.size();
	});

	remake_wall();

	for (int i = 0; i < wall.size(); ++i) {
		for (int j = i + 1; j < wall.size(); ++j) {
			check_walls(i, j);
		}
	}

	cout << "deleted: " << deleted_count << endl;

	cout << result << endl;

	remake_wall();

	covered.resize(alls.size(), false);
	travel(0, 1);
	if (min_depth == 3000)
		min_depth = 0;
	cout << result + min_depth;

	system("pause");
	return 0;

	/*// Перебираем все стены по разному количеству пар
	vector<int> wallmas;
	for (int i = 0; i < wall.size(); ++i)
		if (wall[i].size() != 0)
			wallmas.push_back(i);

	cout << wallmas.size() << " " << result << endl;
	vector<int> mas;
	for (int i = 1; i <= wallmas.size(); ++i) {
		mas.resize(i, 0);
		cout << i << " ";
		if (forall(wallmas, mas, 0, 0, i)) {
			cout << result + i;
			return 0;
		}
	}

	cout << result;
	return 0;*/
}

/*
	Алгоритм:
		Заполняем массив для строк и столбцов
		Затем проходимся по каждому массиву и записываем в пары все стены, которые перекрыкают эти две бомбы
		Далее проходимся по всем парам и находим те, где есть только одна стена, добавлем её к счетчику, и удаляем эту стену из всех остальных пар
		Далее перебираем все возможные варианты выбора по 1, 2, 3 и т. д. стен, чтобы покрыть всё имеющееся
		Далее перебираем все стены и смотрим сколько бомб используют эту стену. Если одна пара, то если эта пара использует хотя бы еще одну стену, то удаляем эту стену оттуда
*/