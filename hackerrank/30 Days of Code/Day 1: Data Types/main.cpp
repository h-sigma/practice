#include <cmath>
#include <cstdio>
#include <iostream>
#include <iomanip>
using namespace std;

int main() {
    int i = 4;
    double d = 4.0;
    string s = "HackerRank ";

    
    int iInput;
    double dInput;
    string sInput;
    
    std::cin >> iInput;
    std::cin >> dInput;
    std::cin.ignore(1, '\n'); // lesson 1: ignore the newline character after std::cin is read
    getline(std::cin, sInput); // lesson 2: use getline to read a full line including spaces
    
    std::cout << i + iInput << std::endl;
    std::cout << std::fixed << std::setprecision(1) << d + dInput << std::endl; // lesson 3: set precision for double output
    std::cout << s << sInput; // lesson 4: no need to concatenate with + operator, just output sequentially
    
    return 0;
}
