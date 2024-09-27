#include <stdio.h>
#include <math.h>
float factorial(float n);
typedef struct FloatClass {float value;};
float factorial(float n) {float result=1.0;for(int i=1;i<=n;i++) {result*=i;}return result;}
float addFloatClassToFloat(FloatClass f, float b) {return f.value+b;}
float addFloatClassToInt(FloatClass f, int b) {return f.value+b;}
float addFloatClassToChar(FloatClass f, char b) {return f.value+b;}
float addFloatClassToShort(FloatClass f, short b) {return f.value+b;}
float addFloatClassToDouble(FloatClass f, double b) {return f.value+b;}
float subFloatClassToFloat(FloatClass f, float b) {return f.value-b;}
float subFloatClassToInt(FloatClass f, int b) {return f.value-b;}
float subFloatClassToChar(FloatClass f, char b) {return f.value-b;}
float subFloatClassToShort(FloatClass f, short b) {return f.value-b;}
float subFloatClassToDouble(FloatClass f, double b) {return f.value-b;}
float divFloatClassToFloat(FloatClass f, float b) {return f.value/b;}
float divFloatClassToInt(FloatClass f, int b) {return f.value/b;}
float divFloatClassToChar(FloatClass f, char b) {return f.value/b;}
float divFloatClassToShort(FloatClass f, short b) {return f.value/b;}
float divFloatClassToDouble(FloatClass f, double b) {return f.value/b;}
float multFloatClassToFloat(FloatClass f, float b) {return f.value*b;}
float multFloatClassToInt(FloatClass f, int b) {return f.value*b;}
float multFloatClassToChar(FloatClass f, char b) {return f.value*b;}
float multFloatClassToShort(FloatClass f, short b) {return f.value*b;}
float multFloatClassToDouble(FloatClass f, double b) {return f.value*b;}
float sqrtFloatClass(FloatClass f) {return sqrt(f.value);}
float powFloatClassToFloat(FloatClass f, float b) {return pow(f.value, b);}
float powFloatClassToInt(FloatClass f, int b) {return pow(f.value, b);}
float powFloatClassToChar(FloatClass f, char b) {return pow(f.value, b);}
float powFloatClassToShort(FloatClass f, short b) {return pow(f.value, b);}
float powFloatClassToDouble(FloatClass f, double b) {return pow(f.value, b);}
float inverseFloatClass(FloatClass f) {return 1.0/f.value;}
/**
** liefezpuebfuzs
*/
int main() {FloatClass f1;f1.value=5.0;float result;result=addFloatClassToFloat(f1, 3.0);printf("Add FloatClass to float: %.2f\n", result);result=addFloatClassToInt(f1, 2);printf("Add FloatClass to int: %.2f\n", result);result=addFloatClassToChar(f1, 'a');printf("Add FloatClass to char: %.2f\n", result);result=addFloatClassToShort(f1, 10);printf("Add FloatClass to short: %.2f\n", result);result=addFloatClassToDouble(f1, 2.5);printf("Add FloatClass to double: %.2f\n", result);result=subFloatClassToFloat(f1, 3.0);printf("Subtract float from FloatClass: %.2f\n", result);result=subFloatClassToInt(f1, 2);printf("Subtract int from FloatClass: %.2f\n", result);result=subFloatClassToChar(f1, 'a');printf("Subtract char from FloatClass: %.2f\n", result);result=subFloatClassToShort(f1, 10);printf("Subtract short from FloatClass: %.2f\n", result);result=subFloatClassToDouble(f1, 2.5);printf("Subtract double from FloatClass: %.2f\n", result);result=divFloatClassToFloat(f1, 3.0);printf("Divide FloatClass by float: %.2f\n", result);result=divFloatClassToInt(f1, 2);printf("Divide FloatClass by int: %.2f\n", result);result=divFloatClassToChar(f1, 'a');printf("Divide FloatClass by char: %.2f\n", result);result=divFloatClassToShort(f1, 10);printf("Divide FloatClass by short: %.2f\n", result);result=divFloatClassToDouble(f1, 2.5);printf("Divide FloatClass by double: %.2f\n", result);result=multFloatClassToFloat(f1, 3.0);printf("Multiply FloatClass by float: %.2f\n", result);result=multFloatClassToInt(f1, 2);printf("Multiply FloatClass by int: %.2f\n", result);result=multFloatClassToChar(f1, 'a');printf("Multiply FloatClass by char: %.2f\n", result);result=multFloatClassToShort(f1, 10);printf("Multiply FloatClass by short: %.2f\n", result);result=multFloatClassToDouble(f1, 2.5);printf("Multiply FloatClass by double: %.2f\n", result);result=sqrtFloatClass(f1);printf("Square root of FloatClass: %.2f\n", result);result=powFloatClassToFloat(f1, 3.0);printf("FloatClass raised to the power of float: %.2f\n", result);result=powFloatClassToInt(f1, 2);printf("FloatClass raised to the power of int: %.2f\n", result);result=powFloatClassToChar(f1, 'a');printf("FloatClass raised to the power of char: %.2f\n", result);result=powFloatClassToShort(f1, 10);printf("FloatClass raised to the power of short: %.2f\n", result);result=powFloatClassToDouble(f1, 2.5);printf("FloatClass raised to the power of double: %.2f\n", result);result=inverseFloatClass(f1);printf("Inverse of FloatClass: %.2f\n", result);return 0;}
