#include <algorithm>
#include <cmath>
#include <exception>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <memory>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

typedef double realreal;

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
// Мой код Мой код Мой код Мой код Мой код Мой код Мой код Мой код Мой код Мой 
using namespace std;

//-----------------------------------------------------------------------------
const realreal mypi = 3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196442881097566593344612847564823378678316527120190914;

struct point { realreal x, y; };
struct line { point a, b; };

struct input 
{
	int n;
	vector<point> p;
	point min;
	point max;
} in;
const realreal line_around = 0.0000001;
const realreal delete_around = 0.00000001;
const realreal zero_around = 0.00001;
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// КОД ДЛЯ ОПРЕДЕЛЕНИЯ, НАХОДИТСЯ ЛИ ТОЧКА ВНУТРИ ПОЛИГОНА
// Нагло скопирован с https://github.com/pglauner/point_in_polygon

//-----------------------------------------------------------------------------
struct Line {
	point p1;
	point p2;
};

//-----------------------------------------------------------------------------
static int ccw(const point& p0, const point& p1, const point& p2) {
	double dx1 = double(p1.x - p0.x);
	double dy1 = double(p1.y - p0.y);
	double dx2 = double(p2.x - p0.x);
	double dy2 = double(p2.y - p0.y);
	double dx1dy2 = dx1 * dy2;
	double dx2dy1 = dx2 * dy1;

	if (dx1dy2 > dx2dy1) {
		return 1;
	} else if (dx1dy2 < dx2dy1) {
		return -1;
	}
	else {
		if (dx1 * dx2 < 0 || dy1 * dy2 < 0) {
			return -1;
		} else if (dx1 * dx1 + dy1 * dy1 >= dx2 * dx2 + dy2 * dy2) {
			return 0;
		} else {
			return 1;
		}
	}
}

//-----------------------------------------------------------------------------
inline int intersect(const Line& line1, const Line& line2) {
	int ccw11 = ccw(line1.p1, line1.p2, line2.p1); if (ccw11 == 0) return 1;
	int ccw12 = ccw(line1.p1, line1.p2, line2.p2); if (ccw12 == 0) return 1;
	if (!(ccw11 * ccw12 < 0))
		return 0;
	int ccw21 = ccw(line2.p1, line2.p2, line1.p1); if (ccw21 == 0) return 1;
	int ccw22 = ccw(line2.p1, line2.p2, line1.p2); if (ccw22 == 0) return 1;
	return (ccw21 * ccw22 < 0) ? 1 : 0;
}

//-----------------------------------------------------------------------------
inline int getNextIndex(int n, int current) {
	return current == n - 1 ? 0 : current + 1;
}

