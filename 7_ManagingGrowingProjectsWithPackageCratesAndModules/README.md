Rust 编程语言
 
Managing Growing Projects with Packages, Crates, and Modules
使用包、板条箱和模块管理不断增长的项目
As you write large programs, organizing your code will become increasingly important. By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.
当您编写大型程序时，组织代码将变得越来越重要。通过对相关功能进行分组并将代码与不同的功能分开，您将弄清楚在哪里可以找到实现特定功能的代码以及在哪里可以更改功能的工作方式。

The programs we’ve written so far have been in one module in one file. As a project grows, you should organize code by splitting it into multiple modules and then multiple files. A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. This chapter covers all these techniques. For very large projects comprising a set of interrelated packages that evolve together, Cargo provides workspaces, which we’ll cover in the “Cargo Workspaces” section in Chapter 14.
到目前为止，我们编写的程序都位于一个文件的一个模块中。随着项目的增长，您应该通过将代码拆分为多个模块，然后拆分为多个文件来组织代码。一个包可以包含多个二进制 crate，也可以包含一个库 crate。随着包的增长，您可以将各个部分提取到单独的包中，这些包将成为外部依赖项。本章涵盖了所有这些技术。对于包含一组共同发展的相互关联的包的大型项目，Cargo 提供了工作空间，我们将在第 14 章的“Cargo 工作空间”部分中介绍这些工作空间。

We’ll also discuss encapsulating implementation details, which lets you reuse code at a higher level: once you’ve implemented an operation, other code can call your code via its public interface without having to know how the implementation works. The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head.
我们还将讨论封装实现细节，这使您可以在更高级别重用代码：实现一个操作后，其他代码可以通过其公共接口调用您的代码，而无需了解实现的工作原理。您编写代码的方式定义了哪些部分是可供其他代码使用的公共部分，以及哪些部分是您保留更改权利的私有实现细节。这是限制您必须记住的细节数量的另一种方法。

A related concept is scope: the nested context in which code is written has a set of names that are defined as “in scope.” When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means. You can create scopes and change which names are in or out of scope. You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.
一个相关的概念是范围：编写代码的嵌套上下文有一组定义为“范围内”的名称。在读取、编写和编译代码时，程序员和编译器需要知道特定位置的特定名称是否引用变量、函数、结构、枚举、模块、常量或其他项目以及该项目的含义。您可以创建范围并更改范围内或范围外的名称。同一范围内不能有两个同名的项目；可以使用工具来解决名称冲突。

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
Rust 具有许多功能，可让您管理代码的组织，包括公开哪些详细信息、哪些详细信息是私有的，以及程序中每个范围内的名称。这些功能有时统称为模块系统，包括：

Packages: A Cargo feature that lets you build, test, and share crates
Packages：Cargo 功能，可让您构建、测试和共享 crate
Crates: A tree of modules that produces a library or executable
板条箱：生成库或可执行文件的模块树
Modules and use: Let you control the organization, scope, and privacy of paths
模块和使用：让您控制路径的组织、范围和隐私
Paths: A way of naming an item, such as a struct, function, or module
路径：命名项目（例如结构、函数或模块）的一种方式
In this chapter, we’ll cover all these features, discuss how they interact, and explain how to use them to manage scope. By the end, you should have a solid understanding of the module system and be able to work with scopes like a pro!
在本章中，我们将介绍所有这些功能，讨论它们如何交互，并解释如何使用它们来管理范围。最后，您应该对模块系统有深入的了解，并能够像专业人士一样使用示波器！

