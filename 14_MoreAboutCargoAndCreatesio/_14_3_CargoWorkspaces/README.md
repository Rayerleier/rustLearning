
# Cargo Workspaces

## Createing a Workspace

```bash
cargo new --lib my_lib
cargo new --bin my_bin
```
A work space is a set of packages that share the same Cargo.lock and output directory. Lets make a project using a workspace-- we'll use trivial code so we can concentrate on the structure of workspace.

```toml
[workspace]
members = ["my_lib", "my_bin"]
```

Next, we'll create the adder binary crate by running `cargo new --bin adder`. within the `my_bin` directory, we'll add the following code:

```rust
mod my_lib;

fn main() {
    println!("{}", my_lib::add(1, 2));
}
```

## Creating the Second Package in the Workspace

Next, lets create another package in the workspacem and call it `my_lib2`.
Change the top level of the `my_lib2` directory to the following:

```toml
[workspace]
members = ["my_lib", "my_bin", "my_lib2"]
```

Then generate a new library crate named `my_lib2` by running `cargo new --lib my_lib2`.

```bash
cargo new --lib my_lib2
```

Now we can have `my_lib2` depend on `my_lib` by adding the following to the `my_lib2` crate's `Cargo.toml` file:

```toml
[dependencies]
my_lib = { path = "../my_lib" }
```

