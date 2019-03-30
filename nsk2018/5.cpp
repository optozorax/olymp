/** Данная задача не решена!!! */

#define _USE_MATH_DEFINES

#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <functional>
#include <sstream>
#include <string>
#include <bitset>
#include <numeric>
#include <algorithm>
#include <iomanip>
#include <fstream>
#include "5_common.h"

using namespace std;

void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

prob_line count_prob_is_alive_and_play(bint n, bint k, const prob_table& last, bint aliveN) {
	prob_line result(last.size(), fraction(0));
	if (last[n][k].a == 0) return result;

	result[k] = fraction(0);
	result[n] = fraction(1);
	if (aliveN == 4 && n != result.size()-1) {
		result.back() = fraction(1);
	} else {
		for (int i = 0; i < result.size(); ++i) {
			if (i != k && i != n) {
				if (last[n][i].a != 0) {
					for (int j = 0; j < i; ++j) {
						if (j != k && j != n) {
							result[i] += last[i][j];
						}
					}
					result[i] *= last[n][i];
					result[i] /= aliveN-3;
				}
			}
		}
	}

	return result;
}

prob_line count_prob_is_alive(bint n, const prob_table& last, bint aliveN) {
	prob_line result(last.size(), fraction(0));
	if (last[n][n].a == 0) return result;

	for (int i = 0; i < n; ++i)
		result += count_prob_is_alive_and_play(n, i, last, aliveN);

	if (n != 0)
		result /= result.back();
	return result;
}

prob_table make_next(const prob_table& last, bint n, bint aliveN) {
	prob_table result;
	bint size = pow(2.0, n);
	for (int i = 0; i < size; ++i)
		result.push_back(count_prob_is_alive(i, last, aliveN));
	return result;
}

prob_table make_first(int n) {
	bint size = pow(2.0, n);
	return prob_table(size, prob_line(size, fraction(1)));
}

int main() {
	bint n = 3, k = 7;
	//cin >> n >> k;

	vector<prob_table> tables;
	tables.push_back(make_first(n));

	ofstream fout("5_tables_fast.txt");
	for (int i = 1; i < n; ++i) {
		cout << i << endl;
		tables.push_back(make_next(tables.back(), n, pow(2.0, n-i+1)));
		fout << tables.back() << endl << endl;
	}
	fout.close();
}