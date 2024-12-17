Separating Modules into Different Files
将模块分成不同的文件
So far, all the examples in this chapter defined multiple modules in one file. When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.
到目前为止，本章中的所有示例都在一个文件中定义了多个模块。当模块变大时，您可能希望将其定义移动到单独的文件中，以使代码更易于导航。

For example, let’s start from the code in Listing 7-17 that had multiple restaurant modules. We’ll extract modules into files instead of having all the modules defined in the crate root file. In this case, the crate root file is src/lib.rs, but this procedure also works with binary crates whose crate root file is src/main.rs.
例如，让我们从清单 7-17 中具有多个餐厅模块的代码开始。我们将把模块提取到文件中，而不是在板条箱根文件中定义所有模块。在本例中，crate 根文件为 src/lib.rs，但此过程也适用于 crate 根文件为 src/main.rs 的二进制 crate。

First, we’ll extract the front_of_house module to its own file. Remove the code inside the curly brackets for the front_of_house module, leaving only the mod front_of_house; declaration, so that src/lib.rs contains the code shown in Listing 7-21. Note that this won’t compile until we create the src/front_of_house.rs file in Listing 7-22.
首先，我们将 front_of_house 模块提取到它自己的文件中。删除 front_of_house 模块大括号内的代码，仅保留 mod front_of_house; 声明，以便 src/lib.rs 包含清单 7-21 所示的代码。请注意，直到我们创建清单 7-22 中的 src/front_of_house.rs 文件后，它才会编译。

Filename: src/lib.rs 文件名：src/lib.rs

This code does not compile!
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
Listing 7-21: Declaring the front_of_house module whose body will be in src/front_of_house.rs
示例 7-21：声明 front_of_house 模块，其主体位于 src/front_of_house.rs 中

Next, place the code that was in the curly brackets into a new file named src/front_of_house.rs, as shown in Listing 7-22. The compiler knows to look in this file because it came across the module declaration in the crate root with the name front_of_house.
接下来，将大括号中的代码放入名为 src/front_of_house.rs 的新文件中，如清单 7-22 所示。编译器知道要查找此文件，因为它在包根中遇到了名为 front_of_house 的模块声明。

Filename: src/front_of_house.rs
文件名：src/front_of_house.rs

pub mod hosting {
    pub fn add_to_waitlist() {}
}
Listing 7-22: Definitions inside the front_of_house module in src/front_of_house.rs
示例 7-22：src/front_of_house.rs 中 front_of_house 模块内的定义

Note that you only need to load a file using a mod declaration once in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared, as covered in the “Paths for Referring to an Item in the Module Tree” section. In other words, mod is not an “include” operation that you may have seen in other programming languages.
请注意，您只需在模块树中使用 mod 声明加载一次文件。一旦编译器知道该文件是项目的一部分（并且知道代码位于模块树中的位置，因为您放置了 mod 语句），项目中的其他文件应该引用加载的文件文件的代码使用其声明位置的路径，如“引用模块树中项目的路径”部分所述。换句话说， mod 并不是您在其他编程语言中见过的“包含”操作。

Next, we’ll extract the hosting module to its own file. The process is a bit different because hosting is a child module of front_of_house, not of the root module. We’ll place the file for hosting in a new directory that will be named for its ancestors in the module tree, in this case src/front_of_house/.
接下来，我们将 hosting 模块提取到它自己的文件中。该过程有点不同，因为 hosting 是 front_of_house 的子模块，而不是根模块。我们将把 hosting 的文件放在一个新目录中，该目录将以其在模块树中的祖先命名，在本例中为 src/front_of_house/。

To start moving hosting, we change src/front_of_house.rs to contain only the declaration of the hosting module:
要开始移动 hosting ，我们更改 src/front_of_house.rs 以仅包含 hosting 模块的声明：

Filename: src/front_of_house.rs
文件名：src/front_of_house.rs

pub mod hosting;
Then we create a src/front_of_house directory and a file hosting.rs to contain the definitions made in the hosting module:
然后我们创建一个 src/front_of_house 目录和一个 Hosting.rs 文件来包含 hosting 模块中所做的定义：

