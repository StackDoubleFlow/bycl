#define MAX_ITERATION 32
#define FIXED_POINT 13

static inline void write(unsigned int data, unsigned int column) {
  int volatile *data_ptr = (int *) 0x80000000;
  int volatile *col_ptr = (int *) 0x80000004;
  *data_ptr = data;
  *col_ptr = column;
}

int mul(int a, int b) {
  return (a * b) >> FIXED_POINT;
}

[[gnu::naked]] void entry() {
  // Just don't worry about it
  asm volatile ("add sp, sp, -16");

  // Magic values :)
  int offset_x = -5677;
  int offset_y = 3664;
  int scale = 90;
  int four_fixed = 4 << FIXED_POINT;

  for (int px = 0; px < 32; px++) {
    int col = 0;
    for (int py = 0; py < 32; py++) {
      int x0 = mul(px << FIXED_POINT, scale) + offset_x;
      int y0 = mul(py << FIXED_POINT, scale) + offset_y;

      int x = 0;
      int y = 0;
      int x2 = 0;
      int y2 = 0;

      int iteration = 0;

      while (x2 + y2 <= four_fixed && iteration < MAX_ITERATION) {
        y = (mul(x, y) << 1) + y0;
        x = x2 - y2 + x0;
        x2 = mul(x, x);
        y2 = mul(y, y);
        iteration += 1;
      }

      if (iteration >= (MAX_ITERATION / 2)) {
        col |= (1 << py);
      }
    }
    write(col, px);
  }
}