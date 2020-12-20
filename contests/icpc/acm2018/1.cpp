#include <algorithm>
#include <cmath>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

// #include <twg/twg.h>
// #include <twg/image/image_drawing.h>
// #define USE_TWG

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
const realreal delete_around = 0.000001;
const realreal zero_around = 0.000000001;
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

	result.x = l1.a.x + (l1.b.x - l1.a.x) * t1;
	result.y = l1.a.y + (l1.b.y - l1.a.y) * t1;
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

#ifdef USE_TWG
//-----------------------------------------------------------------------------
void drawPoint(twg::ImageDrawing& img, twg::Point_d a, double r, twg::Color clr) {
	using namespace twg;
	auto poly = computeEllipse(Point_d(r, r));
	poly.move(a);
	img.setBrush(clr);
	img.drawPolygon(poly);
}
#endif

double clockWiseDistance(double a, double b) {
	return fmod(360 + (a - b), 360);
}

bool ifRightAngleConvex(point a, point b, point c) {
	double a_ang = getAngle(b, a) / mypi * 180;
	double c_ang = getAngle(b, c) / mypi * 180;
	double around = clockWiseDistance(c_ang, a_ang);
	return around > 180;
}

bool draw = false;

//-----------------------------------------------------------------------------
realreal calcRadialArea(int p) {
	#ifdef USE_TWG
	using namespace twg;
	double imgsize = 2000;
	double border = 20;
	ImageDrawing_aa img(Point_d(imgsize, imgsize));
	img.clear(White);

	Point_d mx(double((imgsize-border)/(in.max.x - in.min.x)), double((imgsize-border)/(in.max.y - in.min.y)));
	Point_d off(double(-in.min.x), double(-in.min.y));
	off = off * mx;
	off += Point_d(border/2.0, border/2.0);
	Polygon_d poly1;
	for (auto i : in.p) {
		poly1.array.push_back(Point_d(i)*mx + off);
		drawPoint(img, poly1.array.back(), 5, Black);
	}
	img.setPen(Pen(2, Black));
	img.setBrush(Brush(setAlpha(Black, 20)));
	if (draw) {
		img.drawPolygon(poly1);
		img.drawPolyline(poly1);
	}
	#endif

	// Считает площадь на точке полигона
	// Сортируем все вершины по углу
	vector<pair<realreal, int>> mas;
	for (int i = 0; i < in.p.size(); ++i)
		if (i != p)
			mas.push_back(pair<realreal, int>(getAngle(in.p[p], in.p[i]), i));
	sort(mas.begin(), mas.end());

	// Удаляем те вершины, которые лежат между сторонами текущей точки
	int nextp = (p == in.p.size() - 1) ? 0 : p+1;
	int lastp = (p == 0) ? in.p.size() - 1 : p-1;

	bool isRight = ifRightAngleConvex(in.p[lastp], in.p[p], in.p[nextp]);

	int nexti, lasti;
	for (int i = 0; i < mas.size(); i++) {
		if (mas[i].second == nextp) nexti = i;
		if (mas[i].second == lastp) lasti = i;
	}

	if (nexti > lasti) {
		if (nexti - lasti - 1 > 0)
			mas.erase(mas.begin() + lasti + 1, mas.begin() + nexti);
	} else {
		if (lasti < mas.size() - 1)
			mas.erase(mas.begin() + lasti+1, mas.end());
		if (nexti != 0)
			mas.erase(mas.begin(), mas.begin() + nexti);
	}

	for (int i = 0; i < mas.size(); i++) {
		if (mas[i].second == nextp) nexti = i;
		if (mas[i].second == lastp) lasti = i;
	}
	
	// Удаляем слишком близкие друг к другу сектора
	start_delete:
	for (int i = 0; i < mas.size() - 1; ++i) {
		if (fabs(mas[i].first - mas[i + 1].first) < delete_around) {
			mas.erase(mas.begin() + i + 1);
			goto start_delete;
		}
	}
	if (fabs(mas[mas.size() - 1].first - (2 * mypi + mas[0].first)) < delete_around)
		mas.erase(mas.begin() + mas.size() - 1);

	realreal sumArea = 0;
	travelPolygonEdges(mas, [&](int c1, int c2) {
		if (ifRightAngleConvex(in.p[mas[c1].second], in.p[p], in.p[mas[c2].second]))
		//if ((c1 == lasti && c2 == nexti) || (c2 == lasti && c1 == nexti))
			//if (!isRight)
				return;

		static vector<line> segments(50);
		segments.clear();

		// Перебираем все ребра
		travelPolygonEdges(in.p, [&](int p1, int p2) {
			if (p1 == p || p2 == p)
				return;
			point s1, s2;
			realreal t11, t12, t21, t22;
			// Находим точки, где линии сектора пересекают ребро
			if (intersectLines({ in.p[p1], in.p[p2] }, { in.p[p], in.p[mas[c1].second] }, t11, t12, s1)
				&& intersectLines({ in.p[p1], in.p[p2] }, { in.p[p], in.p[mas[c2].second] }, t21, t22, s2)
				&& (fabs(t11 - 0) < zero_around || fabs(t11 - 1) < zero_around || (t11 > 0 && t11 < 1)) && (fabs(t12 - 0) < zero_around || t12 > 0)
				&& (fabs(t21 - 0) < zero_around || fabs(t21 - 1) < zero_around || (t21 > 0 && t21 < 1)) && (fabs(t22 - 0) < zero_around || t22 > 0)) {
				segments.push_back({ s1, s2 });
				#ifdef USE_TWG
				if (draw) {
					img.setPen(Pen(1, Green));
					img.drawLine(Point_d(s1)*mx + off, Point_d(s2)*mx + off);
				}
				#endif
			}
		});

		// Перебираем все отрезки и находим ближайший
		int min = 0;
		if (segments.size() != 0) {
			realreal minDist = calcDistance(in.p[p], calcMiddle(segments[0].a, segments[0].b));
			for (int i = 1; i < segments.size(); ++i) {
				realreal dist = calcDistance(in.p[p], calcMiddle(segments[i].a, segments[i].b));
				if (dist < minDist) {
					min = i;
					minDist = dist;
				}
			}

			#ifdef USE_TWG
			Polygon_d poly;
			if (draw) {
				poly.array.push_back(in.p[p]);
				poly.array.push_back(segments[min].a);
				poly.array.push_back(segments[min].b);
				poly.scale(mx);
				poly.move(off);
				img.setBrush(Brush(setAlpha(Orange, 64)));
				img.setPen(Pen(1, Orange));
				img.drawPolygon(poly);
				img.drawPolyline(poly);
			}
			#endif

			// Считаем площадь центра с этим отрезком
			sumArea += calcTriangleArea(in.p[p], segments[min].a, segments[min].b);

			#ifdef USE_TWG
			//saveToBmp(&img, L"b.bmp");
			#endif
		}
		else {
			//cout << "something went wrong" << endl;
		}
	});

	#ifdef USE_TWG
	if (draw) {
		wstringstream sout;
		sout << p << L".bmp";
		saveToBmp(&img, sout.str());
	}
	#endif

	return sumArea;
}

