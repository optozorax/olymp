#include <iostream>
#include <vector>
#include <algorithm>
#include <sstream>

using namespace std;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

enum palindrome
{
	wrong_order = -1,
	cant_delete = 0, // Это нельзя удалить
	cant_delete_one_symbol = 1, // Это палиндром из одного символа, его самого по себе нельзя удалить, но в составе других палиндромов можно
	can_delete = 2, 
};

vector<vector<palindrome>> is_delete; // is_delete[i][j] означает, можно ли удалить символы с i по j в строке
string str;

bool is_palindrome(int l, int r) {
	for (int i = l, j = r; i <= (l + r) / 2; ++i, --j) {
		if (str[i] != str[j])
			return false;
	}
	return true;
}

void makeTable(void) {
	is_delete.resize(str.size(), vector<palindrome>(str.size(), wrong_order));
	for (int i = 0; i < str.size(); ++i) {
		for (int j = 0; j < str.size() - i; ++j) {
			// Делаем обход по диагоналям в таблице
			int posi = j;
			int posj = i + j;
			//string locstr = str.substr(posi, posj - posi + 1); // Для дебага, чтобы смотреть какая сейчас строка рассматривается
			
			auto& current = is_delete[posi][posj];
			current = cant_delete;
			if (posi == posj)
				// Если это один символ, то его удалить нельзя, но в составе других палиндромов можно
				current = cant_delete_one_symbol;
			else
			if (is_palindrome(posi, posj))
				// Если это просто палиндром, то можно удалить
				current = can_delete;
			else 
			if (str[posi] == str[posj] && is_delete[posi+1][posj-1])
				// Если крайние две буквы равны, и внутри можно удалить, либо то что внутри является одной буквой, то это тоже можно удалить
				current = can_delete;
			else {
				for (int k = posi; k < posj; ++k) {
					if (is_delete[posi][k] + is_delete[k+1][posj] == 4) {
						// Часть, состоящую из двух частей, которые можно удалить, тоже можно удалить
						current = can_delete;
						break;	
					} else if (is_delete[posi][k] + is_delete[k+1][posj] == 3) {
						// Если одна часть содержит после удаления одну букву, а другую можно удалить, то считается, что изначальная часть содержит одну букву после удаления всех элементов
						current = cant_delete_one_symbol;
						// Здесь break не делаем, потому что потенциально может найтись такое сочетание, которое полностью можно удалить, а не с ограничением на одну букву
					}
				}
			}
		}
	}
}

int findMax(void) {
	// Динамика, похожая на 9 задачу
	vector<int> max_pos(str.size(), 0);
	for (int i = 0; i < str.size(); ++i) {
		int end = str.size() - 1;
		int pos = end - i;
		if (is_delete[pos][end] == can_delete)
			max_pos[pos] = i+1;
		else {
			if (pos != end)
				max_pos[pos] = max_pos[pos+1];
			for (int j = 0; j < i; ++j) {
				if (is_delete[pos][pos + j] == can_delete) {
					max_pos[pos] = max(max_pos[pos], j+1 + max_pos[pos + j+1]);
				}
			}
		}
	}
	return max_pos[0];
}

int main() {
	//test("abcdccb");
	//test("abbccad");
	//test("abcdcba");
	//test("aaaerttew");
	//test("cbcettre");
	//test("hhaghaeahhhibihcffhecehcgahdabaahcdfghgfdbgadjdh"); // ответ 8
	//test("awwbereare"); // ответ 2
	//test("faaghbfdahgeeheddcedfcgabdecgadgefeaachdecbbhgedgeabegdedaefdbfgcdcaeccaafahhfhbdgdhfhdhdbgdfffdbfafhaafchabacedgdfhdddafgebbacdcheededdbfgddbfcaccghggbbedhcbbgfdhggheabebeaagddgcgafahggfebfbcbccgbdebhdeehhchfahdbdcccghecdhbfeehehgdebhhbfddeafdbghffecehhcbdgcbhbfdgbgdchddagafgbbefahhhchcccbdfdcagcebbbcghdbbedbhfcddhehfddachfbdgafgbebadfccagdedaggcehffbgdfeehbadegcadccdgbfehgdghhfdebadhhbgafgefgdgddahghdegggagfhgbeceddegegfcceeceffgfdhbhfcfghfeehfedchfhchchgcedbebbhbgebgceceagcecfahhgdecefhacdcddbaghfceecggahhagafhccegeghchcaedahdhfehgbedddaghcccdhcfagafbefhgafeebdggcchhedabhccgdcaadfhbdccabddfggdhdacdbadfddbcbfacdhgdfcabahagcchgbafgcabgbahededhahedabaaaeeeghhdcefhbhaghbedbabdgfcadehchgbgecdhhachcaacaafefagbdghffaadabfdhedeehbccfbdcacabbbbaadbfchfdehccabgghdfehfggbhhghecbheebfbhfcefegdgfccaacedghhgebaegcbgcafgfeaabghbefbaacgggbeehbecfaheeefcadaecfgfchchfcccbafgeddagccebfhbhchfbhhbdhffadaghhbedfghcfgghaeedcacfceffddbhgcdccgdahdcabhbbchfadgcahddcfcaefgbgfehbbdcdggfhfaafgaefhhhdcebgcefccdc"); // ответ 175

	cin >> str;
	makeTable();
	cout << str.size() - findMax();
}