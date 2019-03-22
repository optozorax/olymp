/*

Сначала сотрируем массив
Строрим массив массивов для каждого следующего максимума на каких позициях он находится

Берем два самых крайних числа

Смотрим самое максимальное число, которое больше или равно двух текущих, которое находится между текущими двумя числами
	Применяем эту рекурсию ко всем отрезкам, образующимся таким образом
Если таких точек нет, то возвращаем длину текущего отрезка

*/

#include <vector>
#include <string>
#include <sstream>
#include <algorithm>
#include <functional>
#include <iostream>
#include <fstream>
#include <map>

using namespace std;

//void test(string str) {string* str1 = new string(str);istringstream* iss = new istringstream(*str1);cin.rdbuf(iss->rdbuf());}
//string readFromFile(string fileName) {std::ifstream fin(fileName);std::string result;result.assign(std::istreambuf_iterator<char>(fin), std::istreambuf_iterator<char>());fin.close();return result;}

vector<int> mas; // по i возвращает какой там монстер по храбрости
vector<pair<int, vector<int>>> max_poses; // по i возвращает номер текущего максимума, и все его позиции
vector<int> max_indx; // по i возвращает позицию заданного числа в max_poses

void readData(void) {
	int n;
	cin >> n;
	mas.resize(n);
	for (auto& i : mas)
		cin >> i;
}

int findMax(int a, int b) {
	int av = mas[a], bv = mas[b];
	int my_max = max(max_indx[av], max_indx[bv]);
	int max_pos = -1;
	for (int i = 0; i <= my_max; ++i) {
		const auto& poses = max_poses[i].second;
		for (int j = 0; j < poses.size(); ++j) {
			if (poses[j] > a && poses[j] < b) {
				max_pos = i;
				goto end_cycle;
			}
		}
	}
	end_cycle:

	if (max_pos == -1) {
		return b - a + 1;
	} else {
		const auto& poses = max_poses[max_pos].second;
		vector<int> segments;
		segments.push_back(a);
		for (int i = 0; i < poses.size(); ++i) {
			int pos = poses[i];
			if (pos > a && pos < b)
				segments.push_back(pos);
		}
		segments.push_back(b);

		int my_max = -1;
		for (int i = 0; i < segments.size() - 1; i++)
			my_max = max(my_max, findMax(segments[i], segments[i+1]));
		return my_max;
	}
}

int main() {
	//test("10\n5 1 8 1 7 2 2 5 6 7");
	//test("14\n7 1 2 3 4 8 5 6 8 2 3 7 4 7");
	//test("7\n5 3 8 6 7 6 5");
	//test(readFromFile("testacm2017/9/input16.txt"));
	//cout << readFromFile("testacm2017/9/output16.txt");
	readData();

	vector<pair<int, int>> mas_adapter(mas.size());
	for (int i = 0; i < mas.size(); ++i)
		mas_adapter[i] = {i, mas[i]};
	stable_sort(mas_adapter.begin(), mas_adapter.end(), [] (auto a, auto b) -> bool {
		return a.second > b.second;
	});

	max_poses.reserve(mas.size());
	max_indx.resize(10010, -1);

	max_poses.push_back({mas_adapter[0].second, {}});
	max_indx[max_poses[0].first] = 0;
	for (auto& i : mas_adapter) {
		if (max_poses.back().first == i.second) {
			max_poses.back().second.push_back(i.first);
		} else {
			auto& last = max_poses.back().second;
			sort(last.begin(), last.end(), less<int>());
			max_poses.push_back({i.second, {i.first}});
			max_indx[i.second] = max_poses.size() - 1;
		}
	}

	cout << findMax(0, mas.size()-1);
}