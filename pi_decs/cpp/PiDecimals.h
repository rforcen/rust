#ifndef PiDecimalsH
#define PiDecimalsH
#include <string>
#include <stdarg.h>

typedef unsigned char byte;
using std::string;

class PiDecimals
{

    int *term, *sum, firstword, words;
    int ndigits, ib;
    byte *byDigs;

public:
    byte *getDecimals(int digits)
    { // 0..9 per byte
        generateDecs(digits);
        return generateBytes();
    }

    byte *getCents(int digits)
    { // 0..9 per byte
        generateDecs(digits);
        return generateCentBytes();
    }

    byte *getAlsoCents(int digits)
    { // 00..99 per byte but MUST getDecimals first
        return generateCentBytes();
    }

    void generateDecs(int digits)
    {
        int remainder, denom, x;
        ndigits = digits;

        // Allocate array space and initialize
        words = digits / 4 + 3;
        sum = new int[words + 2];
        term = new int[words + 2];

        // -- 32*atan(1/10) -
        denom = 3;
        sum[1] = 32;

        for (firstword = 2; firstword <= words; firstword++)
        {
            atan10(denom);
            denom += 4;
        }
        // -- -4*atan(1/239) -
        firstword = 2;
        denom = 3;
        remainder = 40;

        for (x = 2; x <= words; x++)
        {
            digits = (int)remainder * 10000;
            term[x] = digits / 239; // first term
            remainder = digits % 239;
            sum[x] -= term[x];
        }

        while (firstword < words)
        {
            atan239(denom);
            denom += 4;
        }
        // -- -16*atan(1/515) -
        firstword = 2;
        denom = 3;
        remainder = 160;

        for (x = 2; x <= words; x++)
        {
            digits = (int)remainder * 10000;
            term[x] = digits / 515; // first term
            remainder = digits % 515;
            sum[x] -= term[x];
        }

        while (firstword < words)
        {
            atan515(denom);
            denom += 4;
        }

        for (x = words; x >= 1; x--)
        { // release carries & borrows
            if (sum[x] < 0)
            {
                sum[x - 1] += sum[x] / 10000;
                sum[x] = sum[x] % 10000;
                sum[x - 1]--;
                sum[x] += 10000;
            }
            if (sum[x] >= 10000)
            {
                sum[x - 1] += sum[x] / 10000;
                sum[x] = sum[x] % 10000;
            }
        }
    }

    void atan10(int denom)
    {
        int remainder1, remainder2;
        int dividend, denom2 = denom + 2;

        sum[firstword] -= 3200 / denom;
        remainder1 = 3200 % denom;

        sum[firstword] += 32 / denom2;
        remainder2 = 32 % denom2;

        for (int x = firstword + 1; x <= words; x++)
        {
            dividend = (int)remainder1 * 10000;
            sum[x] -= dividend / denom;
            remainder1 = dividend % denom;

            dividend = (int)remainder2 * 10000;
            sum[x] += dividend / denom2;
            remainder2 = dividend % denom2;
        }
    }

    void atan239(int denom)
    {
        int remainder1 = term[firstword++], // perform 1st divide implicitly
            remainder2 = 0, remainder3 = 0, remainder4 = 0;
        int dividend, denom2 = denom + 2, temp, x;

        for (x = firstword; x <= words; x++)
        {
            temp = term[x];

            dividend = (int)remainder1 * 10000 + temp; // add next term
            temp = dividend / 57121;
            remainder1 = dividend % 57121;

            dividend = (int)remainder2 * 10000 + temp;
            sum[x] += dividend / denom;
            remainder2 = dividend % denom;

            dividend = (int)remainder3 * 10000 + temp; // subtract next term
            temp = dividend / 57121;
            remainder3 = dividend % 57121;

            dividend = (int)remainder4 * 10000 + temp;
            sum[x] -= dividend / denom2;
            remainder4 = dividend % denom2;
            term[x] = temp;
        }

        firstword++;
        if (term[firstword] == 0)
            firstword++;
    }

    void atan515(int denom)
    {
        int remainder1 = term[firstword++], // perform 1st divide implicitly
            remainder2 = 0, remainder3 = 0, x, remainder4 = 0, dividend,
            denom2 = denom + 2, temp;

        for (x = firstword; x <= words; x++)
        {
            temp = term[x];
            if (remainder1 < 214745)
            {
                dividend = remainder1 * 10000 + temp; // add next term
                temp = dividend / 265225;
                remainder1 = dividend % 265225;
            }
            else
            {
                remainder1 -= 53045;
                dividend = remainder1 * 10000 + temp;
                temp = dividend / 265225;
                remainder1 = dividend % 265225;
                temp += 2000;
            }

            dividend = remainder2 * 10000 + temp;
            sum[x] += dividend / denom;
            remainder2 = dividend % denom;

            if (remainder3 < 214745)
            { // subtract next term
                dividend = remainder3 * 10000 + temp;
                temp = dividend / 265225;
                remainder3 = dividend % 265225;
            }
            else
            {
                remainder3 -= 53045;
                dividend = remainder3 * 10000 + temp;
                temp = dividend / 265225;
                remainder3 = dividend % 265225;
                temp += 2000;
            }

            dividend = remainder4 * 10000 + temp;
            sum[x] -= dividend / denom2;
            remainder4 = dividend % denom2;
            term[x] = temp;
        }

        firstword++;
        if (term[firstword] == 0)
            firstword++;
    }

    byte *generateBytes()
    {
        byDigs = new byte[ndigits + 1];
        ib = 0;
        addBytes("0314");
        for (int i = 2; i < words; i += 3)
        {
            addBytes(asprintf("%04d%04d%04d", sum[i], sum[i + 1], sum[i + 2]));
        }
        for (int i = 3 * (words / 3) + 2; i < words; i++)
        {
            addBytes(asprintf("%04d", sum[i]));
        }
        return byDigs;
    }

    void addBytes(string s)
    {
        for (int i = 0; i < s.size(); i++)
            if (ib < ndigits + 1)
                byDigs[ib++] = (byte)(s[i + 1] - '0');
    }

    byte *generateCentBytes()
    {
        byDigs = new byte[(ndigits + 1) + 3];
        ib = 0;
        addCentBytes("31");
        for (int i = 2; i < words; i += 3)
        {
            addCentBytes(asprintf("%04d%04d%04d", sum[i], sum[i + 1], sum[i + 2]));
        }
        for (int i = 3 * (words / 3) + 2; i < words; i++)
        {
            addCentBytes(asprintf("%04d", sum[i]));
        }
        return byDigs;
    }

    void addCentBytes(string s)
    {
        for (int i = 0; i < s.size(); i += 2)
            if (ib < ndigits + 1)
                byDigs[ib++] = (byte)((s[i + 1] - '0') * 10 + (s[i + 2] - '0'));
    }

    string asprintf(const char *fmt, ...)
    {
        char *ret;
        va_list ap;

        va_start(ap, fmt);
        auto nb = vasprintf(&ret, fmt, ap);
        va_end(ap);

        string str(ret);
        free(ret);

        return str;
    }
};

#endif
