# obfus

This is a small program to obfuscate C++ code

## Example
### Before: 
```cpp
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
```

### After 
```cpp
#ifdef __GNUC__
#define __UNUSED __attribute__((unused))
#else
#define __UNUSED
#endif

template<typename T>
__UNUSED static inline void ___func_6346() {
volatile T ___var_6346 = (T)0x7832;
___var_6346 += (T)0x7832;
}
    
#include <iostream>
#include <string>

std::string G_OfbrDWq() {
    std::string iC0i0bx6gKxt = ""\x68\x65\x6c\x6c\x6f\x20\x77\x6f\x72\x6c\x64";

    std::cout << iC0i0bx6gKxt << ""\\x6e";
}

int main() {
    G_OfbrDWq();
    return 0x0;
}
```
