#include <iostream>
#include <vector>
#include <map>
#include <cmath>

using namespace std;
typedef int64_t bint;

struct point { double x, y; };
point operator+(const point& a, const point& b) { return { a.x + b.x, a.y + b.y }; }
point operator-(const point& a, const point& b) { return { a.x - b.x, a.y - b.y }; }
point operator*(const point& a, const point& b) { return { a.x * b.x, a.y * b.y }; }
point operator/(const point& a, const point& b) { return { a.x / b.x, a.y / b.y }; }

point operator*(const point& a, const double& b) { return { a.x * b, a.y * b }; }
point operator/(const point& a, const double& b) { return { a.x / b, a.y / b }; }

point operator*(const double& a, const point& b) { return { a * b.x, a * b.y }; }
point operator/(const double& a, const point& b) { return { a / b.x, a / b.y }; }

double dot(const point& a, const point& b) { return a.x*b.x + a.y*b.y; }
double dist(const point& a, const point& b) { 
	#define sqr(a) ((a)*(a))
	double x = sqr(a.x-b.x)+sqr(a.y-b.y);
	return sqrt(x); 
}

struct line { point o, d; };

double project(const line& a, const point& b) {
	return dot(b-a.o, a.d)/dot(a.d, a.d);
}

double dist(const line& a, const point& b) {
	return dist(a.o + a.d*project(a, b), b);
}

bool intersect(const line& a, const pair<point, point>& b) {
	double dista = dist(a, b.first);
	double distb = dist(a, b.second);
	double prec = 1.0 / pow(10.0, 8);
	if (fabs(dista) < prec || fabs(distb) < prec) {
		return true;
	}

	//if (((dista+distb)/2.0-distc) > 0.01)
	double distc = dist(a, (b.first + b.second) / 2.0);
	if (dista < distb) swap(dista, distb);
	dista -= distb;
	distc -= distb;
	if ((dista - 2*distc) > 0.01)
		return true;

	return false;
}

int main() {
	vector<point> ps;
	vector<pair<point, point>> a;
	bint n;
	cin >> n;
	for (int i = 0; i < n; i++) {
		bint x1, y1, x2, y2;
		cin >> x1 >> y1 >> x2 >> y2;
		a.push_back({ {x1, y1}, {x2, y2} });
		
	}
	//n = 4;
	//a = { {{-10, -10}, {-10, 10}}, {{-9, 10}, {9, 10}}, {{10, 10}, {10, -10}}, {{9, -10}, {-9, -10}} };

	for (auto& i : a) {
		ps.push_back(i.first);
		ps.push_back(i.second);
	}

	auto test_line = [&](line l) -> bint {
		if (l.d.x == 0 && l.d.y == 0)
			return 0;
		bint result = 0;
		for (auto& i : a) {
			if (intersect(l, i))
				result++;
		}
		return result;
	};

	bint max1 = 0;
	for (bint i = 0; i < ps.size(); i++) {
		for (bint j = i+1; j < ps.size(); j++) {
			int res = test_line({ ps[i], ps[j]-ps[i] });
			if (res > max1)
				max1 = res;
		}
	}

	cout << max1;

	//system("pause");
}