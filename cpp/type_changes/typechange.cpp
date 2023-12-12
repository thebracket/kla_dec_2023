#include <iostream>

int double_it(long n) {
    return n * 2;
}

int main() {
    int i = 5;
    int j = double_it(i);
    std::cout << "i = " << i << ", j = " << j << std::endl;

    // Just for fun
    std::cout << double_it(2147483648) << std::endl;
    return 0;
}