Filename: src/front_of_house/hosting.rs
文件名：src/front_of_house/hosting.rs

pub fn add_to_waitlist() {}
If we instead put hosting.rs in the src directory, the compiler would expect the hosting.rs code to be in a hosting module declared in the crate root, and not declared as a child of the front_of_house module. The compiler’s rules for which files to check for which modules’ code means the directories and files more closely match the module tree.
如果我们将hosting.rs放在src目录中，编译器会期望hosting.rs代码位于crate根中声明的 hosting 模块中，而不是声明为 front_of_house 模块。编译器关于哪些文件检查哪些模块代码的规则意味着目录和文件与模块树更加匹配。

Alternate File Paths  备用文件路径
So far we’ve covered the most idiomatic file paths the Rust compiler uses, but Rust also supports an older style of file path. For a module named front_of_house declared in the crate root, the compiler will look for the module’s code in:
到目前为止，我们已经介绍了 Rust 编译器使用的最惯用的文件路径，但 Rust 还支持旧样式的文件路径。对于在 crate 根中声明的名为 front_of_house 的模块，编译器将在以下位置查找该模块的代码：

src/front_of_house.rs (what we covered)
src/front_of_house.rs（我们涵盖的内容）
src/front_of_house/mod.rs (older style, still supported path)
src/front_of_house/mod.rs（较旧的样式，仍然受支持的路径）
For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:
对于名为 hosting 的模块，它是 front_of_house 的子模块，编译器将在以下位置查找该模块的代码：

src/front_of_house/hosting.rs (what we covered)
src/front_of_house/hosting.rs（我们介绍的内容）
src/front_of_house/hosting/mod.rs (older style, still supported path)
src/front_of_house/hosting/mod.rs（较旧的样式，仍然受支持的路径）
If you use both styles for the same module, you’ll get a compiler error. Using a mix of both styles for different modules in the same project is allowed, but might be confusing for people navigating your project.
如果您对同一模块使用两种样式，则会出现编译器错误。允许在同一项目中的不同模块中混合使用两种样式，但可能会让浏览项目的人感到困惑。

The main downside to the style that uses files named mod.rs is that your project can end up with many files named mod.rs, which can get confusing when you have them open in your editor at the same time.
使用名为 mod.rs 的文件的样式的主要缺点是，您的项目最终可能会包含许多名为 mod.rs 的文件，当您同时在编辑器中打开它们时，这可能会造成混乱。

We’ve moved each module’s code to a separate file, and the module tree remains the same. The function calls in eat_at_restaurant will work without any modification, even though the definitions live in different files. This technique lets you move modules to new files as they grow in size.
我们已将每个模块的代码移至单独的文件中，并且模块树保持不变。 eat_at_restaurant 中的函数调用无需任何修改即可工作，即使定义位于不同的文件中。此技术允许您在模块大小增长时将模块移动到新文件中。

Note that the pub use crate::front_of_house::hosting statement in src/lib.rs also hasn’t changed, nor does use have any impact on what files are compiled as part of the crate. The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.
请注意，src/lib.rs 中的 pub use crate::front_of_house::hosting 语句也没有更改， use 对哪些文件被编译为 crate 的一部分也没有任何影响。 mod 关键字声明模块，Rust 在与模块同名的文件中查找进入该模块的代码。

Summary 概括
Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the pub keyword.
Rust 允许您将一个包拆分为多个板条箱，并将一个板条箱拆分为模块，以便您可以从另一个模块引用一个模块中定义的项目。您可以通过指定绝对或相对路径来完成此操作。可以使用 use 语句将这些路径引入范围，以便您可以使用较短的路径来多次使用该范围内的项目。默认情况下，模块代码是私有的，但您可以通过添加 pub 关键字将定义公开。

In the next chapter, we’ll look at some collection data structures in the standard library that you can use in your neatly organized code.
在下一章中，我们将介绍标准库中的一些集合数据结构，您可以在组织整齐的代码中使用它们。

