#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <map>
#include <fstream>

using namespace std;

typedef int64_t bint;

struct robots {
	bint x1, y1;
	bint x2, y2;
};

bint gettime(bint n, bint x, bint y) {
	return (n - 1)*(x + y) + x;
}

bint calctime(bint i, bint n, const robots& r) {
	return max(gettime(i, r.x1, r.y1), gettime(n-i, r.x2, r.y2));
}

bool isGrow(bint i, bint n, const robots& r) {
	bint time = calctime(i, n, r);
	if (i == n) {
		bint time1 = calctime(i - 1, n, r);
		return time1 < time;
	} else {
		bint time1 = calctime(i + 1, n, r);
		return time1 > time;
	}
}

bool isMin(bint i, bint n, const robots& r) {
	if (i == 0) {
		return isGrow(i, n, r);
	} else if(i == n) {
		return !isGrow(i, n, r);
	} else {
		return !isGrow(i-1, n, r) && isGrow(i, n, r);
	}
}

bint findMinTime(bint n, const robots& r) {
	bint a = 0, b = n;
	while (true) {
		if (isMin(a, n, r)) return calctime(a, n, r);
		if (isMin(b, n, r)) return calctime(b, n, r);
		bint c = (a + b) / 2;
		if (!isGrow(c, n, r))
			a = c;
		else
			b = c;
	}
}

int main() {
	robots r; bint n;
	cin >> n >> r.x1 >> r.y1 >> r.x2 >> r.y2;
	cout << findMinTime(n, r);
	return 0;
}