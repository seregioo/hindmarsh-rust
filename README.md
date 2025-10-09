# Hindmarsh Rust
A workspace that contains the create programs and libraries to execute some operations over the Hindmarsh Rose Model.

## Dependences
For building and running this project, you will need to download [Rust](https://rust-lang.org/tools/install/).

## Usage

### Hidmarsh Rose Analysis on `e` parameter 

For executing the model, run:
```bash
cargo run --release -p hindmarsh-rose-rs -- -wop -e 3.281 -dr 50 -g 25000
```

Then on another terminal, execute the analyzer, that will communicate with the model through the fifo implemented on the  `model-data-io` library.
```bash
cargo run --release -p hindmarsh-rose-analyzer-rs
```

When it finishes, `hindmarsh-rose-analyzer-rs` will show on stdout the optimal `e` value founded. The evolution of the model will be written on `data/hindmarsh-rose.csv`.