//-----------------------------------------------------------------------------
bool pointInPolygon(point testPoint) {
	static std::vector<point> polygon;
	static bool isFirst = true;
	if (isFirst) {
		polygon.reserve(100);
		isFirst = false;
	}

	polygon = in.p;
	int n = polygon.size();
	Line xAxis;
	Line xAxisPositive;

	point startPoint;
	point endPoint;
	Line edge;
	Line testPointLine;

	int i;
	double startNodePosition;
	int count;
	int seenPoints;

	startPoint.x = 0;
	startPoint.y = 0;

	xAxis.p1.x = 0;
	xAxis.p1.y = 0;
	xAxis.p2.x = 0;
	xAxis.p2.y = 0;
	xAxisPositive.p1.x = 0;
	xAxisPositive.p1.y = 0;
	xAxisPositive.p2.x = 0;
	xAxisPositive.p2.y = 0;

	startNodePosition = -1;

	for (i = 0; i < n; i++) {
		if (testPoint.x == polygon[i].x && testPoint.y == polygon[i].y) {
			return 1;
		}

		polygon[i].x -= testPoint.x;
		polygon[i].y -= testPoint.y;

		if (polygon[i].y != 0) {
			startPoint.x = polygon[i].x;
			startPoint.y = polygon[i].y;
			startNodePosition = i;
		}

		if (polygon[i].x > xAxis.p2.x) {
			xAxis.p2.x = polygon[i].x;
			xAxisPositive.p2.x = polygon[i].x;
		}
		if (polygon[i].x < xAxis.p1.x) {
			xAxis.p1.x = polygon[i].x;
		}
	}

	testPoint.x = 0;
	testPoint.y = 0;
	testPointLine.p1 = testPoint;
	testPointLine.p2 = testPoint;

	for (i = 0; i < polygon.size(); i++) {
		edge.p1 = polygon[i];
		edge.p2 = polygon[getNextIndex(polygon.size(), i)];
		if (intersect(testPointLine, edge) == 1) {
			return 1;
		}
	}

	if (startNodePosition == -1) {
		return 0;
	}

	count = 0;
	seenPoints = 0;
	i = int(startNodePosition);
	while (seenPoints < n) {
		double savedX(polygon[getNextIndex(n, i)].x);
		int savedIndex = getNextIndex(n, i);
		do {
			i = getNextIndex(n, i);
			seenPoints++;
		} while (polygon[i].y == 0);
		endPoint.x = polygon[i].x;
		endPoint.y = polygon[i].y;
		if (startPoint.y * endPoint.y < 0) {
			edge.p1 = startPoint;
			edge.p2 = endPoint;
			if (savedIndex == i) {
				count += intersect(edge, xAxisPositive);
			}
			else if (savedX > 0) {
				count += intersect(edge, xAxis);
			}
		}
		startPoint = endPoint;
	}
	return count % 2;
}
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
realreal calcTriangleArea(const point& a, const point& b, const point& c) {
	// Считает площадь треугольника, заданного тремя точками
	return fabs((a.x-c.x)*(b.y-c.y)-(a.y-c.y)*(b.x-c.x))/2.0;
}

//-----------------------------------------------------------------------------
realreal calcLength(const point& d) {
	// Считает длину точки d
	return sqrt(d.x*d.x + d.y*d.y);
}

//-----------------------------------------------------------------------------
point calcMiddle(const point& a, const point& b) {
	// Считает точку посередине между a и b
	return {(a.x+b.x)/2.0, (a.y+b.y)/2.0};
}

//-----------------------------------------------------------------------------
realreal calcDistance(const point& a, const point& b) {
	// Считает расстояние между двумя точками
	return calcLength({a.x-b.x, a.y-b.y});
}

//-----------------------------------------------------------------------------
bool intersectLines(const line& l1, const line& l2, realreal& t1, realreal& t2, point& result) {
	// Пересекает отрезки
	realreal a = l2.b.x-l2.a.x;
	realreal b = -(l1.b.x-l1.a.x);
	realreal a1 = l2.b.y-l2.a.y;
	realreal b1 = -(l1.b.y-l1.a.y);

	realreal e = a * b1 - b * a1;
	if (fabs(e) < line_around)
		return false;
	realreal c = l1.a.x - l2.a.x;
	realreal c1 = l1.a.y - l2.a.y;
	t1 = -c * a1 + a * c1;
	t2 = b1 * c - b * c1;
	t1 /= e;
	t2 /= e;

	result.x = l1.a.x - b * t1;
	result.y = l1.a.y - b1 * t1;
	return true;
}

//-----------------------------------------------------------------------------
realreal getAngle(const point& center, const point& p) {
	// Возвращает на какой угол повернута точка p относительно center
	return atan2(center.y-p.y, center.x-p.x) + mypi;
}

//-----------------------------------------------------------------------------
template<class T, class F>
void travelPolygonEdges(vector<T> mas, F f) {
	// Обходит все ребра многоугольника
	for (int i = 0; i < mas.size()-1; ++i)
		f(i, i+1);
	f(mas.size()-1, 0);
}

