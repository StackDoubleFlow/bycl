
static inline void write(unsigned int data, unsigned int column) {
  int volatile *data_ptr = (int *)0x80000000;
  int volatile *col_ptr = (int *)0x80000004;
  *data_ptr = data;
  *col_ptr = column;
}

void entry() {
  const unsigned int pattern = 0x55555555;
  for (int col = 0; col < 32; col++) {
    unsigned int data = pattern;
    if ((col & 1) == 0) {
      data <<= 1;
    }
    write(data, col);
  }
}
