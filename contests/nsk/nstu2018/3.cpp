#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
typedef int64_t bint;

bint average(const vector<bint>& mas) {
	bint sum = 0, count = 0;
	for (auto& i : mas) {
		if (i != 0) {
			sum += i;
			count++;
		}
	}

	return sum/count;
}

bint sub(const vector<bint>& mas, bint s) {
	bint sum = 0;
	for (auto& i : mas) {
		if (i != 0) {
			sum += abs(i-s);
		}
	}
	return sum;
}

int main() {
	bint n;
	cin >> n;
	vector<bint> mas(1000, 0);
	for (int i = 0; i < n; ++i) {
		bint temp;
		cin >> temp;
		mas[temp]++;
	}

	auto mymax = max_element(mas.begin(), mas.end());
	cout << sub(mas, *mymax);

	//system("pause");
}