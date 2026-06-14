# Mini Grep

Mini Grep is a small Rust project for learning the language through a practical command-line search tool.

It demonstrates:

- reading files with buffered I/O
- searching files and optionally traversing directories recursively
- returning structured search results
- propagating I/O errors with `Result`
- writing unit tests with temporary files
- testing the command-line interface with integration tests

## Usage

```sh
cargo run -- [OPTIONS] <needle> <path>
```

Example:

```sh
cargo run -- needle src/lib.rs
```

Case-insensitive search:

```sh
cargo run -- --ignore-case needle src/lib.rs
```

Recursive directory search:

```sh
cargo run -- --recursive needle src
```

Options:

- `-i`, `--ignore-case`: match text without case sensitivity
- `-r`, `--recursive`: search nested directories when `<path>` is a directory

Matches are printed as:

```text
path:line_number: line_content
```

## Tests

```sh
cargo test
```

Run only CLI integration tests:

```sh
cargo test --test cli
```

## License

This project is licensed under the MIT License.