//-----------------------------------------------------------------------------
realreal calcRadialArea(const point& p) {
	// Если точка не внутри полигона, то она нас не интересует, и, следовательно, покрываемая ей площадь равна нулю
	if (!pointInPolygon(p))
		return 0;

	// Сортируем все вершины по углу
	vector<pair<realreal, int>> mas(in.p.size());
	for (int i = 0; i < in.p.size(); ++i)
		mas[i] = pair<realreal, int>(getAngle(p, in.p[i]), i);
	sort(mas.begin(), mas.end());

	// Удаляем слишком близкие друг к другу вершины
	start_delete:
	for (int i = 0; i < mas.size() - 1; ++i) {
		if (fabs(mas[i].first - mas[i+1].first) < delete_around) {
			mas.erase(mas.begin() + i + 1);
			goto start_delete;
		}
	}
	if (fabs(mas[mas.size()-1].first - (2 * mypi + mas[0].first)) < delete_around)
		mas.erase(mas.begin() + mas.size() - 1);

	realreal sumArea = 0;
	travelPolygonEdges(mas, [&] (int c1, int c2) {
		static vector<line> segments(50);
		segments.clear();

		// Перебираем все ребра
		travelPolygonEdges(in.p, [&] (int p1, int p2) {
			point s1, s2;
			realreal t11, t12, t21, t22;
			// Находим точки, где линии сектора пересекают ребро
			if (intersectLines({in.p[p1], in.p[p2]}, {p, in.p[mas[c1].second]}, t11, t12, s1)
			&& intersectLines({in.p[p1], in.p[p2]}, {p, in.p[mas[c2].second]}, t21, t22, s2) 
			&& (fabs(t11 - 0) < zero_around || fabs(t11 - 1) < zero_around || (t11 > 0 && t11 < 1)) && (fabs(t12 - 0) < zero_around || t12 > 0) 
			&& (fabs(t21 - 0) < zero_around || fabs(t21 - 1) < zero_around || (t21 > 0 && t21 < 1)) && (fabs(t22 - 0) < zero_around || t22 > 0)) {
				segments.push_back({s1, s2});
			}
		});

		// Перебираем все отрезки и находим ближайший
		int min = 0;
		if (segments.size() != 0) {
			realreal minDist = calcDistance(p, calcMiddle(segments[0].a, segments[0].b));
			for (int i = 1; i < segments.size(); ++i) {
				realreal dist = calcDistance(p, calcMiddle(segments[i].a, segments[i].b));
				if (dist < minDist) {
					min = i;
					minDist = dist;
				}
			}

			// Считаем площадь центра с этим отрезком
			sumArea += calcTriangleArea(p, segments[min].a, segments[min].b);
		} else {
			//cout << "something went wrong" << endl;
		}
	});

	return sumArea;
}

//-----------------------------------------------------------------------------
void readData(void) {
	// Вводит все необходимые в программе данные
	cin >> in.n;
	in.p.resize(in.n);
	for (int i = 0; i < in.n; ++i)
		cin >> in.p[i].x >> in.p[i].y;

	// Находим минимум и максимум среди точек, чтобы у эволюции были границы
	in.min.x = 1010;
	in.min.y = 1010;
	in.max.x = -1010;
	in.max.y = -1010;
	for (const auto& i : in.p) {
		if (i.x < in.min.x)
			in.min.x = i.x;
		if (i.y < in.min.y)
			in.min.y = i.y;
		if (i.x > in.max.x)
			in.max.x = i.x;
		if (i.y > in.max.y)
			in.max.y = i.y;
	}
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

int main() {
	test("");

	//-------------------------------------------------------------------------
	// Вводим данные
	readData();

	//-------------------------------------------------------------------------
	// Код для проверки всех пересечений всех сторон многоугольника. Была теория о том, что максимум находится на одной из таких точек
	vector<pair<double, point>> mas;
	cout << fixed << setprecision(5);
	travelPolygonEdges(in.p, [&] (int a1, int a2) {
		travelPolygonEdges(in.p, [&] (int b1, int b2) {
			if (a1 != b1 && a1 != b2 && a2 != b1 && a2 != b2) {
				double t1, t2;
				point s;
				if (intersectLines(
				{in.p[a1], in.p[a2]}, 
				{in.p[b1], in.p[b2]}, 
					t1, t2, s
				))
				if (pointInPolygon(s))
					mas.push_back(pair<double, point>(calcRadialArea(s), s));
			}
		});
	});

	sort(mas.begin(), mas.end(), [] (auto& a, auto& b) -> bool {
		return a.first > b.first;
	});

	cout << fixed << setprecision(3) << mas[0].first;
}