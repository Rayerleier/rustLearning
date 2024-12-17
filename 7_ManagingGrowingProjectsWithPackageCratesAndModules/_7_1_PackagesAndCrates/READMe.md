Packages and Crates 包装和板条箱
The first parts of the module system we’ll cover are packages and crates.
我们将介绍的模块系统的第一部分是包和板条箱。

A crate is the smallest amount of code that the Rust compiler considers at a time. Even if you run rustc rather than cargo and pass a single source code file (as we did all the way back in the “Writing and Running a Rust Program” section of Chapter 1), the compiler considers that file to be a crate. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.
箱子是 Rust 编译器一次考虑的最小代码量。即使您运行 rustc 而不是 cargo 并传递单个源代码文件（就像我们在第 1 章的“编写和运行 Rust 程序”部分中所做的那样） ，编译器认为该文件是一个 crate。板条箱可以包含模块，并且模块可以在与板条箱一起编译的其他文件中定义，正如我们将在接下来的部分中看到的那样。

A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.
板条箱可以采用以下两种形式之一：二进制板条箱或库板条箱。二进制包是可以编译为可以运行的可执行文件的程序，例如命令行程序或服务器。每个都必须有一个名为 main 的函数，该函数定义可执行文件运行时会发生什么。到目前为止，我们创建的所有 crate 都是二进制 crate。

Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers. Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".
库包没有 main 函数，并且它们不会编译为可执行文件。相反，它们定义了旨在与多个项目共享的功能。例如，我们在第 2 章中使用的 rand 包提供了生成随机数的功能。大多数时候，Rustaceans 说“crate”时，他们指的是库 crate，并且他们将“crate”与“库”的一般编程概念互换使用。

The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate (we’ll explain modules in depth in the “Defining Modules to Control Scope and Privacy” section).
crate 根是 Rust 编译器启动的源文件，并构成 crate 的根模块（我们将在“定义控制范围和隐私的模块”部分中深入解释模块）。

A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code. The Cargo package also contains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.
包是一个或多个提供一组功能的包的捆绑。包中包含一个 Cargo.toml 文件，该文件描述了如何构建这些 crate。 Cargo 实际上是一个包，其中包含您用来构建代码的命令行工具的二进制包。 Cargo 包还包含二进制 crate 所依赖的库 crate。其他项目可以依赖 Cargo 库 crate 来使用 Cargo 命令行工具使用的相同逻辑。

A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.
一个包可以包含任意多个二进制 crate，但最多只能包含一个库 crate。一个包必须至少包含一个 crate，无论是库还是二进制 crate。

Let’s walk through what happens when we create a package. First, we enter the command cargo new:
让我们来看看创建包时会发生什么。首先，我们输入命令 cargo new ：

$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
After we run cargo new, we use ls to see what Cargo creates. In the project directory, there’s a Cargo.toml file, giving us a package. There’s also a src directory that contains main.rs. Open Cargo.toml in your text editor, and note there’s no mention of src/main.rs. Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.
运行 cargo new 后，我们使用 ls 来查看 Cargo 创建的内容。在项目目录中，有一个 Cargo.toml 文件，为我们提供了一个包。还有一个包含 main.rs 的 src 目录。在文本编辑器中打开 Cargo.toml，请注意没有提及 src/main.rs。 Cargo 遵循一个约定，即 src/main.rs 是与包同名的二进制 crate 的 crate 根。同样，Cargo 知道如果包目录包含 src/lib.rs，则该包包含一个与该包同名的库 crate，并且 src/lib.rs 是其 crate 根。 Cargo 将 crate 根文件传递给 rustc 以构建库或二进制文件。

Here, we have a package that only contains src/main.rs, meaning it only contains a binary crate named my-project. If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
在这里，我们有一个仅包含 src/main.rs 的包，这意味着它仅包含一个名为 my-project 的二进制包。如果一个包包含 src/main.rs 和 src/lib.rs，那么它有两个 crate：一个二进制文件和一个库，两者都与包同名。通过将文件放置在 src/bin 目录中，一个包可以拥有多个二进制 crate：每个文件将是一个单独的二进制 crate。

