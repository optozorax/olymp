#include <iostream>
#include <vector>
#include <map>
#include <algorithm>
#include <iomanip>

using namespace std;

int result = 0;
int check_count = 0;
void process_first_cube(map<int, int>& prob, vector<int>& cube1, vector<int>& cube2, int pos);
void process_second_cube(map<int, int>& prob, vector<int>& cube1, vector<int>& cube2, int pos);

void process_first_cube(map<int, int>& prob, vector<int>& cube1, vector<int>& cube2, int pos) {
	int start = (pos > 0) ? cube1[pos-1] : 0;
	int end = (pos > 0) ? start+10 : (prob.begin()->first+1) / 2;

	auto test_number = [&] (int num) -> bool {
		check_count++;
		for (int i = 0; i < pos; ++i) {
			auto& current = prob[num + cube2[i]];
			if (current == 0) {
				for (int j = 0; j < i; ++j)
					prob[num + cube2[j]]++;
				return false;
			}
			current--;
		}
		return true;
	};

	for (int i = start; i <= end; ++i) {
		if (test_number(i)) {
			cube1[pos] = i;
			process_second_cube(prob, cube1, cube2, pos);
			for (int j = 0; j < pos; ++j)
				prob[i + cube2[j]]++;
		}
	}
}

void process_second_cube(map<int, int>& prob, vector<int>& cube1, vector<int>& cube2, int pos) {
	int start = (pos > 0) ? cube2[pos-1] : (*prob.begin()).first - cube1[0];
	int end = (pos > 0) ? start+10 : start;

	auto test_number = [&] (int num) -> bool {
		check_count++;
		for (int i = 0; i <= pos; ++i) {
			auto& current = prob[num + cube1[i]];
			if (current == 0) {
				for (int j = 0; j < i; ++j)
					prob[num + cube1[j]]++;
				return false;
			}
			current--;
		}
		return true;
	};

	for (int i = start; i <= end; ++i) {
		if (test_number(i)) {
			cube2[pos] = i;
			if (pos+1 == 6) {
				result++;
				for (int j = 0; j <= pos; ++j)
					prob[i + cube1[j]]++;
				return;
			} else
				process_first_cube(prob, cube1, cube2, pos+1);
			for (int j = 0; j <= pos; ++j)
				prob[i + cube1[j]]++;
		}
	}
}

void write_probability(const vector<int>& cube1, const vector<int>& cube2) {
	//  sum  probability * 36
	map<int, int> res;
	for (int i = 0; i < 6; ++i) {
		for (int j = 0; j < 6; ++j) {
			res[cube1[i] + cube2[j]]++;
		}
	}

	for (auto& i : res)
		cout << setw(5) << i.first << "|";
	cout << endl;
	for (auto& i : res)
		cout << setw(5) << i.second << "|";
	cout << endl;
}

map<int, int> gen_probability(const vector<int>& a, const vector<int>& b) {
	map<int, int> res;
	for (int i = 0; i < 6; ++i) {
		for (int j = 0; j < 6; ++j) {
			res[a[i] + b[j]]++;
		}
	}
	return res;
}

bool is_equal(const map<int, int>& a, const map<int, int>& b) {
	auto i = a.begin();
	auto j = b.begin();
	while (i != a.end() && j != b.end()) {
		if (i->first == j->first && i->second == j->second) {

		} else {
			return false;
		}
		i++;
		j++;
	}
	return true;
}

int main() {
	//vector<int> a = {1, 1, 1, 2, 2, 2};
	//vector<int> b = {1, 1, 1, 2, 2, 2};
	vector<int> cube1 = {1, 2, 3, 4, 5, 6};
	vector<int> cube2 = {1, 2, 3, 4, 5, 6};
	vector<int> a = {900, 902, 903, 905, 905, 908};
	vector<int> b = {900, 901, 902, 906, 907, 908};

	vector<int> cb1(6, 0), cb2(6, 0);
	process_first_cube(gen_probability(a, b), cb1, cb2, 0);
	cout << result << endl;
	cout << check_count << endl;
	// write_probability(a, b);

	// vector<int> cube1 = {1, 2, 3, 4, 5, 6};
	// vector<int> cube2 = {1, 2, 3, 4, 5, 6};
	// auto prob = gen_probability(a, b);

	// vector<int> cube3 = {2, 3, 4, 5, 6, 7};
	// vector<int> cube4 = {0, 1, 2, 3, 4, 5};


	// int i1, i2, i3, i4, i5, i6;
	// int j1, j2, j3, j4, j5, j6;
	// for (i1 = 0; i1 <= 1; ++i1)
	// for (i2 = i1; i2 < 12; ++i2)
	// for (i3 = i2; i3 < 12; ++i3)
	// for (i4 = i3; i4 < 12; ++i4)
	// for (i5 = i4; i5 < 12; ++i5)
	// for (i6 = i4; i6 < 12; ++i6)

	// for (j1 = 0; j1 <= 2; ++j1)
	// for (j2 = j1; j2 < 12; ++j2)
	// for (j3 = j2; j3 < 12; ++j3)
	// for (j4 = j3; j4 < 12; ++j4)
	// for (j5 = j4; j5 < 12; ++j5)
	// for (j6 = j4; j6 < 12; ++j6)

	// if (i1 + j1 == 2 && i6 + j6 == 4)

	// if (is_equal(prob, gen_probability({i1, i2, i3, i4, i5, i6}, {j1, j2, j3, j4, j5, j6}))) {
	// 	cout << "WOW: " << i1 << ", " << i2 << ", " << i3 << ", " << i4 << ", " << i5 << ", " << i6 << endl;
	// 	cout << "     " << j1 << ", " << j2 << ", " << j3 << ", " << j4 << ", " << j5 << ", " << j6 << endl;
	// }
}

/*

// Те самые 5 вариантов кубиков
{ {0, 1, 1, 2, 2, 3}, {2, 4, 5, 6, 7, 9} }, 
{ {0, 1, 2, 3, 4, 5}, {2, 3, 4, 5, 6, 7} }, // Первый, который приходит на ум
{ {0, 2, 3, 4, 5, 7}, {2, 3, 3, 4, 4, 5} }, 
{ {1, 2, 2, 3, 3, 4}, {1, 3, 4, 5, 6, 8} }, 
{ {1, 2, 3, 4, 5, 6}, {1, 2, 3, 4, 5, 6} }, // Оригинальный
. 
*/