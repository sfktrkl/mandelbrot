# Mandelbrot Set Renderer

![Mandelbrot Set](mandelbrot_set.png)

### Rust Installation

Install [Rust](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:

```bash
rustc --version
```

### Dependencies

Rust package manager will automatically download the dependencies.

- [minifb](https://crates.io/crates/minifb): Window setup.
  ```bash
  cargo add minifb
  ```
- [rayon](https://crates.io/crates/rayon): Work-stealing parallelism.
  ```bash
  cargo add rayon
  ```

### Build and Run the Program

```bash
cargo run
```