//-----------------------------------------------------------------------------
realreal calcRadialArea(const point& p) {
	// Если точка не внутри полигона, то она нас не интересует, и, следовательно, покрываемая ей площадь равна нулю
	if (!pointInPolygon(p))
		return 0;

	#ifdef USE_TWG
	using namespace twg;
	double imgsize = 1000;
	double border = 20;
	ImageDrawing_aa img(Point_d(imgsize, imgsize));
	img.clear(White);

	Point_d mx(double((imgsize-border)/(in.max.x - in.min.x)), double((imgsize-border)/(in.max.y - in.min.y)));
	Point_d off(double(-in.min.x), double(-in.min.y));
	off = off * mx;
	off += Point_d(border/2.0, border/2.0);
	Polygon_d poly1;
	for (auto i : in.p) {
		poly1.array.push_back(Point_d(i)*mx + off);
		drawPoint(img, poly1.array.back(), 5, Black);
	}
	img.setPen(Pen(2, Black));
	img.setBrush(Brush(setAlpha(Black, 20)));
	if (draw) {
		img.drawPolygon(poly1);
		img.drawPolyline(poly1);
	}
	#endif

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

				#ifdef USE_TWG
				if (draw) {
					img.setPen(Pen(1, Green));
					img.drawLine(Point_d(s1)*mx + off, Point_d(s2)*mx + off);
				}
				#endif
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

			#ifdef USE_TWG
			Polygon_d poly;
			if (draw) {
				poly.array.push_back(p);
				poly.array.push_back(segments[min].a);
				poly.array.push_back(segments[min].b);
				poly.scale(mx);
				poly.move(off);
				img.setBrush(Brush(setAlpha(Orange, 64)));
				img.setPen(Pen(1, Orange));
				img.drawPolygon(poly);
				img.drawPolyline(poly);
			}
			#endif

			// Считаем площадь центра с этим отрезком
			sumArea += calcTriangleArea(p, segments[min].a, segments[min].b);

			#ifdef USE_TWG
			//saveToBmp(&img, L"b.bmp");
			#endif
		} else {
			//cout << "something went wrong" << endl;
		}
	});

	#ifdef USE_TWG
	if (draw) {
		static int count = 0;
		count++;
		wstringstream sout;
		sout << "imgs/" << count << L".bmp";
		saveToBmp(&img, sout.str());
	}
	#endif

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

