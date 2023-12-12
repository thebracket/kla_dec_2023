#include <iostream>

struct A {
    int a;
};

A * frobniactor() {
    return nullptr;
}

int main() {
    A * a = frobniactor();
    std::cout << "a is " << a->a << std::endl;
    return 0;
}