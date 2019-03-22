#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <map>
#include <fstream>

using namespace std;

int main() {
	string line;
	cin >> line;
	long long max = 0;
	string letters;
	int k = 0;
	for (int i = 0; i < line.size(); ++i) {
		if (letters.find(line[i]) != string::npos) {
			if (letters.size() >= max)
				max = letters.size();
			k++;
			i = k;
			letters.clear();
		}
		else {
			letters += line[i];
			if (i == line.size() - 1 && letters.size() >= max)
				max = letters.size();
		}
	}

	cout << max;
	return 0;
}