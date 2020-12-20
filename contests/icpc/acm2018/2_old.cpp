#include <iostream>
#include <vector>
#include <sstream>
#include <map>
#include <string>
#include <set>
#include <functional>
#include <algorithm>
#include <fstream>
#include <memory>

using namespace std;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

string readFromFile(string fileName) {
	std::ifstream fin(fileName);
	std::string result;
	result.assign(std::istreambuf_iterator<char>(fin), std::istreambuf_iterator<char>());
	fin.close();

	return result;
}

typedef unsigned char mytype;
class fast_2d {
public:
	int size(void) const { return n; }
	void resize(int m, mytype a) { n = m; nn = n * n; mas.resize(nn * 2, a); }
	mytype& operator()(bool isReverse, int i, int j) { return mas[isReverse * nn + i * n + j]; }
	const mytype& operator()(bool isReverse, int i, int j) const { return mas[isReverse * nn + i * n + j]; }
private:
	int n;
	int nn;
	vector<mytype> mas;
};

map<string, int> id;
vector<string> id_reverse;
int id_count = 0;
int n;
vector<vector<int>> reps;
fast_2d order;
vector<set<int>> os;
vector<int> maked;
vector<int> maked2;

int get_id(const string& str) {
	auto& pos = id[str];
	if (pos == 0) {
		id_count++;
		pos = id_count;
		id_reverse.push_back(str);
		return pos-1;
	} else
		return pos-1;
}

const string& get_str(int id) {
	return id_reverse[id];
}

void read_data(void) {
	cin >> n;
	reps.resize(n, vector<int>());
	int nk;
	for (int i = 0; i < n; ++i) {
		cin >> nk;
		reps[i].resize(nk, 0);
		string str;
		for (int j = 0; j < nk; ++j) {
			cin >> str;
			reps[i][j] = get_id(str);
		}
	}
	order.resize(id_count, false);
	maked.resize(id_count, false);
	os.resize(id_count);
}

//shared_ptr<vector<int>> process_childs(int i) {
//	auto result = make_shared<vector<int>>();
//	if (maked[i] == 1)
//		throw exception();
//	if (maked[i] != 2) {
//		maked[i] = 1;
//		for (int j = 0; j < order.size(); j++) if (order(false, i, j)) {
//			result->push_back(j);
//			auto res = process_childs(j);
//			for (auto k : (*res)) {
//				auto& current = order(false, i, k);
//				if (!current) {
//					current = true;
//					order(true, k, i) = true;
//					result->push_back(k);
//				}
//			}
//		}
//		maked[i] = 2;
//		return result;
//	}
//	return result;
//}

int count1 = 0;
vector<int> count2;
map<int, bool> mama;
void process_childs(int i) {
	if (!maked[i]) {
		for (int j = 0; j < order.size(); j++) if (order(false, i, j)) {
			process_childs(j);
			for (int k = 0; k < order.size(); k++) {
				auto& current = order(false, i, k);
				if (order(false, j, k) && !current) {
					current = true;
					order(true, k, i) = true;
				}
			}
		}
		maked[i] = 1;
	}
}


vector<int> process_one_child(int i, int j) {
	vector<int> result;
	for (int k = 0; k < order.size(); k++) { 
		auto& current = order(false, i, k);
		if (!current && order(false, j, k)) {
			result.push_back(k);
			current = true;
			order(true, k, i) = true;
		}
	}
	return result;
}

void add_to_parents(int i, int k) {
	for (int j = 0; j < order.size(); j++) if (order(true, i, j)) {
		auto& current = order(false, j, k);
		if (!current) {
			current = true;
			order(true, k, j) = true;
			add_to_parents(j, k);
		}
	}
}

void process_parents(const vector<int>& change, int i) {
	for (const auto& k : change)
		add_to_parents(i, k);
}

void make_transit(void) {
	maked.clear();
	count2.resize(id_count, 0);
	maked.resize(id_count, false);
	for (int i = 0; i < order.size(); ++i)
		process_childs(i);
	maked2.clear();
	maked2.resize(id_count, 2);
}

bool is_unknown(int i, int j) {
	return (!order(false, i, j)) && (!order(false, j, i));
};

vector<int> order_size;

void prepare_order_size(void) {
	order_size.clear();
	order_size.resize(order.size(), 0);
	for (int i = 0; i < order.size(); i++) {
		int count1 = 0;
		for (int k = 0; k < order.size(); k++)
			if (order(false, i, k))
				count1++;
		order_size[i] = count1;
	}
}

bool my_greater(int i, int j) {
	return order_size[i] > order_size[j];
};

int main() {
	test(readFromFile("input30.txt"));

	read_data();
	
	for (int i = 0; i < reps.size(); ++i) {
		const auto& rep = reps[i];
		for (int j = 0; j < rep.size(); ++j) {
			for (int k = j+1; k < rep.size(); ++k) {
				int a = rep[j];
				int b = rep[k];
				auto& current = order(false, a, b);
				if (!current) {
					current = true;
					order(true, b, a) = true;
					//add_to_parents(a, b);
				}
			}
		}
	}

	//cout << "a";

	make_transit();

	vector<int> order_mas(order.size());
	for (int i = 0; i < order_mas.size(); ++i)
		order_mas[i] = i;
	prepare_order_size();
	sort(order_mas.begin(), order_mas.end(), my_greater);

	for (int i1 = 0; i1 < order_mas.size(); ++i1) {
		for (int j1 = i1+1; j1 < order_mas.size(); ++j1) {
	//for (int i = 0; i < order.size(); ++i) {
		//for (int j = i+1; j < order.size(); ++j) {
			int i = order_mas[i1];
			int j = order_mas[j1];
			if (is_unknown(i, j)) {
				auto str1 = get_str(i);
				auto str2 = get_str(j);
				if (lexicographical_compare(str1.begin(), str1.end(), str2.begin(), str2.end())) {
					order(false, i, j) = true;
					order(true, j, i) = true;
					auto change = process_one_child(i, j);
					process_parents(change, i);
				} else {
					order(false, j, i) = true;
					order(true, i, j) = true;
					auto change = process_one_child(j, i);
					process_parents(change, j);
				}
			}
		}
	}

	ofstream fout("count.txt");
	fout << count1 << endl;
	for (auto& i : count2)
		fout << i << " ";
	fout.close();

	prepare_order_size();
	sort(order_mas.begin(), order_mas.end(), my_greater);

	cout << order_mas.size() << endl;
	for (int i = 0; i < order_mas.size(); ++i)
		cout << get_str(order_mas[i]) << endl;
}