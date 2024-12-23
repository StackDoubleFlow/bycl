
static inline void write(unsigned int data, unsigned int column) {
  int volatile *data_ptr = (int *) 0x8000000;
  int volatile *col_ptr = (int *) 0x8000004;
  *data_ptr = data;
  *col_ptr = column;
}

void entry() {
  const float radius = 16;
  const float x_pos = 20;
  const float y_pos = 15;

  for (int col = 0; col < 32; col++) {
    unsigned int data = 0;
    for (int row = 0; row < 32; row++) {
      float a = row - y_pos;
      float b = col - x_pos;
      if (a * a + b * b < radius * radius) {
        data |= (1 << row);
      }
    }
    write(data, col);
  }
}
