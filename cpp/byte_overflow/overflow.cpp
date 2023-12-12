#include <iostream>
#include <cstdint>

int main() {
    uint8_t j = 0;
    for (int i = 0; i < 512; i++) {
        j++;
        std::cout << i << " : " << unsigned(j) << std::endl;
    }
    return 0;
}