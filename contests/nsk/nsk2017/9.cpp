#include <iostream>
#include <cmath>
#include <algorithm>

using namespace std;

typedef int64_t bint;

struct rect
{
	bint sx, sy;
};

bint diagSqr(rect r) {
	return r.sx*r.sx + r.sy*r.sy;
}

bool isCanBeInside(rect r, bint R) {
	return diagSqr(r) <= 4*R*R;
}

bool isCanBeOutside(rect r, bint R) {
	return r.sx >= 2*R && r.sy >= 2*R;
}

bool isCanBeBorder(rect in, rect out, bint R) {
	return isCanBeInside(in, R) && isCanBeOutside(out, R);
}

bool isCorrect(rect r) {
	return r.sx > 0 && r.sy > 0;
}

rect startOut(bint R) {
	return {R*2, R*2};
}

rect startIn(bint R) {
	return {bint(sqrt(2)*R), bint(sqrt(2)*R)};
}

bint getArea(rect in, rect out) {
	return out.sx * out.sy - in.sx * in.sy;
}

int main() {
	bint R = 100000;
	cin >> R;

	rect in = startIn(R);
	rect out = startOut(R);
	bint m = -1;
	const bint count = 5;
	for (bint x1 = -count; x1 <= count; ++x1) {
		for (bint y1 = -count; y1 <= count; ++y1) {
			for (bint x2 = -count; x2 <= count; ++x2) {
				for (bint y2 = -count; y2 <= count; ++y2) {
					rect in1 = in, out1 = out;
					in1.sx += x1; in1.sy += y1;
					out1.sx += x2; out1.sy += y2;

					if (isCorrect(in1) && isCorrect(out1) && isCanBeBorder(in1, out1, R)) {
						if (m == -1) {
							m = getArea(in1, out1);
						} else {
							m = min(getArea(in1, out1), m);
						}
					}
				}	
			}			
		}	
	}

	cout << m;

	//cout << endl; system("pause");
}