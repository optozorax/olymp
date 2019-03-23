// Просто полный перебор, запрогранный на олимпиаде, можно использовать для проверки алгоритма с динамическим программированием

#include <iostream>
#include <vector>
#include <cmath>
#include <iomanip>
#include <limits>

#define sqr(a) ((a)*(a))

using namespace std;

typedef int64_t bint;

struct planet
{
	bint x, y, z;
	bool visited;

	vector<pair<planet*, double>> next;
};

double mymin = std::numeric_limits<double>::max(), sum = 0;
int deep = 1;
vector<planet> ps;

void trace(planet* last) {
	for (int i = 0; i < last->next.size(); i++) {
		if (!last->next[i].first->visited) {
			last->next[i].first->visited = true;
			sum += last->next[i].second;
			deep++;
			if (deep == ps.size()) {
				if (sum < mymin)
					mymin = sum;
			}
			else {
				trace(last->next[i].first);
			}
			deep--;
			sum -= last->next[i].second;
			last->next[i].first->visited = false;
		}
	}
}

int main() {
	bint n;
	/*cin >> n;
	for (int i = 0; i < n; i++) {
		planet p;
		cin >> p.x >> p.y >> p.z;
		p.visited = false;
		ps.push_back(p);
	}*/

	/*n = 15;
	ps.push_back({ 0, 0, 0 });
	ps.push_back({ 5, 0, 0 });
	ps.push_back({ 5, 1, 0 });
	ps.push_back({ 5, 2, 0 });
	ps.push_back({ 2, 3, 0 });
	ps.push_back({ 5, 1, 5 });
	ps.push_back({ 5, 0, 8 });
	ps.push_back({ 0, 1, 5 });
	ps.push_back({ 5, 1, 4 });
	ps.push_back({ 4, 0, 1 });
	ps.push_back({ 3, 2, 0 });
	ps.push_back({ 5, 0, 5 });*/

	n = 13;
	ps.push_back({ 0, 0, 0 });
	ps.push_back({ 5, 0, 0 });
	ps.push_back({ 5, 1, 0 });
	ps.push_back({ 5, 2, 0 });
	ps.push_back({ 2, 3, 0 });
	ps.push_back({ 5, 1, 5 });
	ps.push_back({ 5, 0, 8 });
	ps.push_back({ 0, 1, 5 });
	ps.push_back({ 4, 0, 1 });
	ps.push_back({ 3, 2, 0 });
	ps.push_back({ 5, 0, 5 });
	ps.push_back({20, 30, 50});
	ps.push_back({20, 0, 100});

	for (int i = 0; i < ps.size(); i++) {
		for (int j = 0; j < ps.size(); j++) {
			if (i != j) {
				auto& a = ps[i];
				auto& b = ps[j];
				ps[i].next.push_back({&ps[j], sqrt(sqr(a.x-b.x) + sqr(a.y-b.y) + sqr(a.z-b.z))});
			}
		}
	}

	ps[0].visited = true;
	trace(&ps[0]);

	cout << setprecision(3) << fixed;
	cout << mymin;

	system("pause");
}