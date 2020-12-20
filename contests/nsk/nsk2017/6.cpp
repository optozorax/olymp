#include <iostream>
#include <vector>
#include <algorithm>
#include <list>
#include <functional>
#include <cmath>
#include <sstream>

using namespace std;
typedef int bint;

void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

vector<bint> p;

void preparePrimes() {
	// Находит все простые числа до миллиона

	bint upper = sqrt(1000000)+20;
	p.push_back(2);
	for (int i = 3; i < upper; ++i) {
		for (auto& j : p) {
			if (i % j == 0)
				goto not_push_prime;
		}
		p.push_back(i);
		not_push_prime:;
	}
}

void getPrimes(bint n, vector<int>& result) {
	// Возвращает все простые делители числа

	result.clear();
	bint nsq = sqrt(n);
	for (auto& i : p) {
		if (i > nsq)
			break;

		while (n != 1 && n % i == 0) {
			result.push_back(i);
			n /= i;
		}

		if (n == 0)
			break;
	}

	if (n != 1)
		result.push_back(n);
}

void forEachDivisor(bint n, std::function<void(int)> f) {
	// Перебирает каждый делитель числа n, и применяет к нему функцию f.

	static vector<int> primes;
	getPrimes(n, primes);
	bint count = pow(2, primes.size());
	f(1);
	for (int i = 1; i < count; ++i) {
		int icopy = i;
		bint divisor = 1;
		for (int i = 0; icopy != 0 && i < 20; ++i) {
			if (icopy % 2)
				divisor *= primes[i];
			icopy /= 2;
		}
		f(divisor);
	}
}

int main() {
	//test("27 10");
	//test("1000000 11");
	//test("2 1");

	// подготавливаем массив простых чисел до корня из миллиона
	preparePrimes();

	// Считываем данные
	bint n, k;
	cin >> n >> k;

	vector<int> numbers(n+1, -1);
	numbers[k] = 0;
	for (int i = k; i < n; ++i) {
		bint min_replaces = numbers[i];

		forEachDivisor(i, [&] (int k) {
			if (i+k < n+1) {
				auto& elem = numbers[i+k];
				if (elem != -1)
					elem = min<int>(min_replaces+1, elem);
				else
					elem = min_replaces+1;
			}
		});
	}

	cout << numbers.back();

	//cout << endl; system("pause");
}