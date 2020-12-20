#include <iostream>
#include <vector>
#include <cmath>

int main() {
	int n;
	int d;
	std::cin >> n >> d;

	double r = d/2.0;
	double a2 = 0;
	std::vector<int> measures(n, 0);
	for (int i = 0; i < n; ++i) {
		std::cin >> measures[i];
		a2 += measures[i];
	}

	for (int i = 0; i < (n-1)/2; ++i) {
		double a1 = measures[i];
		a2 -= 2*a1;
		double r1 = std::sqrt(r*r-a1*a2-a1*a1);
		std::cout << std::fixed;
		std::cout.precision(3);
		std::cout << r1 << " ";

		r -= r-r1;
	}
}