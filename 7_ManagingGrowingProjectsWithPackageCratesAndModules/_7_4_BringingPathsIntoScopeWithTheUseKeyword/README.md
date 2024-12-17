Bringing Paths into Scope with the use Keyword
使用 use 关键字将路径纳入范围
Having to write out the paths to call functions can feel inconvenient and repetitive. In Listing 7-7, whether we chose the absolute or relative path to the add_to_waitlist function, every time we wanted to call add_to_waitlist we had to specify front_of_house and hosting too. Fortunately, there’s a way to simplify this process: we can create a shortcut to a path with the use keyword once, and then use the shorter name everywhere else in the scope.
必须写出调用函数的路径可能会感觉不方便且重复。在清单 7-7 中，无论我们选择 add_to_waitlist 函数的绝对路径还是相对路径，每次我们想要调用 add_to_waitlist 时，我们都必须指定 front_of_house 和 hosting 也是如此。幸运的是，有一种方法可以简化这个过程：我们可以使用 use 关键字创建一次路径的快捷方式，然后在范围内的其他地方使用较短的名称。

In Listing 7-11, we bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant function so we only have to specify hosting::add_to_waitlist to call the add_to_waitlist function in eat_at_restaurant.
在清单 7-11 中，我们将 crate::front_of_house::hosting 模块带入 eat_at_restaurant 函数的作用域中，因此我们只需指定 hosting::add_to_waitlist 即可调用 add_to_waitlist 中的 /b3> 函数。

Filename: src/lib.rs 文件名：src/lib.rs

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
Listing 7-11: Bringing a module into scope with use
示例 7-11：使用 use 将模块引入作用域

Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.
在作用域中添加 use 和路径类似于在文件系统中创建符号链接。通过在 crate 根中添加 use crate::front_of_house::hosting ， hosting 现在是该范围内的有效名称，就像 hosting 模块已在 crate 根中定义一样。像任何其他路径一样，通过 use 引入范围的路径也会检查隐私。

Note that use only creates the shortcut for the particular scope in which the use occurs. Listing 7-12 moves the eat_at_restaurant function into a new child module named customer, which is then a different scope than the use statement, so the function body won’t compile:
请注意， use 仅为发生 use 的特定范围创建快捷方式。清单 7-12 将 eat_at_restaurant 函数移动到名为 customer 的新子模块中，该子模块的作用域与 use 语句不同，因此函数体获胜不编译：

Filename: src/lib.rs 文件名：src/lib.rs

This code does not compile!
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
Listing 7-12: A use statement only applies in the scope it’s in
示例 7-12： use 语句仅适用于它所在的范围

The compiler error shows that the shortcut no longer applies within the customer module:
编译器错误表明该快捷方式不再适用于 customer 模块：

$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
10 +     use crate::hosting;
   |

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted
Notice there’s also a warning that the use is no longer used in its scope! To fix this problem, move the use within the customer module too, or reference the shortcut in the parent module with super::hosting within the child customer module.
请注意，还有一条警告，表明 use 不再在其范围内使用！要解决此问题，请将 use 也移动到 customer 模块中，或者在子 customer

Creating Idiomatic use Paths
创建惯用的 use 路径
In Listing 7-11, you might have wondered why we specified use crate::front_of_house::hosting and then called hosting::add_to_waitlist in eat_at_restaurant rather than specifying the use path all the way out to the add_to_waitlist function to achieve the same result, as in Listing 7-13.
在清单 7-11 中，您可能想知道为什么我们指定 use crate::front_of_house::hosting ，然后在 eat_at_restaurant 中调用 hosting::add_to_waitlist ，而不是指定 use 路径一直到 add_to_waitlist 函数以获得相同的结果，如清单 7-13 所示。

Filename: src/lib.rs 文件名：src/lib.rs

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
Listing 7-13: Bringing the add_to_waitlist function into scope with use, which is unidiomatic
示例 7-13：使用 use 将 add_to_waitlist 函数引入作用域，这是不惯用的

Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is the idiomatic way to bring a function into scope with use. Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. The code in Listing 7-13 is unclear as to where add_to_waitlist is defined.
尽管清单 7-11 和 7-13 都完成了相同的任务，但清单 7-11 是将函数引入 use 作用域的惯用方法。使用 use 将函数的父模块纳入作用域意味着我们在调用函数时必须指定父模块。在调用函数时指定父模块可以清楚地表明该函数不是本地定义的，同时仍然最大限度地减少完整路径的重复。清单 7-13 中的代码不清楚 add_to_waitlist 的定义位置。

On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate.
另一方面，当使用 use 引入结构、枚举和其他项目时，指定完整路径是惯用的。清单 7-14 显示了将标准库的 HashMap 结构引入二进制 crate 范围的惯用方法。

Filename: src/main.rs 文件名：src/main.rs

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
Listing 7-14: Bringing HashMap into scope in an idiomatic way
示例 7-14：以惯用的方式将 HashMap 引入作用域

There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.
这个习惯用法背后没​​有什么强有力的理由：这只是已经出现的约定，人们已经习惯了以这种方式阅读和编写 Rust 代码。

