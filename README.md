# rust-horoscope

Horscope random generator based on Журнал Код (Journal Yandex Praktikuma) «Пишем гороскоп на Rust».


## Execute

Rust must be installed in the computer.
Once installed, run
```bash
cargo run
```
or
```bash
cargo build
./target/debug/rust-horoscope
```

## Special dependecies

In this little functionalities example we used the `rand` library.
It is a library for random numbers.
It is important to add it to the project Cargo.toml dependencies to download it.

```toml
[dependencies]
rand = "0.8.5"
```