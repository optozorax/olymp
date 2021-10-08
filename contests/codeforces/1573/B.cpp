#include <algorithm>
#include <iostream>
#include <vector>
#include <set>
#include <queue>
#include <cmath>
 
using namespace std;

void debug_arr(char* name, vector<int> a) {
	cout << name << ": ";
	for (int i = 0; i < a.size(); ++i) {
		cout << a[i] << " ";
	}
	cout << endl;
}

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		int n;
		cin >> n;

		vector<int> a;
		for (int i = 0; i < n; ++i) {
			int temp;
			cin >> temp;
			a.push_back(temp);
		}

		vector<int> b;
		for (int i = 0; i < n; ++i) {
			int temp;
			cin >> temp;
			b.push_back(temp);
		}

		vector<int> b1(n);
		for (int i = 0; i < n; ++i) {
			b1[b[i] / 2 - 1] = i;
		}

		for (int i = 1; i <= n; ++i) {
			b1[n-i - 1] = min(b1[n-i - 1], b1[n-i]);
		}

		int my_min = n*3;
		for (int i = 0; i < n; ++i) {
			my_min = min(my_min, i + b1[(a[i]-1)/2]);
		}

		cout << my_min << endl;
	}
}

/* 

делаем массив индексов чётных чисел
делаем по нему суффикс мин
проходимся по нему с первым массивом и считаем минимум

 */