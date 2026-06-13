# Mini Grep

Mini Grep is a small Rust project for learning the language through a practical command-line search tool.

It demonstrates:

- reading files with buffered I/O
- recursively searching directories
- returning structured search results
- propagating I/O errors with `Result`
- writing unit tests with temporary files

## Usage

```sh
cargo run -- <needle> <path>
```

Example:

```sh
cargo run -- needle src/lib.rs
```

Matches are printed as:

```text
path:line_number: line_content
```

## Tests

```sh
cargo test
```

## License

This project is licensed under the MIT License.
