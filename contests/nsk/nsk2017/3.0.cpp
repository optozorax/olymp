// Максимально говнокодерский подход к решению задачи. Тут используется гоуту, и программисткая лень в написании оптамиальных алгоритмов. 3.1 является улучшенной версией этого.

#include <iostream>
#include <vector>
#include <algorithm>
#include <list>
#include <sstream>
#include <iterator>

using namespace std;

void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

typedef int64_t bint;

struct Rect
{
	Rect(int x, int y, int sx, int sy, int count) : x(x), y(y), sx(sx), sy(sy), count(count) {}

	int x, y;
	int sx, sy;

	int count;

	bool isInsideX(int xtest) { return x < xtest && xtest < x+sx; }
	bool isInsideY(int ytest) { return y < ytest && ytest < y+sy; }

	bool operator==(const Rect& b) {
		return x == b.x && y == b.y && sx == b.sx && sy == b.sy;
	}
};

bool isIntersect(const Rect& a, const Rect& b) {
	return !(a.x+a.sx<b.x || b.x+b.sx<a.x || a.y+a.sy<b.y || b.y+b.sy<a.y);
}

template<class T>
void erase_duplicates(T vec) {
	sort(vec.begin(), vec.end());
	vec.erase(unique(vec.begin(), vec.end()), vec.end());
}

list<Rect> intersectRects(const list<Rect>& rects) {
	list<Rect> result;
	result.insert(result.end(), rects.begin(), rects.end());

	// Разрезать прямоугольник линией по оси Х
	auto cut_by_x = [] (const Rect& a, int x) -> list<Rect> {
		Rect a1(a.x, a.y, x-a.x, a.sy, a.count);
		Rect a2(x, a.y, a.x+a.sx-x, a.sy, a.count);
		return {a1, a2};
	};

	// Разрезать прямоугольник линией по оси У
	auto cut_by_y = [] (const Rect& a, int y) -> list<Rect> {
		Rect a1(a.x, a.y, a.sx, y-a.y, a.count);
		Rect a2(a.x, y, a.sx, a.y+a.sy-y, a.count);
		return {a1, a2};
	};


	// Формируем массив всех возможных обрезающих линий
	vector<int> xmas, ymas;
	for (auto i = rects.begin(); i != rects.end(); ++i) {
		xmas.push_back(i->x);
		xmas.push_back(i->x+i->sx);
		ymas.push_back(i->y);
		ymas.push_back(i->y+i->sy);
	}

	// Удаляем одинаковые разрезающие линии
	erase_duplicates(xmas);
	erase_duplicates(ymas);

	// Разрезаем все возможные прямоугольники всеми возможными линиями
	start:
	for (auto i = result.begin(); i != result.end(); ++i) {
		for (auto& j : xmas) {
			if (i->isInsideX(j)) {
				auto newRes = cut_by_x(*i, j);
				result.erase(i);
				result.insert(result.end(), newRes.begin(), newRes.end());
				goto start;
			}	
		}
		for (auto& j : ymas) {
			if (i->isInsideY(j)) {
				auto newRes = cut_by_y(*i, j);
				result.erase(i);
				result.insert(result.end(), newRes.begin(), newRes.end());
				goto start;
			}
		}
	}

	// Объединям одинаковые прямоугольники
	start2:
	for (auto i = result.begin(); i != result.end(); ++i) {
		for (auto j = next(i); j != result.end(); ++j) {
			if (*i == *j) {
				i->count += j->count;
				result.erase(j);
				goto start2;
			}
		}
	}

	return result;
}

int main() {
	//test("6 4 2\n0 0 4 3\n2 1 4 3\n");
	//test("10 10 10\n0 4 2 1\n6 3 4 1\n0 9 9 1\n9 6 1 3\n7 4 2 4\n0 6 7 1\n9 1 1 5\n4 6 1 4\n1 8 3 2\n2 1 7 5\n");
	//test("100 100 100\n86 81 6 13\n40 95 23 5\n88 42 3 53\n60 57 38 35\n86 15 2 73\n84 33 11 19\n42 80 26 16\n14 30 80 11\n17 11 78 74\n74 21 19 72\n9 55 69 30\n2 62 36 38\n74 46 24 23\n4 1 54 53\n18 11 20 54\n51 60 41 24\n49 2 38 49\n64 10 3 59\n43 20 44 23\n57 98 11 2\n44 45 2 38\n41 51 7 8\n56 39 29 39\n97 12 2 44\n72 16 5 84\n88 87 4 9\n39 24 60 64\n25 27 72 15\n86 44 3 4\n57 15 8 84\n92 57 8 11\n46 43 37 18\n81 26 11 31\n82 25 17 43\n11 39 26 11\n54 26 42 22\n44 43 44 31\n96 32 4 39\n99 17 1 26\n93 7 2 90\n57 66 2 34\n71 54 11 28\n52 58 46 8\n87 1 6 66\n67 64 27 2\n66 14 12 43\n33 99 29 1\n19 60 52 2\n23 32 46 60\n23 90 45 10\n6 16 1 32\n37 74 44 26\n73 40 17 18\n26 13 22 38\n31 53 46 34\n12 71 37 27\n95 90 5 6\n41 55 26 35\n7 91 91 5\n14 97 10 2\n30 43 65 46\n97 33 3 7\n2 16 56 59\n24 85 60 6\n75 8 11 44\n80 13 12 16\n97 85 1 6\n16 79 25 8\n42 6 25 65\n79 62 20 24\n13 84 46 5\n99 73 1 6\n13 62 65 3\n54 63 37 7\n87 73 11 4\n11 41 84 14\n87 21 4 15\n43 9 32 13\n13 80 30 20\n60 31 8 17\n32 0 16 54\n4 22 77 62\n28 25 59 5\n80 81 10 14\n90 29 4 56\n46 35 43 3\n8 61 85 29\n29 82 40 11\n55 57 37 5\n19 40 15 40\n42 48 52 10\n27 48 25 17\n31 62 4 12\n93 41 6 58\n22 18 36 36\n30 5 35 44\n69 24 16 17\n1 38 52 32\n11 71 32 29\n45 59 42 25\n");

	list<Rect> rects;

	bint w, h, k;
	cin >> w >> h >> k;
	for (int i = 0; i < k; ++i) {
		bint x, y, sx, sy;
		cin >> x >> y >> sx >> sy;
		Rect newRect(x, y, sx, sy, 1);
		list<Rect> unionRects;
		unionRects.push_back(newRect);

		auto j = rects.begin();
		while (j != rects.end()) {
			bool ispp = true;
			while (j != rects.end() && isIntersect(*j, newRect)) {
				unionRects.push_back(*j);
				j = rects.erase(j);
				ispp = false;
			}
			if (ispp) j++;
		}

		auto newRes = intersectRects(unionRects);
		rects.insert(rects.end(), newRes.begin(), newRes.end());
	}

	vector<int> result(k+1);
	result[0] = w * h;
	for (auto& i : rects) {
		result[0] -= i.sx * i.sy;
		result[i.count] += i.sx * i.sy;
	}

	//cout << "Size: " << rects.size() << endl;

	for (int i = 0; i < result.size(); ++i) {
		cout << i << " " << result[i] << endl;
	}

	//system("pause");
}