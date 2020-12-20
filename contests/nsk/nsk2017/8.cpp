#include <iostream>
#include <sstream>
#include <iterator>
#include <fstream>

using namespace std;

typedef int64_t bint;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

#define sqr(c) ((c)*(c))

struct point { bint x, y; };

point operator-(point a, point b) { return {a.x-b.x, a.y-b.y}; }
istream& operator>>(istream& in, point& p) { in >> p.x >> p.y; return in; }

bint sqrLength(point a, point b) { return sqr(a.x-b.x) + sqr(a.y-b.y); }

bool isAngle90(point a, point b, point c) {
	point ab = a - b;
	point cb = c - b;
	// a.x/a.y == -c.y/c.x
	return ab.x*cb.x + cb.y*ab.y == 0;
}

bool check(point a, point b, point c, bint& l1, bint& l2) {
	if (isAngle90(a, b, c)) {
		l1 = sqrLength(a, b);
		l2 = sqrLength(c, b);
		return true;
	} else {
		return false;
	}
}

bool isRightTriangle(point a, point b, point c, bint& l1, bint& l2) {
	if (check(a, b, c, l1, l2)) return true;
	if (check(a, c, b, l1, l2)) return true;
	if (check(c, a, b, l1, l2)) return true;

	return false;
}

int main() {
	//test("0 0 10 0 0 12\n10 10 22 10 10 20");
	//test("0 0 10 0 0 15\n10 10 24 0 0 21");

	point a1, b1, c1;
	point a2, b2, c2;

	cin >> a1 >> b1 >> c1;
	cin >> a2 >> b2 >> c2;

	bint la1 = 0, lb1 = 0;
	bint la2 = 0, lb2 = 0;
	
	if (isRightTriangle(a1, b1, c1, la1, lb1) && isRightTriangle(a2, b2, c2, la2, lb2)) {
		if ((la1 == la2 && lb1 == lb2) || (la1 == lb2 && lb1 == la2)) {
			cout << "YES";
		} else {
			cout << "NO";
		}
	} else {
		cout << "NO";
	}

	//system("pause");
}