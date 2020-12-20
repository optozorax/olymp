#include <iostream>
#include <vector>
#include <cmath>
#include <map>

using namespace std;

typedef int64_t bint;

struct point {
	double x, y;
};

point operator+(const point& a, const point& b) { return { a.x + b.x, a.y + b.y }; }
point operator-(const point& a, const point& b) { return { a.x - b.x, a.y - b.y }; }
point operator*(const point& a, const double& b) { return { a.x * b, a.y * b }; }
point operator*(const double& a, const point& b) { return { a * b.x, a * b.y }; }
point operator/(const point& a, const double& b) { return { a.x / b, a.y / b }; }
point operator/(const double& a, const point& b) { return { a / b.x, a / b.y }; }

point perpendicular(point a) {
	return { a.y, -a.x };
}

struct line {
	point o, d;
};

double dot(point a, point b) {
	return a.x * b.x + a.y * b.y;
}

double project(line a, point b) {
	return (dot(b, a.d) - dot(a.o, a.d)) / dot(a.d, a.d);
}

bool equal(point a, point b) {
	return fabs(a.x - b.x) + fabs(a.y - b.y) < 0.0001;
}

line getSymmLine(point a, point b) {
	point ab = a - b;
	point d = perpendicular(ab);
	point c = (a + b)/2.0;
	return { c, d };
}

point getSymmPoint(line a, point b) {
	double t = project(a, b);
	point online = a.o + a.d * t;
	point check = perpendicular(a.d);
	point d = b - online;
	return online - d;
}

vector<point> points;
map<bint, bint> m;

bint getPos(point a) {
	return a.x * 101 + a.y;
}

bool checkSymm(line l) {
	for (auto& i : points) {
		auto s = getSymmPoint(l, i);
		bool pointExists = m[getPos(s)] != 0;
		bool pointsAreSame = pointExists && equal(points[m[getPos(s)]-1], s);
		if (!pointsAreSame)
			return false;
	}
	return true;
}

int main() {
	bint n;
	cin >> n;
	for (int i = 0; i < n; i++) {
		point a;
		cin >> a.x >> a.y;
		points.push_back(a);
	}

	for (int i = 0; i < points.size(); i++)
		m[getPos(points[i])] = i+1;

	for (int i = 0; i < points.size(); i++) {
		for (int j = i + 1; j < points.size(); j++) {
			if (checkSymm(getSymmLine(points[i], points[j]))) {
				cout << "YES";
				return 0;
			}
		}
	}

	cout << "NO";
}