# Hindmarsh Rust
A workspace that contains the create programs and libraries to execute some operations over the Hindmarsh Rose Model.

## Dependences
For building and running this project, you will need to download [Rust](https://rust-lang.org/tools/install/).

## Usage

### Hindmarsh-Rose Rust
Run the following line to see the different command, arguments and defaults:
```bash
cargo run --release -p hindmarsh-rose-rs -- -h
```

For each command you can run:
```bash
cargo run --release -p hindmarsh-rose-rs -- [COMMAND] -h
```

### Hindmarsh-Rose Analyzer Rust
Run the following command to see the different arguments and defaults:
```bash
cargo run --release -p hindmarsh-rose-analyzer-rs -- -h
```

>[!WARNING]
>If you send SIGINT to one of the programs, the fifo won't be deleted, delete it yourself (default names have path like: `/tmp/hindmarsh-rust-*`) or execute one time the other end that didn't received the signal (reader fifos are cleaned on exit too).

### Execute Hidmarsh Rose Analysis on `e` parameter 

For executing the model, run:
```bash
cargo run --release -p hindmarsh-rose-rs -- -e 3.281 -dr 50 -g 25000 analysis -wop 
```

Then on another terminal, execute the analyzer, that will communicate with the model through the fifo implemented on the  `model-data-io` library.
```bash
cargo run --release -p hindmarsh-rose-analyzer-rs
```

When it finishes, `hindmarsh-rose-analyzer-rs` will show on stdout the optimal `e` value founded. The evolution of the model will be written on `data/hindmarsh-rose.csv`.
