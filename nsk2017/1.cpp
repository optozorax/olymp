#include <iostream>
#include <vector>
#include <algorithm>
#include <sstream>

using namespace std;
typedef int64_t bint;

void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

int main() {
	//test("3\n3 1 5\n");
	//test("3\n4 1 5\n");
	//test("10\n5 6 7 8 5 6 7 8 9 10\n");
	//test("5\n5 4 3 2 1\n");		

	// Считываем данныеx
	bint n;
	cin >> n;
	vector<bint> mas(n);
	for (int i = 0; i < n; ++i)
		cin >> mas[i];

	struct Replace
	{
		bint count;
		bint number;
	};
	vector<Replace> replaces;
	replaces.reserve(mas.size());

	// Если это первое число, то есть два варианта: сделать 0 замен, и оставить его как есть, либо сделать одну замену, и сделать это число равным 1. Оба варианта закидываются в массив замен.
	replaces.push_back({0, mas[0]});
	replaces.push_back({1, 1});

	// Идем от первого числа до последнего
	for (int i = 1; i < mas.size(); ++i) {
		vector<Replace> new_replaces;
		// Перебираются все замены
		for (auto& r : replaces) {
			if (mas[i] > r.number) {
				// Если текущее число больше чем число на замене, то данная замена закидывается как есть вместе с заменой, когда текущее число делают максимально минимальным
				new_replaces.push_back({r.count, mas[i]});
				new_replaces.push_back({r.count+1, r.number+1});
			} else {
				// Если текущее число меньше, чем число на замене, то закидывается только замена, когда число заменили текущее число на следующее минимальное
				new_replaces.push_back({r.count+1, r.number+1});
			}
		}

		sort(new_replaces.begin(), new_replaces.end(), [] (auto& a, auto& b) -> bool {
			if (a.count != b.count)
				return a.count < b.count;
			else
				return a.number < b.number;
		});

		// Далее из нового массиива замен остаются только те, что имеют минимальное число, при одинаковом числе замен
		replaces.clear();
		for (auto j = new_replaces.begin(); j != new_replaces.end(); ++j) {
			replaces.push_back(*j);
			while (j+1 != new_replaces.end() && j[1].count == j[0].count) 
				j++;
		}
	}

	// В итоге выводим минимальное число замен
	cout << replaces[0].count;

	//cout << endl; system("pause");
}