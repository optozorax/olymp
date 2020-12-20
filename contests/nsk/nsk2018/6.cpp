#define _USE_MATH_DEFINES

#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <functional>
#include <sstream>
#include <string>
#include <bitset>
using namespace std;
typedef int64_t bint;

vector<pair<bint, bint>> games = {{0, 1}, {0, 2}, {0, 3}, {1, 2}, {1, 3}, {2, 3}};
bitset<6> played;
vector<bint> scores = {0, 0, 0, 0};
bitset<4> can_win;

vector<pair<bint, bint>> type = {{3, 0}, {0, 3}, {1, 1}};

bint findGame(bint a, bint b) {
	for (int i = 0; i < games.size(); ++i) {
		if (games[i].first == a && games[i].second == b)
			return i;
	}
	for (int i = 0; i < games.size(); ++i) {
		if (games[i].first == b && games[i].second == a)
			return i;
	}
}

void recur() {
	if (played.all()) {
		for (int i = 0; i < scores.size(); ++i) {
			for (int j = 0; j < scores.size(); ++j) {
				if (i != j) {
					if (scores[j] >= scores[i])
						goto next_iter;
				}
			}
			can_win[i] = true;
			next_iter:;
		}
	} else {
		for (int i = 0; i < games.size(); ++i) if (!played[i]) {
			played[i] = true;
			for (auto& j : type) {
				scores[games[i].first] += j.first;
				scores[games[i].second] += j.second;
				recur();
				scores[games[i].first] -= j.first;
				scores[games[i].second] -= j.second;
			}
			played[i] = false;
		}
	}
}

void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

int main() {
	//test("3\n1 2 10 0\n3 2 2 0\n4 2 1 0\n");

	bint n;
	cin >> n;
	for (int i = 0; i < n; ++i) {
		bint t1, t2, g1, g2;
		cin >> t1 >> t2 >> g1 >> g2;
		t1--; t2--;
		played[findGame(t1, t2)] = true;
		if (g1 > g2) {
			scores[t1] += 3;
		} else if (g1 < g2) {
			scores[t2] += 3;
		} else if (g1 == g2) {
			scores[t1]++;
			scores[t2]++;
		}
	}

	recur();

	for (int i = 0; i < can_win.size(); ++i) {
		if (can_win[i]) 
			cout << "YES" << endl;
		else
			cout << "NO" << endl;
	}

	//system("pause");
}