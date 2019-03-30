#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <numeric>
#include <string>
#include <fstream>
#include "5_common.h"
using namespace std;

struct game_result
{
	int game_count;
	int score;

	int count;
};

vector<vector<vector<prob_line>>> lines;
vector<prob_table> tables;

game_result simulate_game(const vector<int>& game, int k) {
	int tablen = floor(log2(game.size()))-1;
	auto& table = tables[tablen]; 
	for (auto& i : game) {
		for (auto& j : game) {
			table[i-1][j-1].a++;
		}
	}

	auto& line = lines[tablen];
	for (auto& i : game) {
		for (auto& j : game) {
			for (auto& k : game) {
				line[i-1][j-1][k-1].a++;
			}
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
		for (int j = 0; j < i.size(); ++j) {
			bint temp = i[j][j].a;
			for (int k = 0; k < i[j].size(); ++k) {
				if (temp != 0) {
					i[j][k].b = temp;
					i[j][k].optimize();
				}
			}
		}
	}

	for (auto& line : lines) {
		for (int i = 0; i < line.size(); ++i) {
			for (int j = 0; j < line[i].size(); ++j) {
				bint temp = line[i][j][i].a;
				for (int k = 0; k < line[i][j].size(); ++k) {
					if (temp != 0) {
						line[i][j][k].b = temp;
						line[i][j][k].optimize();
					}
				}
			}
		}
	}
}

int main() {
	int k = 5, n = 3;
	int count = pow(2.0, n);
	vector<int> game(count);
	for (int i = 0; i < count; ++i)
		game[i] = i+1;

	tables.resize(n, vector<prob_line>(count, prob_line(count, fraction(0))));
	lines.resize(n, vector<vector<prob_line>>(count, vector<prob_line>(count, prob_line(count, fraction(0)))));

	vector<game_result> results2;
	//bint howMany = 0, all = 1ll*2ll*3ll*4ll*5ll*6ll*7ll*8ll*9ll*10ll*11ll*12ll*13ll*14ll*15ll*16ll;
	do {
		//if (howMany%10000000 == 0) cout << "\r" << fixed << 100.0*howMany/all << "%           ";

		//howMany++;
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

	cout << "Result: " << sum << "/" << mycount << " = " << fraction(sum, mycount).optimize() << endl;

	normalize_tables();

	ofstream fout("5_tables.txt");
	for (auto& i : tables)
		fout << i << endl << endl;
	fout.close();

	fout.open("5_lines.txt");
	for (auto& line : lines) {
		fout << "new table" << endl;
		for (int i = 0; i < line.size(); ++i) {
			for (int j = 0; j < line[i].size(); ++j) {
				fout << i+1 << " and " << j+1 << ": " << line[i][j] << endl;
			}
		}
		fout << endl << endl << endl;
	}
	fout.close();

	system("pause");
}