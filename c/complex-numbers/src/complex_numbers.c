#include "complex_numbers.h"

complex_t c_add(complex_t a, complex_t b)
{
   return (complex_t) {a.real+b.real, a.imag+b.imag};
}

complex_t c_sub(complex_t a, complex_t b)
{
   return (complex_t) {a.real-b.real, a.imag-b.imag};
}

complex_t c_mul(complex_t a, complex_t b)
{
   return (complex_t) {
      a.real*b.real - a.imag*b.imag,
      a.real*b.imag + a.imag*b.real
   };
}

complex_t c_div(complex_t a, complex_t b)
{
   double b2 = b.real*b.real + b.imag*b.imag;

   return c_mul(a, (complex_t) {b.real/b2, -b.imag/b2}); 
}

double c_abs(complex_t x)
{
   return sqrt(c_mul(x, c_conjugate(x)).real);
}

complex_t c_conjugate(complex_t x)
{
   return (complex_t) {x.real, -x.imag};
}

double c_real(complex_t x)
{
   return x.real;
}

double c_imag(complex_t x)
{
   return x.imag;
}

complex_t c_exp(complex_t x)
{
   double expa = exp(x.real);
   return (complex_t) {expa*cos(x.imag), expa*sin(x.imag)};
}
