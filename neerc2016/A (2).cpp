#define _CRT_SECURE_NO_WARNINGS
#include <vector>
#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <algorithm>
using namespace std;

enum type {
	word,
	capitalized,
	space,
	punctuation
};

void test(string str) {
	auto str1 = new string(str);
	auto iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

bool is_letter(char c) {
	return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

bool is_upper(char c) {
	return (c >= 'A' && c <= 'Z');
}

char read(string& str) {
	char c = str.front();
	str.erase(str.begin());
	return c;
}

typedef pair<type, string> elem;

int main() {
	ifstream t("abbreviation.in");
	stringstream buffer;
	buffer << t.rdbuf();

	string sin = buffer.str();

	vector<elem> s;
	char c = read(sin);
	if (is_letter(c))
		s.push_back({ word,{ c } });
	else
		s.push_back({ punctuation,{ c } });
	while (!sin.empty()) {
		c = read(sin);
		if (s.back().first == word) {
			if (is_letter(c)) {
				s.back().second.push_back(c);
			}
			else {
				s.push_back({ punctuation,{ c } });
			}
		}
		else {
			if (is_letter(c)) {
				s.push_back({ word,{ c } });
			}
			else {
				s.back().second.push_back(c);
			}
		}
	}

	// Определяем, какие слова capitalized, а какая пунктуация ровно один пробел
	for (auto& i : s) {
		if (i.first == word) {
			if (i.second.size() > 1 && is_upper(i.second[0]) &&
				find_if(i.second.begin() + 1, i.second.end(), [](auto c) -> bool {
				return is_upper(c);
			}) == i.second.end()) {
				i.first = capitalized;
			}
		}
		else {
			if (i.second == " ")
				i.first = space;
		}
	}

	auto find_abbr = [&](int from, int& start, int& end) -> bool {
		// Находим кандидата для аббревиатуры
		start = -1; end = -1;
		for (int i = from; i < s.size(); ++i) {
			if (start == -1 && s[i].first == capitalized) {
				start = i;
			}
			else if (start != -1 && (i - start) % 2 == 0 && s[i].first != capitalized) {
				if (i - start > 2) {
					end = i - 2;
					break;
				}
				else {
					start = -1;
					end = -1;
				}
			}
			else if (start != -1 && (i - start) % 2 == 1 && s[i].first != space) {
				if (i - start > 2) {
					end = i - 1;
					break;
				}
				else {
					start = -1;
					end = -1;
				}
			}
		}
		if (end == -1 && start != -1) {
			end = s.size() - 1;
			if (end - start < 2) {
				start = -1;
				end = -1;
			}
		}
		return start != -1;
	};

	// Заменяем все слова на аббривеатуры
	int from = 0, start, end;
	while (find_abbr(from, start, end)) {
		string abbr;
		for (int i = start; i <= end; i += 2)
			abbr += s[i].second[0];
		s.insert(s.begin() + start, { punctuation, abbr + " (" });
		s.insert(s.begin() + end + 2, { punctuation, ")" });
		from = end + 2;
	}

	// Выводим результат
	for (auto& i : s)
		cout << i.second;
	cout << endl;
}