#include <iostream>

int main() {
    std::string * s = new std::string("Hello");
    delete s;
    std::cout << *s << std::endl;
}