#include <iostream>

using namespace std;

int main()
{
	int test1 = 0;
	int test2 = 1;

    int a[70][70][70][70][70]; // 1.6гб

    a[20][10][0][1][1] = 6;
    
    cout << &a[1][0][0][0][0] - &a[0][69][69][69][69] << endl;

    cout << &test2 - &test1 << endl;

    cout << (void*)(&test1) - ((void*)(&a)) << endl;

    return 0;
}


