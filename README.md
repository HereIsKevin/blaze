# Blaze 🔥

Nearly all modern programming languages contain too much syntax and too many
features. On the other hand, simple programming languages like C and Go lack
enough features or safety to make programming pleasant.

Blaze seeks to **strike the right balance** between simplicity, safety, and
ease-of-use. Built on Rust, Blaze has guaranteed memory safety. With its simple
but intuitive syntax, Blaze is much more pleasant to use than C.

By compiling down to native code, Blaze is as fast as C most of the time, and
sometimes even faster.

### Getting Started

Before attempting to compile Blaze, download Rust first, preferably through
[rustup](https://rustup.rs/).

First, download the source code of Blaze from GitHub.

```bash
git clone https://github.com/HereIsKevin/blaze.git
```

After entering the directory with `cd`, compile Blaze.

```bash
cargo build --release
```

The executable file for Blaze should be `target/release`, called either `blaze`
or `blaze.exe`.

As you first program, create a file called `hello.blz` and type the following
program in it.

```rust
fn main() {
    print("Hello, world!")
}
```

After creating your first program, compile it with Blaze.

```bash
./target/release/blaze ./hello.blz ./hello
```

This compiles the program and creates an executable called `hello`. The
generated Rust code can be found at `hello.rs`.

### Next Steps

Blaze is very, very young and relatively unstable. Currently, there are only two
built-in functions.

  - `fn clock() -> f64`: Returns the number of seconds since the epoch as `f64`.
  - `fn print(value: ?)`: Takes any value an prints it to standard output.

Here's an example program displaying all of Blaze's features.

```rust
// function types don't quite work yet...

type TypeAlias = fn(i32, i32): i32

fn inner(): f64 {
    return 10.10
}

fn outer(): fn(): f64 {
    return inner
}

fn main() {
    loop {
        if false != false {
            // this is pointless
            break
        } else {
            if true {
                // else if is still not available yet
            }
        }

        if true != true {
            // this is also pointless
            continue
        } else {
            // nothing here
        }
    }
}
```

Currently, there are only a handful of built-in types.

  - `i32`: 32-bit integer
  - `f64`: 64-bit float
  - `bool`: Boolean
  - `fn(...) -> ...`: Functions types, don't quite work yet
