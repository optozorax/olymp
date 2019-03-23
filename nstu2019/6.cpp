#include <iostream>
#include <vector>
#include <string>
#include <cctype>
#include <sstream>
using namespace std;

int i = 0;
string str;

#define next_if(a) if (str[i++] != (a)) {i--; return false; }
#define call(a) if (!(a)) return false;

bool check_string(void);
bool check_number(void);
bool check_false(void);
bool check_true(void);
bool check_null(void);
bool check_array(void);
bool check_object(void);
bool check_value(void);

void ignore_spaces() {
	while (str[i] == ' ' || str[i] == '\t' || str[i] == '\n') i++;
}

bool check_string(void) {
	next_if('"');
	if (str[i] != '"')
		for (int j = 0; j < 50; ++j) {
			if (str[i] == '\"') break;
			if (!islower(str[i++])) return false;
		}
	next_if('"');

	return true;
}

bool check_number(void) {
	string num;
	num += str[i];
	if (!isdigit(str[i])) return false;
	i++;
	while (isdigit(str[i])) {
		num += str[i]; 
		i++;
	}
	string thousand(to_string(1000));
	if (num.size() > thousand.size())
		return false;
	else if (num.size() < thousand.size())
		return true;
	else if (num > thousand)
		return false;
	else
		return true;
}

bool check_false(void) {
	next_if('f');
	next_if('a');
	next_if('l');
	next_if('s');
	next_if('e');

	return true;
}

bool check_true(void) {
	next_if('t');
	next_if('r');
	next_if('u');
	next_if('e');

	return true;
}

bool check_null(void) {
	next_if('n');
	next_if('u');
	next_if('l');
	next_if('l');
}

bool check_array(void) {
	next_if('[');
	ignore_spaces();
	if (str[i] != ']') {
	start:
		ignore_spaces();
		call(check_value());
		ignore_spaces();
		if (str[i] == ',') { i++; goto start; }
	}

	next_if(']');
}

bool check_object(void) {
	next_if('{');
	ignore_spaces();
	if (str[i] != '}') {
	start:
		ignore_spaces();
		call(check_string());
		ignore_spaces();
		next_if(':');
		ignore_spaces();
		call(check_value());
		ignore_spaces();
		if (str[i] == ',') { i++; goto start; }
	}

	next_if('}');	
}

bool check_value(void) {
	ignore_spaces();
	if (check_number()) return true;
	if (check_string()) return true;
	if (check_null()) return true;
	if (check_true()) return true;
	if (check_false()) return true;
	if (check_object()) return true;
	if (check_array()) return true;
	ignore_spaces();

	return false;
}

bool check_value_start(void) {
	call(check_value());
	ignore_spaces();
	return i == str.size();
}

void check(string s) {
	str = s + '\n';
	i = 0;
	bool returned = check_value_start();
	if (returned) 
		cout << "OK:    " << s << endl;
	else
		cout << "ERROR: " << s.substr(0, i) << "^^^" << ((i < s.size()) ? s.substr(i, s.size() - i) : "") << endl;
}

int main() {
	/*check("null");
	check("truee");
	check("true   ");
	check("true");
	check("{}       ");
	check(" null  ");
	check("    false    ");
	check("{}");
	check("[]");
	check("\"\"");
	check("\"aoeuaoeu\"");
	check("5");
	check("100");
	check("500");
	check("999");
	check("1000");
	check("1001");
	check("100500");
	check("{\"a\":{}}");
	check("{\"a\":[]}");
	check("{\"a\":\"aoeu\"}");
	check("{\"a\":\"\",\"b\":{}}");
	check("[100, \"a\", true]");
	check("[{}, {\"key\":null}, null]");
	check("{\"key\": \"sample\" , \"value\": [100, \"a\", true]}");
	check("{\"key\": \"sample\" , \"value\": [100, \"a\", ]}");
	check("{\"key\": \"sample\" , \"value\": [100500, \"a\", true]}");*/
	//system("pause");

	// --------------------------------------------------
	// Ввод всего cin в строку, вместе с переносами строк
	string input, line;
	while (getline(cin, line)) {
		if (line == "^D")
			break;

		input += " " + line;
	}
	// Ввод всего cin в строку, вместе с переносами строк
	// --------------------------------------------------

	i = 0;
	str = input;
	if (check_value_start())
		cout << "YES";
	else
		cout << "NO";

	//system("pause");
}