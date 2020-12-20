// Ой, а я даже не заметил, что эту задачу сняли с решения. И ведь не отправить, не проверить... В общем, мою ситуацию идеально описывает это видео: https://www.youtube.com/watch?v=9udxWFXpj_U

#include <iostream>
#include <vector>
#include <algorithm>
#include <sstream>

using namespace std;

void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

typedef int64_t bint;

struct Elem
{
	Elem() : value(0), isUnique(true), neighbors(0), up(nullptr), down(nullptr), left(nullptr), right(nullptr) {}

	bint value;
	bool isUnique;
	bint neighbors;

	Elem* up; 
	Elem* down; 
	Elem* left; 
	Elem* right;
};

vector<vector<Elem>> elems;

void processNearestUp(int i, int j) {
	auto& current = elems[i][j];
	int value = current.value;
	for (int k = i-1; k >= 0; --k) {
		auto& elem = elems[k][j];
		if (elem.value == value) {
			elem.isUnique = false;
			elem.neighbors++;
			elem.down = &current;
			
			current.isUnique = false;
			current.neighbors++;
			current.up = &elem;

			Elem* k = current.up->up;
			while (k != nullptr) {
				k->neighbors++;
				k = k->up;
			}

			return;
		}
	}
}

void processNearestLeft(int i, int j) {
	auto& current = elems[i][j];
	int value = current.value;
	for (int k = j-1; k >= 0; --k) {
		auto& elem = elems[i][k];
		if (elem.value == value) {
			elem.isUnique = false;
			elem.neighbors++;
			elem.right = &current;
			
			current.isUnique = false;
			current.neighbors++;
			current.left = &elem;

			Elem* k = current.left->left;
			while (k != nullptr) {
				k->neighbors++;
				k = k->left;
			}
			
			return;
		}
	}
}

vector<Elem*> makeMas(int i, int j) {
	std::vector<Elem*> result;
	Elem* current = &elems[i][j];
	while (current != nullptr) {
		result.push_back(current);
		current = current->right;
	}

	return result;
}

bint deleteCount = 0;

void deleteNumber(Elem* current) {
	deleteCount++;

	current->value = -1;
	current->isUnique = true;
	current->neighbors = 0;

	if (current->left != nullptr) current->left->right = current->right;
	if (current->right != nullptr) current->right->left = current->left;
	if (current->up != nullptr) current->up->down = current->down;
	if (current->down != nullptr) current->down->up = current->up;

	Elem* k;

	k = current->right;
	while (k != nullptr) {
		k->neighbors--;
		k = k->right;
	}

	k = current->left;
	while (k != nullptr) {
		k->neighbors--;
		k = k->left;
	}

	k = current->up;
	while (k != nullptr) {
		k->neighbors--;
		k = k->up;
	}

	k = current->down;
	while (k != nullptr) {
		k->neighbors--;
		k = k->down;
	}
}

int main() {
	//test("3 3\n2 2 4\n1 2 2\n3 1 3\n");
	//test("3 3\n1 1 1\n1 1 1\n1 1 1\n");

	// Сначала вводим матрицу
	bint n, m;
	cin >> m >> n;
	elems.resize(m, vector<Elem>(n));
	for (int i = 0; i < m; ++i) {
		for (int j = 0; j < n; ++j) {
			cin >> elems[i][j].value;
		}
	}

	// Считаем всех соседей и их количество
	for (int i = 0; i < m; ++i) {
		for (int j = 0; j < n; ++j) {
			processNearestLeft(i, j);
			processNearestUp(i, j);
		}
	}

	// Перебираем построчно все числа, что не являются уникальными
	for (int i = 0; i < m; ++i) {
		for (int j = 0; j < n; ++j) {
			if (!elems[i][j].isUnique) {
				// Когда встречаем такое, формируем массив всех чисел, что находятся в данной строке
				auto mas = makeMas(i, j);

				// сортируем их по количеству влияний
				sort(mas.begin(), mas.end(), [] (auto& a, auto& b) -> bool {
					return a->neighbors > b->neighbors;
				});

				// И потихоньку вычеркиваем все первые числа, кроме последнего
				for (int k = 0; k < mas.size() - 1; ++k)
					deleteNumber(mas[k]);

				// У последнего вычеркиваем все ниже него
				Elem* k = mas.back()->down;
				while (k != nullptr) {
					deleteNumber(k);
					k = k->down;
				}
			}
		}
	}
	
	cout << deleteCount;

	//cout << endl; system("pause");
}