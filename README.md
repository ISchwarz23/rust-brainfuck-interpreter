# Rust Brainfuck Interpreter

Brainfuck is an esoteric programming language. For the purpose of getting into Rust, I implemented an interpreter for the Brainfuck language.


## Build and run
To build the Brainfuck interpreter run:
```sh
cargo build --release
```

Afterwards the program can be ported anywhere and executed using:
```sh
./bfi <bf-file-or-code>
```


## Usage
Here are some usage examples on how to run the Brainfuck interpreter:
```sh
./bfi hello-world.bf        # will interpret hello-world.bf
./bfi ,>,<[>+<-]            # will interpret the passed bf code
```


## Build and Run for wasi
TODO