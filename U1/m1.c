
#include <float.h>
#include <math.h>
/* #include <stdlib.h> */

// TODO: use #define instead!
const float EPSI = 1e-8;

float numerical_der(float (*f)(float), float x) {
  const float h = x < 0 ? -FLT_MAX : x + (-FLT_MAX);
  return ((f(x + h) - f(x)) / h);
}

// assumes that the interval already have a signal change
bool teste_fuleiro(float a, float b, float (*f)(float)) {
  if (numerical_der(f, a) * numerical_der(f, b) < 0)
    return false;
  if (numerical_der(f, a) * numerical_der(f, (b + a) / 2) < 0)
    return false;
  return true;
}

float method0(float a, float b, float (*f)(float)) {
  float x = (b + a) / 2;
  float fx = f(x);

  // CASO JA CONVERGIU
  if (fabs(fx) < EPSI) {
    return x;
  }

  // SE O SINAL MUDOU
  if (f(a) * f(x) < 0) {
    return method0(a, x, f); // b <- x
  } else {
    return method0(x, b, f); // a <- x
  }
}

float sec(float a, float b, float (*f)(float)) {
  return ((a * f(b) - b * f(a)) / f(b) - f(a));
}

float method1(float a, float b, float (*f)(float)) {
  float x = sec(a, b, f);
  // CASO JA CONVERGIU
  if (fabs(f(x)) < EPSI) {
    return x;
  }

  // SE O SINAL MUDOU
  if (f(a) * f(x) < 0) {
    return method0(a, x, f); // b <- x
  } else {
    return method0(x, b, f); // a <- x
  }
}