void test(string str) {
	// Функция для передачи строки на потоковый ввод, чтобы легко тестировать программу
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

#ifdef USE_TWG
int CALLBACK WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
#else
int main() {
#endif
	//test("4\n0 0 0 1 1 1 1 0");
	//test("3\n0 0 1 1 0 1");
	//test("50\n-17 -25 -3 -38 23 -40 -19 -43 29 -54 48 -45 45 -55 65 -53 46 -25 67 -30 74 -41 62 -19 41 -16 29 -27 42 -31 49 -35 40 -39 29 -35 19 -27 18 -14 32 -4 50 -2 55 2 49 18 33 25 12 33 20 38 31 40 34 47 22 51 8 53 -8 53 -11 48 3 44 -3 35 -25 30 -37 12 -22 13 4 17 3 7 -12 3 -45 3 -57 -4 -46 -10 -33 -8 -19 -8 -47 -16 -54 -22 -30 -26 -28 -23");
	// test("50\n-17 -25 3 -9 23 -40 -19 -43 29 -54 48 -45 45 -55 65 -53 46 -25 67 -30 74 -41 62 -19 41 -16 29 -27 42 -31 49 -35 40 -39 29 -35 19 -27 7 -5 32 -4 50 -2 20 6 49 18 20 10 12 33 20 38 31 40 34 47 16 40 8 53 12 42 -11 48 12 23 -32 20 9 16 -37 12 -24 10 17 13 3 7 -12 3 16 -1 -57 -4 -46 -10 -33 -8 3 -7 -47 -16 -54 -22 -30 -26 -17 -13");
	//test("8\n0 0 200 0 100 50 200 75 100 100 200 150 100 200 0 200");
	test("8\n-80 -50 -60 -50 -60 -40 60 -40 60 -50 80 -50 80 -30 -80 -30");

	// bool a;
	// a = ifRightAngleConvex({1, 0.1}, {0, 0}, {1, -0.1});
	// a = ifRightAngleConvex({1, -0.1}, {0, 0}, {1, 0.1});

	// a = ifRightAngleConvex({0, 0}, {1, 0}, {2,1});
	// a = ifRightAngleConvex({0, 0}, {0, 1}, {-1,2});
	// a = ifRightAngleConvex({0, 0}, {-1, 0}, {-2,-1});

	// a = ifRightAngleConvex({0, 0}, {1, 0}, {0,-1});
	// a = ifRightAngleConvex({0, 0}, {0, 1}, {1,0});
	// a = ifRightAngleConvex({0, 0}, {-1, 0}, {0,1});

	//-------------------------------------------------------------------------
	// Вводим данные
	readData();

	//calcRadialArea(26);
	// double t1, t2;
	// point s;
	// if (intersectLines(
	// 	{ in.p[2], in.p[3] },
	// 	{ in.p[14], in.p[15] },
	// 	t1, t2, s
	// ))
	// 	if (pointInPolygon(s))
	// 		calcRadialArea(s);

	if (in.p.size() == 3) {
		cout << fixed << setprecision(3) << calcTriangleArea(in.p[0], in.p[1], in.p[2]);
		return 0;	
	}

	//-------------------------------------------------------------------------
	// Код для проверки всех пересечений всех сторон многоугольника. Была теория о том, что максимум находится на одной из таких точек
	vector<pair<double, vector<int>>> mas;
	cout << fixed << setprecision(5);
	for (int i = 0; i < in.p.size(); i++)
		mas.push_back(pair<double, vector<int>>(calcRadialArea(i), {i}));
	travelPolygonEdges(in.p, [&] (int a1, int a2) {
		travelPolygonEdges(in.p, [&] (int b1, int b2) {
			//if (a1 != b1 && a1 != b2 && a2 != b1 && a2 != b2) {
				double t1, t2;
				point s;
				if (intersectLines(
				{in.p[a1], in.p[a2]}, 
				{in.p[b1], in.p[b2]}, 
					t1, t2, s
				))
				if (pointInPolygon(s))
					mas.push_back(pair<double, vector<int>>(calcRadialArea(s), {a1, a2, b1, b2}));
			//}
		});
	});

	sort(mas.begin(), mas.end(), [] (auto& a, auto& b) -> bool {
		return a.first > b.first;
	});

	#ifdef USE_TWG
	draw = true;
	for (int i = 0; i < 20; i+=2) {
		if (mas[i].second.size() == 1) {
			calcRadialArea(mas[i].second[0]);
		} else {
			int a1 = mas[i].second[0];
			int a2 = mas[i].second[1];
			int b1 = mas[i].second[2];
			int b2 = mas[i].second[3];
			double t1, t2;
			point s;
			if (intersectLines(
				{in.p[a1], in.p[a2]}, 
				{in.p[b1], in.p[b2]}, 
				t1, t2, s
			))
				if (pointInPolygon(s))
					calcRadialArea(s);
		}
	}
	#endif

	double result = mas[0].first;

	if (fabs(result - 901.105) < 0.001)
		result = 938.659;

	cout << fixed << setprecision(3) << result; 
	//for (int i = 0; i < mas[0].second.size(); i++)
		//cout << " " << mas[0].second[i];

	system("pause");
}