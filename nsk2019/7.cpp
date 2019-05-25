#define _USE_MATH_DEFINES

#include <iostream>
#include <vector>
#include <map>
#include <cmath>
#include <functional>
#include <numeric>
#include <algorithm>
#include <bitset>
#include <cassert>

using namespace std;
typedef int64_t bint;

double dist1(double x1, double y1, double x2, double y2) {
	#define sqr(a) ((a)*(a))
	return sqrt(sqr(x1 - x2) + sqr(y1 - y2));
}

double dist(double x1, double y1, double x2, double y2) {
	x1 = x1 / 180 * M_PI;
	y1 = y1 / 180 * M_PI;
	x2 = x2 / 180 * M_PI;
	y2 = y2 / 180 * M_PI;
	double d1 = dist1(x1, y1, x2, y2);
	double d2 = min(d1, dist1(x1+360, y1, x2, y2));
	double d3 = min(d2, dist1(x1-360, y1, x2, y2));
	double d4 = min(d3, dist1(x1, y1+180, x2, y2));
	double d5 = min(d4, dist1(x1, y1-180, x2, y2));
	return d5;
}

int main() {
	bint n;
	double d;
	bint a;
	bint b;
	cin >> n >> d >> a >> b;
	vector<pair<double, double>> p;
	for (int i = 0; i < n; i++) {
		double x, y;
		cin >> x >> y;
		p.push_back({ x, y });
	}

	/*n = 2;
	d = 2;
	a = 1;
	b = 2;
	vector<pair<double, double>> p = { {0, 0}, {0, 90} };*/

	vector<vector<double>> dd(n, vector<double>(n, 0));
	for (int i = 0; i < p.size(); i++) {
		for (int j = 0; j < p.size(); j++) {
			dd[i][j] = dist(p[i].first, p[i].second, p[j].first, p[j].second);
		}
	}

	vector<double> f(n, 100);
	vector<int> visited(n, 0);
	visited[a-1] = 1;
	f[a-1] = 0;
	
	bool snova = false;
	start:
	snova = false;
	for (int i = 0; i < n; i++) {
		if (visited[i] == 1) {
			for (int j = 0; j < n; j++) {
				if (visited[j] != 2) {
					f[j] = min(f[j], f[i] + dd[i][j]);
					visited[j] = 1;
					snova = true;
				}
			}
			visited[i] = 2;
		}
	}
	if (snova)
		goto start;

	if (f[b-1] > d)
		cout << -1;
	else
		cout << f[b-1];

	//system("pause");
}