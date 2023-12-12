#include <thread>
#include <iostream>
#include <atomic>

int main() {
    std::atomic_int counter = 0;
    std::thread t1([&counter]() {
        for (int i = 0; i < 1000000; ++i) {
            ++counter;
        }
    });
    std::thread t2([&counter]() {
        for (int i = 0; i < 1000000; ++i) {
            ++counter;
        }
    });
    t1.join();
    t2.join();

    std::cout << counter << std::endl;

    return 0;
}