Separating Modules into Different Files
将模块分成不同的文件
So far, all the examples in this chapter defined multiple modules in one file. When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.
到目前为止，本章中的所有示例都在一个文件中定义了多个模块。当模块变大时，您可能希望将其定义移动到单独的文件中，以使代码更易于导航。

For example, let’s start from the code in Listing 7-17 that had multiple restaurant modules. We’ll extract modules into files instead of having all the modules defined in the crate root file. In this case, the crate root file is src/lib.rs, but this procedure also works with binary crates whose crate root file is src/main.rs.
例如，让我们从清单 7-17 中具有多个餐厅模块的代码开始。我们将把模块提取到文件中，而不是在板条箱根文件中定义所有模块。在本例中，crate 根文件为 src/lib.rs，但此过程也适用于 crate 根文件为 src/main.rs 的二进制 crate。

First, we’ll extract the front_of_house module to its own file. Remove the code inside the curly brackets for the front_of_house module, leaving only the mod front_of_house; declaration, so that src/lib.rs contains the code shown in Listing 7-21. Note that this won’t compile until we create the src/front_of_house.rs file in Listing 7-22.
首先，我们将 front_of_house 模块提取到它自己的文件中。删除 front_of_house 模块大括号内的代码，仅保留 mod front_of_house; 声明，以便 src/lib.rs 包含清单 7-21 所示的代码。请注意，直到我们创建清单 7-22 中的 src/front_of_house.rs 文件后，它才会编译。

Filename: src/lib.rs 文件名：src/lib.rs

This code does not compile!
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
Listing 7-21: Declaring the front_of_house module whose body will be in src/front_of_house.rs
示例 7-21：声明 front_of_house 模块，其主体位于 src/front_of_house.rs 中

Next, place the code that was in the curly brackets into a new file named src/front_of_house.rs, as shown in Listing 7-22. The compiler knows to look in this file because it came across the module declaration in the crate root with the name front_of_house.
接下来，将大括号中的代码放入名为 src/front_of_house.rs 的新文件中，如清单 7-22 所示。编译器知道要查找此文件，因为它在包根中遇到了名为 front_of_house 的模块声明。

Filename: src/front_of_house.rs
文件名：src/front_of_house.rs

pub mod hosting {
    pub fn add_to_waitlist() {}
}
Listing 7-22: Definitions inside the front_of_house module in src/front_of_house.rs
示例 7-22：src/front_of_house.rs 中 front_of_house 模块内的定义

Note that you only need to load a file using a mod declaration once in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared, as covered in the “Paths for Referring to an Item in the Module Tree” section. In other words, mod is not an “include” operation that you may have seen in other programming languages.
请注意，您只需在模块树中使用 mod 声明加载一次文件。一旦编译器知道该文件是项目的一部分（并且知道代码位于模块树中的位置，因为您放置了 mod 语句），项目中的其他文件应该引用加载的文件文件的代码使用其声明位置的路径，如“引用模块树中项目的路径”部分所述。换句话说， mod 并不是您在其他编程语言中见过的“包含”操作。

Next, we’ll extract the hosting module to its own file. The process is a bit different because hosting is a child module of front_of_house, not of the root module. We’ll place the file for hosting in a new directory that will be named for its ancestors in the module tree, in this case src/front_of_house/.
接下来，我们将 hosting 模块提取到它自己的文件中。该过程有点不同，因为 hosting 是 front_of_house 的子模块，而不是根模块。我们将把 hosting 的文件放在一个新目录中，该目录将以其在模块树中的祖先命名，在本例中为 src/front_of_house/。

To start moving hosting, we change src/front_of_house.rs to contain only the declaration of the hosting module:
要开始移动 hosting ，我们更改 src/front_of_house.rs 以仅包含 hosting 模块的声明：

Filename: src/front_of_house.rs
文件名：src/front_of_house.rs

pub mod hosting;
Then we create a src/front_of_house directory and a file hosting.rs to contain the definitions made in the hosting module:
然后我们创建一个 src/front_of_house 目录和一个 Hosting.rs 文件来包含 hosting 模块中所做的定义：

