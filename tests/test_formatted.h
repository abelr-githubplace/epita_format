#ifndef TEST_H
#define TEST_H

#include <math.h>
#include <stdio.h>

typedef struct
{
    float value;
} FloatClass; // brace should not be followed by anything but a semicolon or comment and this line is now too long

float factorial(float n);
float addFloatClassToFloat(FloatClass f, float b);
float addFloatClassToInt(FloatClass f, int b);
float addFloatClassToChar(FloatClass f, char b);
float addFloatClassToShort(FloatClass f, short b);
float addFloatClassToDouble(FloatClass f, double b);
float subFloatClassToFloat(FloatClass f, float b);
float subFloatClassToInt(FloatClass f, int b);
float subFloatClassToChar(FloatClass f, char b);
float subFloatClassToShort(FloatClass f, short b);
float subFloatClassToDouble(FloatClass f, double b);
float divFloatClassToFloat(FloatClass f, float b);
float divFloatClassToInt(FloatClass f, int b);
float divFloatClassToChar(FloatClass f, char b);
float divFloatClassToShort(FloatClass f, short b);
float divFloatClassToDouble(FloatClass f, double b);
float multFloatClassToFloat(FloatClass f, float b);
float multFloatClassToInt(FloatClass f, int b);
float multFloatClassToChar(FloatClass f, char b);
float multFloatClassToShort(FloatClass f, short b);
float multFloatClassToDouble(FloatClass f, double b);
float sqrtFloatClass(FloatClass f);
float powFloatClassToFloat(FloatClass f, float b);
float powFloatClassToInt(FloatClass f, int b);
float powFloatClassToChar(FloatClass f, char b);
float powFloatClassToShort(FloatClass f, short b);
float powFloatClassToDouble(FloatClass f, double b);
float inverseFloatClass(FloatClass f);
void goto_test(); // empty parameters

// following line is missing a bang
#endif /* TEST_H */