Defining Modules to Control Scope and Privacy
定义模块来控制范围和隐私
In this section, we’ll talk about modules and other parts of the module system, namely paths that allow you to name items; the use keyword that brings a path into scope; and the pub keyword to make items public. We’ll also discuss the as keyword, external packages, and the glob operator.
在本节中，我们将讨论模块和模块系统的其他部分，即允许您命名项目的路径；将路径引入范围的 use 关键字；以及用于公开项目的 pub 关键字。我们还将讨论 as 关键字、外部包和 glob 运算符。

First, we’re going to start with a list of rules for easy reference when you’re organizing your code in the future. Then we’ll explain each of the rules in detail.
首先，我们将从一系列规则开始，以便您将来组织代码时轻松参考。然后我们将详细解释每条规则。

Modules Cheat Sheet  模块备忘单
Here we provide a quick reference on how modules, paths, the use keyword, and the pub keyword work in the compiler, and how most developers organize their code. We’ll be going through examples of each of these rules throughout this chapter, but this is a great place to refer to as a reminder of how modules work.
在这里，我们提供了有关模块、路径、 use 关键字和 pub 关键字在编译器中如何工作以及大多数开发人员如何组织代码的快速参考。我们将在本章中介绍每条规则的示例，但这是一个很好的地方，可以用来提醒模块如何工作。

Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
从 crate 根开始：编译 crate 时，编译器首先在 crate 根文件（对于库 crate 通常为 src/lib.rs，对于二进制 crate 则为 src/main.rs）查找要编译的代码。
Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
声明模块：在 crate 根文件中，您可以声明新模块；比如说，您使用 mod garden; 声明一个“garden”模块。编译器将在这些位置查找模块的代码：
Inline, within curly brackets that replace the semicolon following mod garden
内联，在大括号内，替换 mod garden 后面的分号
In the file src/garden.rs
在文件 src/garden.rs 中
In the file src/garden/mod.rs
在文件 src/garden/mod.rs 中
Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
声明子模块：在除 crate 根之外的任何文件中，您都可以声明子模块。例如，您可以在 src/garden.rs 中声明 mod vegetables; 。编译器将在以下位置的父模块命名的目录中查找子模块的代码：
Inline, directly following mod vegetables, within curly brackets instead of the semicolon
内联，直接在 mod vegetables 之后，在大括号内而不是分号内
In the file src/garden/vegetables.rs
在文件 src/garden/vegetables.rs 中
In the file src/garden/vegetables/mod.rs
在文件 src/garden/vegetables/mod.rs 中
Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.
模块中代码的路径：一旦模块成为您的 crate 的一部分，只要隐私规则允许，您就可以使用代码的路径从同一 crate 中的其他任何位置引用该模块中的代码。例如，花园蔬菜模块中的 Asparagus 类型可以在 crate::garden::vegetables::Asparagus 处找到。
Private vs public: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.
私有与公共：默认情况下，模块内的代码对于其父模块是私有的。要使模块公开，请使用 pub mod 而不是 mod 声明它。要使公共模块中的项目也成为公共的，请在其声明之前使用 pub 。
The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus to make use of that type in the scope.
use 关键字：在某个范围内， use 关键字创建项目的快捷方式以减少长路径的重复。在任何可以引用 crate::garden::vegetables::Asparagus 的作用域中，您可以使用 use crate::garden::vegetables::Asparagus; 创建快捷方式，从那时起，您只需编写 Asparagus 即可使用该类型范围内。
Here we create a binary crate named backyard that illustrates these rules. The crate’s directory, also named backyard, contains these files and directories:
在这里，我们创建一个名为 backyard 的二进制包来说明这些规则。板条箱的目录也称为 backyard ，包含以下文件和目录：

backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
The crate root file in this case is src/main.rs, and it contains:
本例中的板条箱根文件是 src/main.rs，它包含：

Filename: src/main.rs 文件名：src/main.rs

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
The pub mod garden; line tells the compiler to include the code it finds in src/garden.rs, which is:
pub mod garden; 行告诉编译器包含在 src/garden.rs 中找到的代码，即：

Filename: src/garden.rs 文件名：src/garden.rs

pub mod vegetables;
Here, pub mod vegetables; means the code in src/garden/vegetables.rs is included too. That code is:
这里， pub mod vegetables; 表示 src/garden/vegetables.rs 中的代码也包含在内。该代码是：

#[derive(Debug)]
pub struct Asparagus {}
Now let’s get into the details of these rules and demonstrate them in action!
现在让我们详细了解这些规则并在实践中演示它们！