The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that. Listing 7-15 shows how to bring two Result types into scope that have the same name but different parent modules and how to refer to them.
这种习惯用法的例外是，如果我们使用 use 语句将两个同名的项带入作用域，因为 Rust 不允许这样做。清单 7-15 显示了如何将两个具有相同名称但不同父模块的 Result 类型引入作用域，以及如何引用它们。

Filename: src/lib.rs 文件名：src/lib.rs

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
Listing 7-15: Bringing two types with the same name into the same scope requires using their parent modules.
示例 7-15：将两个同名类型放入同一作用域需要使用它们的父模块。

As you can see, using the parent modules distinguishes the two Result types. If instead we specified use std::fmt::Result and use std::io::Result, we’d have two Result types in the same scope and Rust wouldn’t know which one we meant when we used Result.
如您所见，使用父模块区分了两种 Result 类型。如果我们指定 use std::fmt::Result 和 use std::io::Result ，我们就会在同一范围内有两个 Result 类型，并且 Rust 不会知道我们使用时指的是哪一个。 Result 。

Providing New Names with the as Keyword
使用 as 关键字提供新名称
There’s another solution to the problem of bringing two types of the same name into the same scope with use: after the path, we can specify as and a new local name, or alias, for the type. Listing 7-16 shows another way to write the code in Listing 7-15 by renaming one of the two Result types using as.
对于使用 use 将两种同名类型引入同一作用域的问题，还有另一种解决方案：在路径之后，我们可以指定 as 和一个新的本地名称或别名，对于类型。清单 7-16 显示了编写清单 7-15 中的代码的另一种方法，即使用 as 重命名两个 Result 类型之一。

Filename: src/lib.rs 文件名：src/lib.rs

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
Listing 7-16: Renaming a type when it’s brought into scope with the as keyword
示例 7-16：当使用 as 关键字将类型引入作用域时重命名该类型

In the second use statement, we chose the new name IoResult for the std::io::Result type, which won’t conflict with the Result from std::fmt that we’ve also brought into scope. Listing 7-15 and Listing 7-16 are considered idiomatic, so the choice is up to you!
在第二个 use 语句中，我们为 std::io::Result 类型选择了新名称 IoResult ，这不会与 Result 冲突 std::fmt 我们也将其纳入范围。清单 7-15 和清单 7-16 被认为是惯用的，因此选择取决于您！

Re-exporting Names with pub use
使用 pub use 重新导出名称
When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
当我们使用 use 关键字将名称引入作用域时，新作用域中可用的名称是私有的。为了使调用我们代码的代码能够引用该名称，就好像它已在该代码的范围内定义一样，我们可以组合 pub 和 use 。这种技术称为重新导出，因为我们将某个项目纳入范围，同时也使该项目可供其他人纳入其范围。

Listing 7-17 shows the code in Listing 7-11 with use in the root module changed to pub use.
清单 7-17 显示了清单 7-11 中的代码，其中根模块中的 use 更改为 pub use 。

Filename: src/lib.rs 文件名：src/lib.rs

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
Listing 7-17: Making a name available for any code to use from a new scope with pub use
示例 7-17：使用 pub use 为新作用域中的任何代码提供可用的名称

Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist(). Now that this pub use has re-exported the hosting module from the root module, external code can now use the path restaurant::hosting::add_to_waitlist() instead.
在此更改之前，外部代码必须使用路径 restaurant::front_of_house::hosting::add_to_waitlist() 调用 add_to_waitlist 函数。现在这个 pub use 已经从根模块重新导出了 hosting 模块，外部代码现在可以使用路径 restaurant::hosting::add_to_waitlist() 来代替。

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With pub use, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library. We’ll look at another example of pub use and how it affects your crate’s documentation in the “Exporting a Convenient Public API with pub use” section of Chapter 14.
当代码的内部结构与调用代码的程序员对域的看法不同时，重新导出非常有用。例如，在这个餐厅比喻中，经营餐厅的人会想到“前台”和“后台”。但光顾餐厅的顾客可能不会从这些方面考虑餐厅的各个部分。使用 pub use ，我们可以使用一种结构编写代码，但公开一种不同的结构。这样做可以使我们的库为使用该库的程序员和调用该库的程序员提供良好的组织。我们将在第 14 章的“使用 pub use 导出便捷的公共 API”部分中查看 pub use 的另一个示例以及它如何影响您的 crate 文档。

Using External Packages 使用外部包
In Chapter 2, we programmed a guessing game project that used an external package called rand to get random numbers. To use rand in our project, we added this line to Cargo.toml:
在第 2 章中，我们编写了一个猜谜游戏项目，该项目使用名为 rand 的外部包来获取随机数。为了在我们的项目中使用 rand ，我们将此行添加到 Cargo.toml 中：

Filename: Cargo.toml 文件名：Cargo.toml

rand = "0.8.5"
Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.
在 Cargo.toml 中添加 rand 作为依赖项，告诉 Cargo 从 crates.io 下载 rand 包和任何依赖项，并使 rand 可用于我们的项目。

