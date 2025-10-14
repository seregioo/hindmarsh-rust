# Hindmarsh Rust
A workspace that contains the create programs and libraries to execute some operations over the Hindmarsh Rose Model.

## Dependencies
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
cargo run --release -p hindmarsh-rose-rs -- --e 3.281 --downsample-rate 50 --goal 25000 --write-on-pipe --runge-kutta analysis  
```

Then on another terminal, execute the analyzer, that will communicate with the model through the fifo implemented on the  `model-data-io` library.
```bash
cargo run --release -p hindmarsh-rose-analyzer-rs
```

When it finishes, `hindmarsh-rose-analyzer-rs` will show on stdout the optimal `e` value founded. The evolution of the model will be written on `data/hindmarsh-rose.csv`.


### Execute Hidmarsh Rose monodirectional synapse 

For executing a monodirectional synapse, three programs are needed, the first two are the two neuron models (Hindmarsh-Rose in this case):

```bash
cargo run --release -p hindmarsh-rose-rs -- --downsample-rate  50 --goal 25000 --write-fifo-path '/tmp/hindmarsh-rust-electrical-syn-voltage' --read-fifo-path '/tmp/hindmarsh-rust-electrical-syn-current' --runge-kutta synapse --postsynaptics 0
```


```bash
cargo run --release -p hindmarsh-rose-rs -- --downsample-rate 50 --goal 25000 --write-fifo-path '/tmp/hindmarsh-rust-electrical-syn-voltage' --read-fifo-path '/tmp/hindmarsh-rust-electrical-syn-current' --runge-kutta synapse --presynaptics 0
```

The other program needed is an electrical synapse:
```bash
argo run --release -p electrical-synapse-rs 
```

### Execute Hidmarsh Rose bidirectional synapse 

For bidirectional synapse, are needed 2 synapse models, and their ids needs to be indicated on the Hindmarsh-Rose model arguments: 
```bash
cargo run --release -p hindmarsh-rose-rs -- --downsample-rate 50 -g 25000 --write-fifo-path '/tmp/hindmarsh-rust-electrical-syn-voltage' --read-fifo-path '/tmp/hindmarsh-rust-electrical-syn-current' --runge-kutta synapse --postsynaptics 0 --presynaptics 1
```

```bash
cargo run --release -p hindmarsh-rose-rs -- --downsample-rate 50 -g 25000 --write-fifo-path '/tmp/hindmarsh-rust-electrical-syn-voltage' --read-fifo-path '/tmp/hindmarsh-rust-electrical-syn-current' --runge-kutta synapse --postsynaptics 1 --presynaptics 0 
```

In this case is on antiphase:
```bash
cargo run --release -p electrical-synapse-rs -- --g-fast -0.44
```

The id needs to be indicated, and its recommended to change the output file:
```bash
cargo run --release -p electrical-synapse-rs -- --g-fast -0.44 --synapse-id 1 --filename hindmarsh-rose-syn-1.csv
```


## TODO:
Improve temporal scale (ensure same points for each model, maybe with a channel to send the data to thread). They have a temporal "lag" because the time for reading, writing and calculating,