Grouping Related Code in Modules
将相关代码分组到模块中
Modules let us organize code within a crate for readability and easy reuse. Modules also allow us to control the privacy of items, because code within a module is private by default. Private items are internal implementation details not available for outside use. We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.
模块让我们可以在包内组织代码，以提高可读性和易于重用。模块还允许我们控制项目的隐私，因为模块内的代码默认是私有的。私有项目是内部实现细节，不可供外部使用。我们可以选择将模块及其中的项目公开，从而公开它们以允许外部代码使用和依赖它们。

As an example, let’s write a library crate that provides the functionality of a restaurant. We’ll define the signatures of functions but leave their bodies empty to concentrate on the organization of the code, rather than the implementation of a restaurant.
作为示例，让我们编写一个提供餐厅功能的库箱。我们将定义函数的签名，但将它们的主体留空，以专注于代码的组织，而不是餐厅的实现。

In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house. Front of house is where customers are; this encompasses where the hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.
在餐饮业中，餐厅的某些部分称为前台，其他部分称为后台。前台是顾客所在的地方；这包括主人让顾客就座、服务员接受订单和付款以及调酒师调制饮料的地方。后厨是厨师和厨师在厨房工作、洗碗机清洁以及经理进行行政工作的地方。

To structure our crate in this way, we can organize its functions into nested modules. Create a new library named restaurant by running cargo new restaurant --lib; then enter the code in Listing 7-1 into src/lib.rs to define some modules and function signatures. Here’s the front of house section:
为了以这种方式构建我们的箱子，我们可以将其功能组织到嵌套模块中。通过运行 cargo new restaurant --lib 创建一个名为 restaurant 的新库；然后将清单7-1中的代码输入到src/lib.rs中以定义一些模块和函数签名。这是房子的前面部分：

Filename: src/lib.rs 文件名：src/lib.rs

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
Listing 7-1: A front_of_house module containing other modules that then contain functions
清单 7-1：一个 front_of_house 模块，其中包含其他包含函数的模块

We define a module with the mod keyword followed by the name of the module (in this case, front_of_house). The body of the module then goes inside curly brackets. Inside modules, we can place other modules, as in this case with the modules hosting and serving. Modules can also hold definitions for other items, such as structs, enums, constants, traits, and—as in Listing 7-1—functions.
我们使用 mod 关键字定义一个模块，后跟模块名称（在本例中为 front_of_house ）。然后模块的主体进入大括号内。在模块内部，我们可以放置其他模块，例如本例中的模块 hosting 和 serving 。模块还可以保存其他项目的定义，例如结构、枚举、常量、特征以及（如清单 7-1 所示）函数。

By using modules, we can group related definitions together and name why they’re related. Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them. Programmers adding new functionality to this code would know where to place the code to keep the program organized.
通过使用模块，我们可以将相关的定义分组在一起并说出它们相关的原因。使用此代码的程序员可以根据组导航代码，而不必阅读所有定义，从而更容易找到与其相关的定义。向此代码添加新功能的程序员将知道将代码放置在哪里以保持程序的组织性。

Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.
之前，我们提到 src/main.rs 和 src/lib.rs 称为 crate 根。它们的名称的原因是这两个文件中的任何一个的内容在 crate 模块结构的根部形成一个名为 crate 的模块，称为模块树。

Listing 7-2 shows the module tree for the structure in Listing 7-1.
清单 7-2 显示了清单 7-1 中结构的模块树。

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
Listing 7-2: The module tree for the code in Listing 7-1
清单 7-2：清单 7-1 中代码的模块树

This tree shows how some of the modules nest inside one another; for example, hosting nests inside front_of_house. The tree also shows that some modules are siblings to each other, meaning they’re defined in the same module; hosting and serving are siblings defined within front_of_house. If module A is contained inside module B, we say that module A is the child of module B and that module B is the parent of module A. Notice that the entire module tree is rooted under the implicit module named crate.
该树显示了一些模块如何相互嵌套；例如， hosting 嵌套在 front_of_house 内。该树还显示一些模块彼此是兄弟模块，这意味着它们是在同一模块中定义的； hosting 和 serving 是 front_of_house 中定义的同级。如果模块 A 包含在模块 B 中，我们说模块 A 是模块 B 的子模块，而模块 B 是模块 A 的父模块。请注意，整个模块树的根位于名为 crate .

The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.
模块树可能会让您想起计算机上文件系统的目录树；这是一个非常恰当的比较！就像文件系统中的目录一样，您可以使用模块来组织代码。就像目录中的文件一样，我们需要一种方法来查找模块。

