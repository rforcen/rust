// compile w/
// g++ main.cpp -O2 -o pidecs -I.

#include <PiDecimals.h>

int main() {
    PiDecimals piDec;
    int ndec=1000;

    piDec.getDecimals(ndec);
    auto pi = piDec.generateBytes();
    for (auto i=0; i<ndec; i++) printf("%c", pi[i]+'0');

    return 0;
}