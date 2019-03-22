#include <bits/stdc++.h>

using namespace std;

struct game {
	int i, j, k; // last elem
	int awin, bwin; // count of win by each comand
	bool is_game_over;
	bool is_first_game;
	int ascore, bscore; // score of current game
};

const int maxsize = 201;

vector<vector<vector<game>>> s(maxsize, vector<vector<game>>(maxsize));

void write_possible_games(game g) {
	int offset;

	// Если это последний матч, то максимальное количество очков должно быть 15
	if ((g.awin + g.bwin) == 4) offset = 15;
	else offset = 25;

	auto push_game = [&] (int a, int b) {
		if (g.ascore + a < maxsize && g.bscore + b < maxsize) {
			int newawin = g.awin+((a>b) ? 1 : 0), newbwin = g.bwin+((b>a) ? 1 : 0);
			auto& current = s[g.ascore+a][g.bscore+b];

			// Чтобы не переполнять массив, добавляем только те игры, счет которых ещё не был для текущего количества очков
			bool is_should_push_back = true;
			for (auto& i : current)
				is_should_push_back &= !(i.awin == newawin && i.bwin == newbwin);
			if (is_should_push_back)
				current.push_back({
					g.i, g.j, g.k, 
					newawin, newbwin,
					max(newawin, newbwin) == 3, (newawin + newbwin) == 1,
					a, b
				});
		}
	};

	for (int i = 0; i <= offset-2; ++i) {
		push_game(offset, i);
		push_game(i, offset);
	}

	for (int i = 0; i < maxsize; ++i) {
		push_game(offset+1+i, offset-1+i);
		push_game(offset-1+i, offset+1+i);
	}
}

void make_table(void) {
	for (int i = 0; i < maxsize; ++i){
		for (int j = 0; j < maxsize; ++j) {
			s[i][j].reserve(10);
		}
	}

	write_possible_games({
		0, 0, 0,
		0, 0,
		false, false,
		0, 0
	});

	for (int i = 0; i < maxsize; ++i){
		for (int j = 0; j < maxsize; ++j) {
			for (int k = 0; k < s[i][j].size(); ++k) {
				auto& current = s[i][j][k];
				if (!current.is_game_over) {
					write_possible_games({
						i, j, k,
						current.awin, current.bwin,
						false, false,
						i, j
					});
				}
			}
		}
	}
}

void write_recursively(int a, int b, int i) {
	auto& current = s[a][b][i];
	if (!current.is_first_game)
		write_recursively(current.i, current.j, current.k);
	cout << current.ascore << ":" << current.bscore << " ";
}

void write_game(int a, int b) {
	auto& current = s[a][b];
	int min = -1;
	for (int i = 0; i < current.size(); ++i) {
		if (current[i].is_game_over) {
			if (min == -1) min = i;
			else 
			if (current[min].awin < current[i].awin) 
				min = i;
			else 
			if (current[min].awin == current[i].awin && (current[min].awin-current[min].bwin) < abs(current[i].awin-current[i].bwin))
				min = i;
		}
	}

	if (min == -1) {
		cout << "Impossible";
	} else {
		cout << current[min].awin << ":" << current[min].bwin << endl;
		write_recursively(a, b, min);
	}

	cout << endl;
}

int main() {
	make_table();

	int n;
	cin >> n;
	for (int i = 0; i < n; ++i) {
		int a, b;
		cin >> a >> b;
		write_game(a, b);
	}
}