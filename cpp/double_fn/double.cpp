#include <iostream>

int double_it(int x) {
    return x * 2;
}

int main() {
    auto i = 5;
    auto j = double_it(i);
    std::cout << i << " * 2 = " << j << std::endl;
    return 0;
}