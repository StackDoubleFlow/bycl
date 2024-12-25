
static inline void write(unsigned int data, unsigned int column) {
  int volatile *data_ptr = (int *) 0x80000000;
  int volatile *col_ptr = (int *) 0x80000004;
  *data_ptr = data;
  *col_ptr = column;
}

void entry() {
  const int radius = 16;
  const int x_pos = 15;
  const int y_pos = 15;

  for (int col = 0; col < 32; col++) {
    unsigned int data = 0;
    for (int row = 0; row < 32; row++) {
      int a = row - y_pos;
      int b = col - x_pos;
      if (a * a + b * b < radius * radius) {
        data |= (1 << row);
      }
    }
    write(data, col);
  }
}
