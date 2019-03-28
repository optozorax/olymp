#define _USE_MATH_DEFINES

#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <functional>
#include <sstream>
#include <string>
using namespace std;
typedef int64_t bint;

vector<double> points;

int count(int pos, double max) {
	int counted = 0;
	bool isNext = false;
	for (int i = pos; i < points.size(); ++i) {
		if (points[i] < max) {
			if (i == points.size() - 1) 
				isNext = true;
			counted++;
		} else {
			break;
		}
	}
	if (isNext) {
		for (int i = 0; i < points.size(); ++i) {
			if (points[i]+2*M_PI < max) {
				counted++;
			} else {
				break;
			}
		}
	}

	return counted;
}

void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

int main() {
	//test("8\n3 1\n1 1\n2 3\n 2 2\n-1 1\n-1 2\n-8 -2\n1 -5\n");

	double toSum = atan2(1, 1005) - atan2(1, 1006);

	bint n;
	cin >> n;
	for (int i = 0; i < n; ++i) {
		bint x, y;
		cin >> x >> y;
		points.push_back(M_PI + atan2(y, x));
	}
	sort(points.begin(), points.end(), less<double>());
	int max = 0;
	for (int i = 0; i < points.size(); ++i) {
		int res = count(i, points[i] + M_PI/2.0 + toSum);
		if (res > max) max = res;
	}

	cout << max;

	//system("pause");
}