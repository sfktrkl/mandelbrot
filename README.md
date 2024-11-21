# Mandelbrot Renderer

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

### Build and Run the Program

```bash
cargo run
```
