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

vector<vector<bool>> is_filled;
vector<vector<bool>> is_delete; // is_delete[i][j] означает, можно ли удалить символы с i по j в строке
string str;
bool get_can_delete(int i, int j) {
	if (i > j) throw exception();
	if (is_filled[i][j]) return is_delete[i][j];
	if (i == j) return false;

	auto len = j - i + 1;

	bool result = false;
	if (str[i] == str[j]) {
		if (len == 2 || len == 3) result = true;
		if (len >= 4) result |= get_can_delete(i+1, j-1);	
		if (len >= 5) {
			result |= get_can_delete(i+1, j-2);
			result |= get_can_delete(i+2, j-1);
			for (int k = i+1; k < j-2; ++k) {
				if (get_can_delete(i+1, k) && get_can_delete(k+2, j-1)) {
					result = true;
					break;
				}
			}
		}
	}
	for (int k = i; k < j; ++k) {
		if (get_can_delete(i, k) && get_can_delete(k+1, j)) {
			result = true;
			break;
		}
	}

	is_filled[i][j] = true;
	is_delete[i][j] = result;
	return result;
}

int findMax(void) {
	vector<int> max_pos(str.size(), 0);
	for (int i = 0; i < str.size(); ++i) {
		int end = str.size() - 1;
		int pos = end - i;
		if (get_can_delete(pos, end))
			max_pos[pos] = i+1;
		else {
			if (pos != end) max_pos[pos] = max_pos[pos+1];
			for (int j = 0; j < i; ++j) {
				if (get_can_delete(pos, pos + j)) {
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
	test("hhaghaeahhhibihcffhecehcgahdabaahcdfghgfdbgadjdh"); // ответ 8
	// test("awwbereare"); // ответ 2
	//test("faaghbfdahgeeheddcedfcgabdecgadgefeaachdecbbhgedgeabegdedaefdbfgcdcaeccaafahhfhbdgdhfhdhdbgdfffdbfafhaafchabacedgdfhdddafgebbacdcheededdbfgddbfcaccghggbbedhcbbgfdhggheabebeaagddgcgafahggfebfbcbccgbdebhdeehhchfahdbdcccghecdhbfeehehgdebhhbfddeafdbghffecehhcbdgcbhbfdgbgdchddagafgbbefahhhchcccbdfdcagcebbbcghdbbedbhfcddhehfddachfbdgafgbebadfccagdedaggcehffbgdfeehbadegcadccdgbfehgdghhfdebadhhbgafgefgdgddahghdegggagfhgbeceddegegfcceeceffgfdhbhfcfghfeehfedchfhchchgcedbebbhbgebgceceagcecfahhgdecefhacdcddbaghfceecggahhagafhccegeghchcaedahdhfehgbedddaghcccdhcfagafbefhgafeebdggcchhedabhccgdcaadfhbdccabddfggdhdacdbadfddbcbfacdhgdfcabahagcchgbafgcabgbahededhahedabaaaeeeghhdcefhbhaghbedbabdgfcadehchgbgecdhhachcaacaafefagbdghffaadabfdhedeehbccfbdcacabbbbaadbfchfdehccabgghdfehfggbhhghecbheebfbhfcefegdgfccaacedghhgebaegcbgcafgfeaabghbefbaacgggbeehbecfaheeefcadaecfgfchchfcccbafgeddagccebfhbhchfbhhbdhffadaghhbedfghcfgghaeedcacfceffddbhgcdccgdahdcabhbbchfadgcahddcfcaefgbgfehbbdcdggfhfaafgaefhhhdcebgcefccdc"); // ответ 175

	cin >> str;
	is_filled = vector<vector<bool>>(str.size(), vector<bool>(str.size(), false));
	is_delete = vector<vector<bool>>(str.size(), vector<bool>(str.size(), false));
	cout << str.size() - findMax();
}