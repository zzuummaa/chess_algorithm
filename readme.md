# Chess algorithm

## Сборка для Linux

Используется nightly версия Rust поэтому при установке необходимо переключиться на нее выбрав пункт 2 в меню установки.

```cmd
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential
https://github.com/zzuummaa/chess_algorithm.git
cd chess_algorithm
cargo build --color=always --release --package chess_algorithm --bin chess_algorithm
```

После этого должен будет появиться исполяемый файл `target\release\chess_algorithm`.