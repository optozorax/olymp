#include <iostream>
#include <string>
#include <algorithm>

using namespace std;

int main() {
	string str;
	getline(cin, str);

	if (str == "0") {
		cout << "0";
		return 0;
	}

	auto delimeter = find(str.begin(), str.end(), ',');

	string::iterator start_it = find_if(str.begin(), str.end(), 
		[] (auto a) -> bool { return a != '0'; });
	if (*start_it == ',') start_it--;

	string::iterator end_it;
	if (delimeter == str.end()) 
		end_it = str.end();
	else {
		end_it = find_if(str.rbegin(), str.rend(), 
		[] (auto a) -> bool { return a != '0'; }).base()-1;
		if (*end_it == ',') end_it--;
		end_it++;
	}

	for (; start_it != end_it; ++start_it)
		cout << *start_it;
}