/* Послание к тому, кто будет это вдруг читать: извини меня, но я снова не знаю, почему это правильно... Я подумал, может здесь прокатит такая же штука как и в третьей, типо жадным алгоритмом. Ну и оно сработало. Я не знаю почему. Я не могу для себя доказать что это правильное решение. Видимо у меня появилось новое правило: если не знаешь как что-то оптимизировать, как решить динамическим программированием, просто сделай жадный алгоритм. */

#define _USE_MATH_DEFINES

#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <functional>
#include <sstream>
#include <string>
#include <bitset>
#include <numeric>
#include <algorithm>
#include <iomanip>
using namespace std;
typedef int64_t bint;
void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

vector<bint> parent;
vector<bint> w, w1;
vector<bool> leaf;
bint leaf_count = 0;

void recalc_value() {
	w1 = w;
	for (int i = 1; i < parent.size(); ++i)
		w1[i] += w1[parent[i]];
}

int find_max() {
	int mymax = -1;
	for (int i = 0; i < w1.size(); ++i)
		if (leaf[i])
			if (mymax == -1 || w1[i] > w1[mymax]) 
				mymax = i;
	return mymax;
}

void delete_money(int i) {
	w[i] = 0;
	if (i != 0)
		delete_money(parent[i]);
} 

int main() {
	//test("5 4\n0 0 1 1\n1 1 1 1 1\n");
	//test("5 6\n0 0 1 1\n1 1 1 1 1\n");
	bint n, s;
	cin >> n >> s;
	parent.resize(n, 0);
	w.resize(n, 0);
	leaf.resize(n, true);
	for (int i = 1; i < n; ++i) {
		cin >> parent[i];
		leaf[parent[i]] = false;
	}
	for (int i = 0; i < n; ++i)
		cin >> w[i];

	for (auto i : leaf)
		if (i)
			leaf_count++;

	recalc_value();

	int sum = 0;
	int count = 0;
	while (leaf_count) {
		leaf_count--;
		bint mymax = find_max();
		sum += w1[mymax];
		count++;
		delete_money(mymax);
		recalc_value();
		if (sum >= s) break;
	}

	if (sum < s) cout << "IMPOSSIBLE";
	else cout << count;

	//system("pause");
}