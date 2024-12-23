
void entry() {
  int volatile *a_ptr = (int *)0x80000000 + 15;
  int volatile *b_ptr = (int *)0x80000000 + 14;
  *a_ptr = *a_ptr * *b_ptr;
}
