#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <numeric>
#include <string>
#include <fstream>
using namespace std;
typedef int64_t bint;

struct fraction 
{
	fraction() : a(0), b(1) {}
	fraction(bint a1, bint b1) : a(a1), b(b1) {
		if (a == 0 && b == 0) 
			b = 1;
	}
	bint a, b;
};

fraction optimize(const fraction& a) {
	bint g = gcd(a.a, a.b);
	fraction result = a;
	if (g != 0) {
		result.a /= g;
		result.b /= g;
	}
	return result;
}

fraction operator*(const fraction& a, const fraction& b) {
	return optimize(fraction(a.a*b.a, a.b*b.b));
}

fraction operator+(const fraction& a, const fraction& b) {
	return optimize(fraction(a.a*b.b + b.a*a.b, a.b*b.b));
}

ostream& operator<<(ostream& out, const fraction& a) {
	if (a.b == 1) 
		out << a.a;
	else
		out << (to_string(a.a) + "/" + to_string(a.b));
	return out;
}

struct game_result
{
	int game_count;
	int score;

	int count;
};

vector<vector<vector<fraction>>> tables;

game_result simulate_game(const vector<int>& game, int k) {
	int tablen = floor(log2(game.size()))-1;
	auto& table = tables[tablen]; 
	for (int i = 0; i < game.size(); ++i) {
		for (int j = 0; j < game.size(); ++j) {
			table[game[i]-1][game[j]-1].a++;
		}
	}

	game_result result = {0, 0, 0};
	vector<int> next_game(game.size()/2);
	for (int i = 0; i < game.size(); i+=2) {
		if (game[i] == k) {
			if (game[i] > game[i+1]) {
				result.game_count = 1;
				result.score = game[i+1];
			}
		}
		if (game[i+1] == k) {
			if (game[i+1] > game[i]) {
				result.game_count = 1;
				result.score = game[i];
			}
		}

		next_game[i/2] = max(game[i], game[i+1]);
	}
	game_result res = {0, 0, 0};
	if (next_game.size() != 1)
		res = simulate_game(next_game, k);
	return {result.game_count + res.game_count, result.score + res.score, 0};
}

void normalize_tables() {
	for (auto& i : tables) {
		auto copy = i;
		for (int j = 0; j < i.size(); ++j) {
			for (int k = 0; k < i[j].size(); ++k) {
				copy[k][j] = optimize(fraction(i[k][j].a, i[j][j].a));
			}
		}
		i = copy;
	}
}

int main() {
	int k = 3, n = 2;
	int count = pow(2.0, n);
	vector<int> game(count);
	for (int i = 0; i < count; ++i)
		game[i] = i+1;

	tables = vector<vector<vector<fraction>>>(n, vector<vector<fraction>>(count, vector<fraction>(count)));

	vector<game_result> results2;
	bint howMany = 0, all = 1ll*2ll*3ll*4ll*5ll*6ll*7ll*8ll*9ll*10ll*11ll*12ll*13ll*14ll*15ll*16ll;
	do {
		if (howMany%10000000 == 0) 
			cout << "\r" << fixed << 100.0*howMany/all << "%           ";

		howMany++;
		auto i = simulate_game(game, k);
		for (auto& j : results2) {
			if (j.game_count == i.game_count && j.score == i.score) {
				j.count++;
				goto next_elem;
			}
		}
		results2.push_back(i);
		results2.back().count++;
		next_elem:;
	} while (next_permutation(game.begin(), game.end()));

	sort(results2.begin(), results2.end(), [] (auto& a, auto& b) -> bool {
		if (a.game_count == b.game_count)
			return a.score < b.score;
		else
			return a.game_count < b.game_count;
	});

	int64_t sum = 0;
	int64_t mycount = 0;
	for (auto& i : results2) {
		sum += i.count * i.score;
		mycount += i.count;
		cout << setw(2) << i.game_count << ":" << setw(3) << i.score << " x " << i.count/384 << endl;
	}

	cout << "Result: " << sum << "/" << mycount << " = " << fraction(sum, count) << endl;

	normalize_tables();

	ofstream fout("5_tables.txt");
	for (auto& i : tables) {
		for (auto& j : i) {
			for (auto& k : j) {
				fout << setw(8) << k;
			}
			fout << endl;
		}

		fout << endl << endl;
	}
	fout.close();

	system("pause");
}