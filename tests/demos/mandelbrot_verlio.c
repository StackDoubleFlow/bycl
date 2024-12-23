#ifdef __BYCL__

static inline void write(unsigned int data, unsigned int column) {
  int volatile *data_ptr = (int *) 0x80000000;
  int volatile *col_ptr = (int *) 0x80000004;
  *data_ptr = data;
  *col_ptr = column;
}

#else

#include <stdio.h>

unsigned char display[32][32];

static inline void write(unsigned int data, unsigned int column) {
  for (int i = 0; i < 32; ++i) {
    display[i][column] = ((data & 0x80000000) != 0);
    data <<= 1;
  }
}

#endif

static inline unsigned int sqr(short x) { // code totally not copied from kuggo
  if (x < 0)
    x = -x;
  unsigned int out = 0;
  unsigned char i = 0;
  while (x > 0) {
    if (x & 1) {
      out += (((x >> 1) << 2) | 1) << i;
    }
    i += 2;
    x >>= 1;
  }
  return out;
}

int main() {
  short x = -(3 << 11);
  const short inc = 346;
  short y;

  for (int column = 0; column < 32; ++column) {
    unsigned int final = 0;
    y = inc;
    for (int row = 0; row < 10;) {
      short resr = x;
      short resi = y;
      unsigned int result = 0x10000;
      for (int i = 0; i < 64; ++i) {
        short tmp = (((int)resr * (int)resi) >> 12) << 1;
        resr = (sqr(resr) >> 12) - (sqr(resi) >> 12) + x;
        resi = tmp + y;
        if (resr + resi >= (2 << 12)) {
          result = 0;
          break;
        }
      }
      final += result << row;
      final += result >> (++row);
      y += inc;
    }
    write(final, column);
    x += inc;
  }

#ifndef __BYCL__
  for (int i = 0; i < 32; ++i) {
    for (int j = 0; j < 32; ++j) {
      printf("%1$c%1$c", display[i][j] ? '#' : ' ');
    }
    printf("\n");
  }
#endif

  return 0;
}

#ifdef __BYCL__

void entry() {
  main();
}

#endif