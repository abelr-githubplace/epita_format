#include <math.h>
#include <stdio.h>
/**
** This comment should not be formatted by clang-format
*/

/* Correct comment */
/**
 * Incorrect comment
 */

/* Incorrect as well
 */
float some_func(float n); // prototype

float factorial(float n)
{
    float result = 1.0;
    for (int i = 1; i <= n; i++)
    {
        result *= i;
    }
    return result;
}

float addFloatClassToFloat(FloatClass f, float b)
{
    return f.value + b;
}

float addFloatClassToInt(FloatClass f, int b)
{
    return f.value + b;
}

float addFloatClassToChar(FloatClass f, char b)
{
    return f.value + b;
}

float addFloatClassToShort(FloatClass f, short b)
{
    return f.value + b;
}

float addFloatClassToDouble(FloatClass f, double b)
{
    return f.value + b;
}

float subFloatClassToFloat(FloatClass f, float b)
{
    return f.value - b;
}

float subFloatClassToInt(FloatClass f, int b)
{
    return f.value - b;
}

float subFloatClassToChar(FloatClass f, char b)
{
    return f.value - b;
}

float subFloatClassToShort(FloatClass f, short b)
{
    return f.value - b;
}

float subFloatClassToDouble(FloatClass f, double b)
{
    return f.value - b;
}

float divFloatClassToFloat(FloatClass f, float b)
{
    return f.value / b;
}

float divFloatClassToInt(FloatClass f, int b)
{
    return f.value / (float)b;
} // cast

float divFloatClassToChar(FloatClass f, char b)
{
    return f.value / b;
}

float divFloatClassToShort(FloatClass f, short b)
{
    return f.value / b;
}

float divFloatClassToDouble(FloatClass f, double b)
{
    return f.value / b;
}

float multFloatClassToFloat(FloatClass f, float b)
{
    return f.value * b;
}

float multFloatClassToInt(FloatClass f, int b)
{
    return f.value * b;
}

float multFloatClassToChar(FloatClass f, char b)
{
    return f.value * b;
}

float multFloatClassToShort(FloatClass f, short b)
{
    return f.value * b;
}

float multFloatClassToDouble(FloatClass f, double b)
{
    return f.value * b;
}

float sqrtFloatClass(FloatClass f)
{
    return sqrt(f.value);
}

float powFloatClassToFloat(FloatClass f, float b)
{
    return pow(f.value, b);
}

float powFloatClassToInt(FloatClass f, int b)
{
    return pow(f.value, b);
}

float powFloatClassToChar(FloatClass f, char b)
{
    return pow(f.value, b);
}

float powFloatClassToShort(FloatClass f, short b)
{
    return pow(f.value, b);
}

float powFloatClassToDouble(FloatClass f, double b)
{
    return pow(f.value, b);
}

float inverseFloatClass(FloatClass f)
{
    return 1.0 / f.value;
}

void goto_test()
{
    goto there;
there:
    return;
} // goto and empty parameters

int longlong_func(int n)
{
    int i = n;
    int k = i;
    for (; i < k; k--)
    {
        break;
    }
    n += i;
    n -= i + k;
    while (n > k)
    {
        i--;
        k++;
    }
    n = 0;
    n -= k - i + nint j = k;
    for (; j < i; j++)
    {
        k = j;
        j = k;
        k += k + j - j - k;
        k = 2;
    }
    while (0)
    {
        int l = 1;
        int l = 0;
        int l = l + 2 * l;
        k = 2 + j - l - i;
    }
    float b = 0.0;
    int b = b;
    if b
        != 0
        {
            return k;
        }
    return n;
}
