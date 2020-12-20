/*

	с

*/

#include "stdc++.h"
using namespace std;

typedef int64_t bint;

const bint inf = -1;

struct col { bint len, h; };

vector<col> cols;
vector<bint> area;
vector<bint> len;

void precalc_area(void) {
	area.resize(cols.size());
	len.resize(cols.size());

	bint last = cols.size()-1;
	if (cols[last].len == inf) area[last] = inf;
	else area[last] = cols[last].len * cols[last].h
	len[last] = cols[last].len;
	for (int i = last-1; i >= 0; --i) {
		if (area[i+1] == inf) area[i] = inf;
		else area[i] = area[i+1] + cols[i].len * cols[i].h;
		if (len[i+1] == inf) len[i] = inf;
		else len[i] = len[i+1] + cols[i].len;
	}
}

void find_h(bint h) {
	bint a = 0, b = cols.size() - 1;
	if (h > cols[b].h) return cols.size();
	if (h < cols[a].h) return 0;
	while (true) {
		if (b - a == 1) if (h > cols[a].h) return a; else return b;
		if (h ==)
	}
}

int main() {
	bint n, q;
	cin >> n >> q;
	bint lastpos = 0, lasth = 0, t, k;
	char c;
	for (int i = 0; i < n; ++i) {
		cin >> c >> t >> k;
		cols.push_back({lastpos-t, lasth});
		if (c == '+') lasth += k;
		else lasth -= k;
		lastpos = t;
	}
	cols.push_back({inf, lasth});

	sort(cols.begin(), cols.end(), [] (auto& a, auto& b) -> bool {
		return a.h < b.h;
	});

	for (int i = 0; i < q; ++i) {
		bint question;
		cin >> question;
		print_time(question);
	}
}