Filename: src/front_of_house/hosting.rs
文件名：src/front_of_house/hosting.rs

pub fn add_to_waitlist() {}
If we instead put hosting.rs in the src directory, the compiler would expect the hosting.rs code to be in a hosting module declared in the crate root, and not declared as a child of the front_of_house module. The compiler’s rules for which files to check for which modules’ code means the directories and files more closely match the module tree.
如果我们将hosting.rs放在src目录中，编译器会期望hosting.rs代码位于crate根中声明的 hosting 模块中，而不是声明为 front_of_house 模块。编译器关于哪些文件检查哪些模块代码的规则意味着目录和文件与模块树更加匹配。

Alternate File Paths  备用文件路径
So far we’ve covered the most idiomatic file paths the Rust compiler uses, but Rust also supports an older style of file path. For a module named front_of_house declared in the crate root, the compiler will look for the module’s code in:
到目前为止，我们已经介绍了 Rust 编译器使用的最惯用的文件路径，但 Rust 还支持旧样式的文件路径。对于在 crate 根中声明的名为 front_of_house 的模块，编译器将在以下位置查找该模块的代码：

src/front_of_house.rs (what we covered)
src/front_of_house.rs（我们涵盖的内容）
src/front_of_house/mod.rs (older style, still supported path)
src/front_of_house/mod.rs（较旧的样式，仍然受支持的路径）
For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:
对于名为 hosting 的模块，它是 front_of_house 的子模块，编译器将在以下位置查找该模块的代码：

src/front_of_house/hosting.rs (what we covered)
src/front_of_house/hosting.rs（我们介绍的内容）
src/front_of_house/hosting/mod.rs (older style, still supported path)
src/front_of_house/hosting/mod.rs（较旧的样式，仍然受支持的路径）
If you use both styles for the same module, you’ll get a compiler error. Using a mix of both styles for different modules in the same project is allowed, but might be confusing for people navigating your project.
如果您对同一模块使用两种样式，则会出现编译器错误。允许在同一项目中的不同模块中混合使用两种样式，但可能会让浏览项目的人感到困惑。

The main downside to the style that uses files named mod.rs is that your project can end up with many files named mod.rs, which can get confusing when you have them open in your editor at the same time.
使用名为 mod.rs 的文件的样式的主要缺点是，您的项目最终可能会包含许多名为 mod.rs 的文件，当您同时在编辑器中打开它们时，这可能会造成混乱。

We’ve moved each module’s code to a separate file, and the module tree remains the same. The function calls in eat_at_restaurant will work without any modification, even though the definitions live in different files. This technique lets you move modules to new files as they grow in size.
我们已将每个模块的代码移至单独的文件中，并且模块树保持不变。 eat_at_restaurant 中的函数调用无需任何修改即可工作，即使定义位于不同的文件中。此技术允许您在模块大小增长时将模块移动到新文件中。

Note that the pub use crate::front_of_house::hosting statement in src/lib.rs also hasn’t changed, nor does use have any impact on what files are compiled as part of the crate. The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.
请注意，src/lib.rs 中的 pub use crate::front_of_house::hosting 语句也没有更改， use 对哪些文件被编译为 crate 的一部分也没有任何影响。 mod 关键字声明模块，Rust 在与模块同名的文件中查找进入该模块的代码。

Summary 概括
Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the pub keyword.
Rust 允许您将一个包拆分为多个板条箱，并将一个板条箱拆分为模块，以便您可以从另一个模块引用一个模块中定义的项目。您可以通过指定绝对或相对路径来完成此操作。可以使用 use 语句将这些路径引入范围，以便您可以使用较短的路径来多次使用该范围内的项目。默认情况下，模块代码是私有的，但您可以通过添加 pub 关键字将定义公开。

In the next chapter, we’ll look at some collection data structures in the standard library that you can use in your neatly organized code.
在下一章中，我们将介绍标准库中的一些集合数据结构，您可以在组织整齐的代码中使用它们。

