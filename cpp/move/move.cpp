#include <string>

void do_it(std::string s) {
    // Do something
}

int main() {
    std::string s = "Hello";
    do_it(std::move(s));
    // s is now in a valid but unspecified state
    return 0;
}