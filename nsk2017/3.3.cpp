// Это улучшенная версия 3.2 - моя последняя надежда на быстрое решение
// Видите ли, тут тоже не хватает времени на 18 тесте! Да пошли вы в жопу!

#include <iostream>
#include <vector>
#include <algorithm>
#include <list>
#include <sstream>
#include <iterator>
#include <fstream>

using namespace std;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

typedef int64_t bint;

struct Rect
{
	int x, y;
	int sx, sy;
};

void erase_duplicates(vector<int>& vec) {
	static vector<int> newvec;
	newvec.clear();
	sort(vec.begin(), vec.end());
	for (auto i = vec.begin(); i != vec.end(); ++i) {
		newvec.push_back(*i);
		while (i+1 != vec.end() && i[1] == i[0])
			i++;
	}
	swap(vec, newvec);
}

inline int findInCoords(const std::vector<int>& coords, int pos) {
	int a = 0, b = coords.size();
	while (true) {
		int c = (a+b)/2;
		if (coords[c] > pos)
			b = c;
		else if (coords[c] == pos)
			return c;
		else
			a = c;
	}
}

int main() {
	//test("6 4 2\n0 0 4 3\n2 1 4 3\n");
	//test("10 10 10\n0 4 2 1\n6 3 4 1\n0 9 9 1\n9 6 1 3\n7 4 2 4\n0 6 7 1\n9 1 1 5\n4 6 1 4\n1 8 3 2\n2 1 7 5\n");
	//test("100 100 100\n86 81 6 13\n40 95 23 5\n88 42 3 53\n60 57 38 35\n86 15 2 73\n84 33 11 19\n42 80 26 16\n14 30 80 11\n17 11 78 74\n74 21 19 72\n9 55 69 30\n2 62 36 38\n74 46 24 23\n4 1 54 53\n18 11 20 54\n51 60 41 24\n49 2 38 49\n64 10 3 59\n43 20 44 23\n57 98 11 2\n44 45 2 38\n41 51 7 8\n56 39 29 39\n97 12 2 44\n72 16 5 84\n88 87 4 9\n39 24 60 64\n25 27 72 15\n86 44 3 4\n57 15 8 84\n92 57 8 11\n46 43 37 18\n81 26 11 31\n82 25 17 43\n11 39 26 11\n54 26 42 22\n44 43 44 31\n96 32 4 39\n99 17 1 26\n93 7 2 90\n57 66 2 34\n71 54 11 28\n52 58 46 8\n87 1 6 66\n67 64 27 2\n66 14 12 43\n33 99 29 1\n19 60 52 2\n23 32 46 60\n23 90 45 10\n6 16 1 32\n37 74 44 26\n73 40 17 18\n26 13 22 38\n31 53 46 34\n12 71 37 27\n95 90 5 6\n41 55 26 35\n7 91 91 5\n14 97 10 2\n30 43 65 46\n97 33 3 7\n2 16 56 59\n24 85 60 6\n75 8 11 44\n80 13 12 16\n97 85 1 6\n16 79 25 8\n42 6 25 65\n79 62 20 24\n13 84 46 5\n99 73 1 6\n13 62 65 3\n54 63 37 7\n87 73 11 4\n11 41 84 14\n87 21 4 15\n43 9 32 13\n13 80 30 20\n60 31 8 17\n32 0 16 54\n4 22 77 62\n28 25 59 5\n80 81 10 14\n90 29 4 56\n46 35 43 3\n8 61 85 29\n29 82 40 11\n55 57 37 5\n19 40 15 40\n42 48 52 10\n27 48 25 17\n31 62 4 12\n93 41 6 58\n22 18 36 36\n30 5 35 44\n69 24 16 17\n1 38 52 32\n11 71 32 29\n45 59 42 25\n");
	/*std::ifstream t("input18.txt");
	std::string str((std::istreambuf_iterator<char>(t)),
	std::istreambuf_iterator<char>());
	test(str);*/

	bint w, h, k;
	cin >> w >> h >> k;
	vector<Rect> rects(k);
	vector<int> ymas;
	ymas.reserve(k*2);
	for (int i = 0; i < k; ++i) {
		cin >> rects[i].x >> rects[i].y >> rects[i].sx >> rects[i].sy;
		ymas.push_back(rects[i].y); ymas.push_back(rects[i].y + rects[i].sy);
	}
	erase_duplicates(ymas);

	vector<vector<pair<int, int>>> lines(ymas.size());
	for (auto& r : rects) {
		int x = r.x, y = r.y;
		int x1 = r.x+r.sx, y1 = r.y+r.sy;
		int _y = findInCoords(ymas, y);
		int _y1 = findInCoords(ymas, y1);

		for (int i = _y; i < _y1; ++i)
			lines[i].push_back({x, x1});
	}

	vector<bint> result(k+1);
	result[0] = w * h;

	vector<int> xmas;
	for (int i = 0; i < lines.size()-1; ++i) {
		xmas.clear();
		for (auto& j : lines[i]) {
			xmas.push_back(j.first);
			xmas.push_back(j.second);
		}
		erase_duplicates(xmas);

		vector<int16_t> matrix(xmas.size(), 0);
		for (int j = 0; j < lines[i].size(); ++j) {
			int x = lines[i][j].first;
			int x1 = lines[i][j].second;
			int _x = findInCoords(xmas, x);
			int _x1 = findInCoords(xmas, x1);

			auto k = matrix.begin() + _x;
			auto end = matrix.begin() + _x1;
			for (; k != end; ++k) {
				++(*k);
			}
		}

		bint ysize = ymas[i+1]-ymas[i];
		for (int j = 0; j < xmas.size()-1; ++j) {
			bint xsize = xmas[j+1]-xmas[j];
			bint s = ysize * xsize;

			result[0] -= s;
			result[matrix[j]] += s;
		}
	}

	for (int i = 0; i < result.size(); ++i)
		cout << i << " " << result[i] << endl;

	//system("pause");
}