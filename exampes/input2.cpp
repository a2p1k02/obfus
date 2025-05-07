#include <iostream>
#include <string>

std::string test_func() {
    std::string test_string = "hello world";

    std::cout << test_string << "\n";
}

int main() {
    test_func();
    return 0;
}
