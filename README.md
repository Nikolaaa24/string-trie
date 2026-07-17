# String-trie: A trie (prefix-tree) for Rust

String-trie is a library that provides a trie data structure for storing, prefix searching and deleting strings.

It includes the ability to limit the output string count.

## Features

- Trie data structure
- Ability to limit results of search
- Store, search and delete strings easily

## Usage

```bash
cargo add string-trie
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
string-trie = "0.1"
```

## Examples

### Storing a string
```rust
use string_trie::Trie;
let mut trie = Trie::new();

trie.insert("hello");
```
### Searching strings by prefix
```rust
use string_trie::{ResultLimit, Trie};

let mut trie = Trie::new();
trie.insert("hello");

let found = trie.search("h", ResultLimit::Unlimited);
```
To limit the amount of items found (this will also speed up query time because it does not have to travel to some parts of the trie):
```rust
use string_trie::{ResultLimit, Trie};

let mut trie = Trie::new();
trie.insert("hello");

let found = trie.search("h", ResultLimit::Limited(10));
```
### Deleting strings
```rust
use string_trie::{ResultLimit, Trie};
let mut trie = Trie::new();
trie.insert("car");
trie.insert("cart");

trie.delete("car");
```
If the node does not have children and is not an end of a string, it will be removed completely, otherwise if the end is reached, it will just mark the end as false (which means that node no longer represents the end of the string).
## License

Licensed under either of:

- Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or https://opensource.org/license/mit)
