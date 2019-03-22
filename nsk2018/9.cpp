#include <iostream>
int main() {
	int m, k, n;
	std::cin >> m >> k >> n;
	std::cout << (m/k + ((m%k==0)?0:1))*n;
}