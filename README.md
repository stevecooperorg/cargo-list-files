# cargo-list-files

cargo command for listing files used in a rust workspace.

Intended for use in, eg, Makefiles, where the list of source files is needed to detect changes. E.g.

```
SRC_FILES=$(shell cargo list-files path/to/Cargo.toml)
```

# Installation

```
cargo install --git https://github.com/stevecooperorg/cargo-list-files
```