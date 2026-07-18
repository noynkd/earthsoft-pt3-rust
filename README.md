# earthsoft-pt3-rust

## プロジェクト

- `earthsoft-pt3-example` [Rust]
  - `earthsoft-sdk` を使って PT3-Example-400.exe を実装したもの。
- `earthsoft-pt3-lib` [C++]
  - `EARTH_PT3.h` を C++23 にしたもの。
  - PT3-Example-400.exe を、作成したヘッダーファイルを使用して実装したものも含む。
- `earthsoft-pt3-sys` [C++, Rust]
  - `earthsoft-pt3-lib` を C ABI に展開して、Rust でラップしたもの。
- `earthsoft-sdk` [Rust]
  - `earthsoft-pt3-sys` を使用して Rust で扱えるようにしたもの。
