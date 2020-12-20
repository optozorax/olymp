#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
	int n;
	cin >> n;
	vector<long> mas(n, 0);
	for (int i = 0; i < n; i++) {
		cin >> mas[i];
	}
	
	sort(mas.begin(), mas.end());
	mas.erase(unique(mas.begin(), mas.end()), mas.end());

	int max = 0;
	long sum = 0;
	long last = mas.size() - 1;
	for (int i = mas.size() - 1; i >= 0; --i) {
		sum++;
		for (int j = last; j >= i; --j) {
			last = j;
			if (mas[last]-mas[i] >= n)
				sum--;
			else
				break;
		}
		if (sum > max)
			max = sum;
	}
	
	cout << n-max;
}