Then, to bring rand definitions into the scope of our package, we added a use line starting with the name of the crate, rand, and listed the items we wanted to bring into scope. Recall that in the “Generating a Random Number” section in Chapter 2, we brought the Rng trait into scope and called the rand::thread_rng function:
然后，为了将 rand 定义引入到我们的包的范围内，我们添加了一个 use 行，以箱的名称 rand 开头，并列出了项目我们想将其纳入范围。回想一下，在第 2 章的“生成随机数”部分中，我们将 Rng 特征引入作用域并调用 rand::thread_rng 函数：

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
Members of the Rust community have made many packages available at crates.io, and pulling any of them into your package involves these same steps: listing them in your package’s Cargo.toml file and using use to bring items from their crates into scope.
Rust 社区的成员在 crates.io 上提供了许多包，将它们中的任何一个拉入您的包中都涉及以下相同的步骤：将它们列在包的 Cargo.toml 文件中，并使用 use 从他们的板条箱进入范围。

Note that the standard std library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std. But we do need to refer to it with use to bring items from there into our package’s scope. For example, with HashMap we would use this line:
请注意，标准 std 库也是我们包外部的一个包。因为标准库是随 Rust 语言一起提供的，所以我们不需要更改 Cargo.toml 来包含 std 。但我们确实需要用 use 引用它，以将项目从那里带入我们的包的范围内。例如，对于 HashMap 我们将使用这一行：

use std::collections::HashMap;
This is an absolute path starting with std, the name of the standard library crate.
这是以 std （标准库包的名称）开头的绝对路径。

Using Nested Paths to Clean Up Large use Lists
使用嵌套路径清理大型 use 列表
If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files. For example, these two use statements we had in the Guessing Game in Listing 2-4 bring items from std into scope:
如果我们使用在同一板条箱或同一模块中定义的多个项目，则将每个项目单独列出可能会占用文件中的大量垂直空间。例如，清单 2-4 中的猜谜游戏中的这两个 use 语句将 std 中的项目引入作用域：

Filename: src/main.rs 文件名：src/main.rs

// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
Instead, we can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ, as shown in Listing 7-18.
相反，我们可以使用嵌套路径将相同的项目放入一行中的范围内。为此，我们指定路径的公共部分，后跟两个冒号，然后用大括号括住路径不同部分的列表，如清单 7-18 所示。

Filename: src/main.rs 文件名：src/main.rs

// --snip--
use std::{cmp::Ordering, io};
// --snip--
Listing 7-18: Specifying a nested path to bring multiple items with the same prefix into scope
示例 7-18：指定嵌套路径以将具有相同前缀的多个项目引入作用域

In bigger programs, bringing many items into scope from the same crate or module using nested paths can reduce the number of separate use statements needed by a lot!
在较大的程序中，使用嵌套路径将同一包或模块中的许多项纳入范围可以大大减少所需的单独 use 语句的数量！

We can use a nested path at any level in a path, which is useful when combining two use statements that share a subpath. For example, Listing 7-19 shows two use statements: one that brings std::io into scope and one that brings std::io::Write into scope.
我们可以在路径中的任何级别使用嵌套路径，这在组合共享子路径的两个 use 语句时非常有用。例如，清单 7-19 显示了两个 use 语句：一个将 std::io 引入作用域，另一个将 std::io::Write 引入作用域。

Filename: src/lib.rs 文件名：src/lib.rs

use std::io;
use std::io::Write;
Listing 7-19: Two use statements where one is a subpath of the other
示例 7-19：两个 use 语句，其中一个是另一个的子路径

The common part of these two paths is std::io, and that’s the complete first path. To merge these two paths into one use statement, we can use self in the nested path, as shown in Listing 7-20.
这两条路径的共同部分是 std::io ，这是完整的第一条路径。要将这两个路径合并到一个 use 语句中，我们可以在嵌套路径中使用 self ，如清单 7-20 所示。

Filename: src/lib.rs 文件名：src/lib.rs

use std::io::{self, Write};
Listing 7-20: Combining the paths in Listing 7-19 into one use statement
示例 7-20：将示例 7-19 中的路径合并到一个 use 语句中

This line brings std::io and std::io::Write into scope.
此行将 std::io 和 std::io::Write 纳入范围。

The Glob Operator  全局运算符
If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
如果我们要将路径中定义的所有公共项纳入范围，我们可以指定该路径，后跟 * glob 运算符：

use std::collections::*;
This use statement brings all public items defined in std::collections into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.
此 use 语句将 std::collections 中定义的所有公共项带入当前范围。使用 glob 运算符时要小心！ Glob 会使判断哪些名称在范围内以及程序中使用的名称的定义位置变得更加困难。

The glob operator is often used when testing to bring everything under test into the tests module; we’ll talk about that in the “How to Write Tests” section in Chapter 11. The glob operator is also sometimes used as part of the prelude pattern: see the standard library documentation for more information on that pattern.
测试时经常使用 glob 运算符，将所有被测试的内容放入 tests 模块中；我们将在第 11 章的“如何编写测试”部分中讨论这一点。 glob 运算符有时也用作前奏模式的一部分：有关该模式的更多信息，请参阅标准库文档。

