#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <sstream>
#include <map>

//-----------------------------------------------------------------------------
using namespace std;

typedef int64_t bint;

struct input 
{
	bint n, m, k;
	vector<bint> t;
} in;

//-----------------------------------------------------------------------------
bint getCoveredCount(bint i) {
	// Возвращает количество деталей, которые робот не сможет произвести, если будет делать i-ю
	bint count = in.t[i]/in.m;
	if (in.t[i]%in.m == 0 && count > 0)
		count--;
	return min(in.n-i-1, count);
}

//-----------------------------------------------------------------------------
vector<bint> cached;
bint findMaxDetails(bint i) {
	if (i >= in.n)
		return 0;

	auto func = [] (bint i) -> bint {
		// Сама функция
		bint covered = getCoveredCount(i);
		bint mymax;

		// Проверяем, успеет ли робот сделать текущую деталь
		if (i * in.m + in.t[i] <= in.k)
			mymax = 1 + findMaxDetails(i + covered + 1);
		else
			mymax = 0;

		// Обходим все детали, которые робот не может сделать, и среди них выбираем максимум
		for (int j = 0; j < covered; ++j)
			mymax = max(mymax, findMaxDetails(i+j+1));

		return mymax;
	};

	//-------------------------------------------------------------------------
	// Кеширование
	auto& current_answer = cached[i];
	if (current_answer != 0) 
		return current_answer;
	current_answer = func(i);
	return current_answer;
}

//-----------------------------------------------------------------------------
void readData(void) {
	cin >> in.n >> in.m >> in.k;
	in.t.resize(in.n, 0);
	cached.resize(in.n, 0);
	for (int i = 0; i < in.n; ++i)
		cin >> in.t[i];
}

//-----------------------------------------------------------------------------
void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

int main() {
	//test("3 10 100\n30 10 10"); // Из условия задачи
	//test("5 10 60\n12 25 7 23 10"); // Мой тест
	//test("3 10 20\n11 10 10"); // Тест 4

	readData();
	cout << findMaxDetails(0);
}