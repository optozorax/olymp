#include <bits/stdc++.h>

using namespace std;

static std::mt19937 generator(58);

/* Random number [0; 1] */
double random_01(void) {
	static std::uniform_real_distribution<double> distribution(0, 1);
	return distribution(generator);
}

bool random_bool(void) {
	static std::uniform_int_distribution<char> distribution(0, 1);
	return distribution(generator);
}

int random_int(int min, int max) {
	static std::uniform_int_distribution<char> distribution(min, max);
	return distribution(generator);
}

/* Executing time of `f` in microseconds. */
inline double calc_time_microseconds(const std::function<void(void)>& f) {
	using namespace std::chrono;
	auto start = high_resolution_clock::now();
	f();
	auto end = high_resolution_clock::now();
	return duration_cast<microseconds>(end - start).count();
}

#include "../lru-cache-3.cpp"

int main() {
	int size = 100;
	int attempts = 100;
	int calls = 100000;
	int int_size = 10000;

	vector<vector<tuple<bool, int, int>>> test;
	for (int i = 0; i < attempts; ++i) {
		test.push_back({});
		for (int j = 0; j < calls; ++j) {
			test.back().push_back(make_tuple(
				random_bool(),
				// random_int(0, 100) < 10, 
				random_int(0, int_size), 
				random_int(0, int_size)
			));
		}
	}

	unsigned int black_box = 0;
	double sum = calc_time_microseconds([&] () {
		for (const auto& i : test) {
			LRUCache c(size);
			for (const auto& j : i) {
				if (get<0>(j)) {
					c.put(get<1>(j), get<2>(j));
				} else {
					black_box += c.get(get<1>(j));
				}
			}
		}
	});
	cout << "black_box: " << black_box << endl;
	cout << "time: " << sum / double(attempts) / 1000. << "ms" << endl;
}

/* 

чередующиеся команды:
lru-3-move-front: 4.15736ms
lru-3: 4.33804ms
lru-2: 5.38521ms
lru: 11.3337ms

приоритет отдаётся get (90%):
lru-3-move-front: 4.12348ms
lru-3: 4.36802ms
lru-2: 5.37976ms
lru: 11.2295ms

без деаллокации:
lru-3-move-front: 4.21804ms
lru-2: 5.47817ms
lru: 11.6019ms

 */