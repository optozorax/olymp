/* Послание тому, кто вдруг будет это читать: извини, но я не знаю почему это работает правильно! Я просто реализовал тупую идею и всё. Оно приняло. Я не дошел до этого логически, и не уверен в этом. Я просто реализовал идею, которую мне дала интуиция :( */

#define _USE_MATH_DEFINES

#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <functional>
#include <sstream>
#include <string>
#include <bitset>
#include <numeric>
#include <algorithm>
#include <iomanip>
using namespace std;
typedef int64_t bint;
void test(string str) {
  string* str1 = new string(str);
  istringstream* iss = new istringstream(*str1);
  cin.rdbuf(iss->rdbuf());
}

struct stone
{
	bint price;
	bint mass;
	double coef;
};

bint gcd(bint u, bint v) {
    // simple cases (termination)
    if (u == v)
        return u;

    if (u == 0)
        return v;

    if (v == 0)
        return u;

    // look for factors of 2
    if (~u & 1) // u is even
    {
        if (v & 1) // v is odd
            return gcd(u >> 1, v);
        else // both u and v are even
            return gcd(u >> 1, v >> 1) << 1;
    }

    if (~v & 1) // u is odd, v is even
        return gcd(u, v >> 1);

    // reduce larger argument
    if (u > v)
        return gcd((u - v) >> 1, v);

    return gcd((v - u) >> 1, u);
}

void print(stone s) {
	cout << setprecision(2) << fixed << setw(6) << s.coef << setw(5) << s.price << setw(5) << s.mass << endl;
}

int main() {
	//test("3 2\n5 1\n3 2\n2 3\n");
	//test("100 50\n1 1\n2 1\n3 1\n4 1\n5 1\n6 1\n7 1\n8 1\n9 1\n10 1\n1 1\n1 2\n1 3\n1 4\n1 5\n1 6\n1 7\n1 8\n1 9\n1 10\n1 11\n1 12\n1 13\n1 14\n1 15\n1 16\n1 17\n1 18\n1 19\n1 20\n2 1\n2 2\n2 3\n2 4\n2 5\n2 6\n2 7\n2 8\n2 9\n2 10\n2 11\n2 12\n2 13\n2 14\n2 15\n2 16\n2 17\n2 18\n2 19\n2 20\n3 1\n3 2\n3 3\n3 4\n3 5\n3 6\n3 7\n3 8\n3 9\n3 10\n3 11\n3 12\n3 13\n3 14\n3 15\n3 16\n3 17\n3 18\n3 19\n3 20\n3 21\n3 22\n3 23\n3 24\n3 25\n3 26\n3 27\n3 28\n3 29\n3 30\n3 31\n3 32\n3 33\n3 34\n3 35\n3 36\n3 37\n3 38\n3 39\n3 40\n3 51\n3 52\n3 53\n3 54\n3 55\n3 56\n3 57\n3 58\n3 59\n3 60\n");

	vector<stone> stones;
	bint n, k;
	cin >> n >> k;
	for (int i = 0; i < n; ++i) {
		bint price, mass;
		cin >> price >> mass;
		stones.push_back({price, mass, 1});
	}

	bint pricesum = 0, masssum = 0;
	for (int i = 0; i < k; ++i) {
		for (auto& i : stones)
			i.coef = double(pricesum + i.price) / double(masssum + i.mass);
		sort(stones.begin(), stones.end(), [] (const auto& a, const auto& b) -> bool {
			return a.coef > b.coef;
		});
		pricesum += stones[0].price;
		masssum += stones[0].mass;
		stones.erase(stones.begin());
	}

	bint gcdsum = gcd(pricesum, masssum);
	pricesum /= gcdsum;
	masssum /= gcdsum;

	cout << pricesum << "/" << masssum;

	//system("pause");
}