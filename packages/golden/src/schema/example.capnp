@0xde5c62fa71be02c0;

enum StandardPaperSize {
  usLetter @0;
  a4 @1;
}

struct CustomPaperSize {
  width @0: UInt16;
  height @1: UInt16;
}

struct PaperSize {
  union {
    standard @0: StandardPaperSize;
    custom @1: CustomPaperSize;
  }
}

