#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <cassert>
#include <optional>
#include <map>
#include <unordered_map>

using namespace std;
typedef int64_t bint;

void test(string str) {
	auto s = new string(str);
	auto iss = new istringstream(*s);
	cin.rdbuf(iss->rdbuf());
}

struct node {
	bint n;
	bint x, y;
	bint color;
	vector<node*> connect;
};

int norm(int a) {
	return (a + 4) % 4;
}

int find(node* where, const node* what) {
	for (int i = 0; i < 4; i++) {
		if (where->connect[i] == what) {
			return i;
		}
	}
	throw exception();
}

pair<node*, int> right(node* current, int s) {
	auto ans = current->connect[norm(s-1)];
	if (ans != nullptr)
		return { ans, find(ans, current) };
	else
		return { nullptr, 0 };
}

pair<node*, int> left(node* current, int s) {
	auto ans = current->connect[norm(s+1)];
	if (ans != nullptr)
		return { ans, find(ans, current) };
	else
		return { nullptr, 0 };
}

int main() {
	//test("0 1 0 0 0 0\n2 3 4 0 0 0\n0 5 0 0 0 0\n0 6 0 0 0 0\n0 0 0 0 0 0\n0 0 0 0 0 0\n1 2 3");
	//test("0 1 0 0 0 0\n3 2 4 0 0 0\n0 5 0 0 0 0\n0 6 0 0 0 0\n0 0 0 0 0 0\n0 0 0 0 0 0\n1 2 3");

	vector<vector<int>> p(6, vector<int>(6, 0));
	vector<node> nodes;
	nodes.reserve(6);
	for (int i = 0; i < 6; i++) {
		for (int j = 0; j < 6; j++) {
			bint tmp;
			cin >> tmp;
			p[i][j] = tmp;
			if (tmp != 0) {
				nodes.push_back({ nodes.size() + 1, j, i, tmp, {nullptr, nullptr, nullptr, nullptr} });
			}
		}
	}
	bint a, b, c;
	cin >> a >> b >> c;
	
	for (int i = 0; i < 6; i++) {
		for (int j = i+1; j < 6; j++) {
			if (nodes[i].y + 1 == nodes[j].y && nodes[i].x == nodes[j].x) {
				nodes[i].connect[3-1] = &nodes[j];
				nodes[j].connect[1-1] = &nodes[i];
			} else if (nodes[i].x + 1 == nodes[j].x && nodes[i].y == nodes[j].y) {
				nodes[i].connect[2-1] = &nodes[j];
				nodes[j].connect[4-1] = &nodes[i];
			} else {
				//nothing
			}
		}
	}

	start:
	for (int i = 0; i < 6; i++) {
		auto current = &nodes[i];
		for (int j = 0; j < 4; j++) {
			auto next = current->connect[j];
			if (next != nullptr) {
				auto r = right(next, find(next, current));
				auto l = left(next, find(next, current));
				if (r.first != nullptr && current->connect[norm(j+1)] == nullptr) {
					current->connect[norm(j + 1)] = r.first;
					r.first->connect[norm(r.second - 1)] = current;
					goto start;
				}
				if (l.first != nullptr && current->connect[norm(j - 1)] == nullptr) {
					current->connect[norm(j - 1)] = l.first;
					l.first->connect[norm(l.second + 1)] = current;
					goto start;
				}
			}
		}
	}
	for (int i = 0; i < 6; i++) {
		if (nodes[i].color == a) {
			for (int j = 0; j < 4; j++) {
				auto next = nodes[i].connect[j];
				if (next->color == b) {
					auto r = right(next, find(next, &nodes[i]));
					if (r.first != nullptr && r.first->color == c) {
						cout << "YES";
						return 0;
					}
				}
			}
		}
	}
	cout << "NO";
}