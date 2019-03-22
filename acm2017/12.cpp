#include <iostream>
#include <vector>
#include <map>
#include <string>
#include <sstream>
#include <algorithm>

using namespace std;

//void test(string str) {string* str1 = new string(str);istringstream* iss = new istringstream(*str1);cin.rdbuf(iss->rdbuf());}

typedef int pos;
typedef int num;

int n;
int k;
vector<num> h;
vector<pos> result;
vector<pos> sort_pos;
vector<pos> pos_max;

void read_data(void) {
	cin >> n >> k;
	h.resize(n);
	sort_pos.resize(n);
	pos_max.resize(n);
	for (int i = 0; i < n; ++i)
		cin >> h[i];
}

int main() {
	//test("5 2\n1 2 3 7 4");
	read_data();

	for (int i = 0; i < n; ++i)
		sort_pos[i] = i;
	sort(sort_pos.begin(), sort_pos.end(), [] (auto a, auto b) -> bool {
		return h[a] > h[b];
	});
	for (int i = 0; i < n; ++i)
		pos_max[sort_pos[i]] = i;

	int i = 0;
	while (i < n) {
		for (; i < n; ++i) {
			if (sort_pos[i] != -1)
				break;
		}
		if (i == n)
			break;

		result.push_back(sort_pos[i]+1);

		int start = sort_pos[i];
		for (int j = start; j < min(start+k+1, n); ++j)
			sort_pos[pos_max[j]] = -1;
		for (int j = max(start-k, 0); j < start; ++j)
			sort_pos[pos_max[j]] = -1;
	}

	cout << result.size() << endl;
	for (auto& i : result)
		cout << i << " ";
}

/*

Сначала создаем массив позиций максимумов по убыванию
Затем создаем массив положений в массиве максимумов

Затем пробегаемся по массиву, находим первый максимум не равный -1
Когда нашли, занимаем его
	Так же делаем все вершины вокруг
Повторяем снова

*/