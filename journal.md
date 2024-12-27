# SysY Compiler 完成记录

Author: snow276

本文档在完成 sysy-compiler 的过程中实时更新，记录了作者在完成本 lab 的各个 level 时 1）梳理的思路 2）实现的方法 3）遇到的问题 4）生发的感想。

作者使用的编程语言为 Rust 。

作者在完成本 lab 时在一些地方借鉴了课程的实例编译器 [kira-rs](https://github.com/pku-minic/kira-rs.git) 。

## Lv1. `main` 函数 & Lv2. 初试目标代码生成

写作本文档时，距离作者完成 Lv1 和 Lv2 都已经过了一个多月，距离完成 Lv3 也已经过了好几天。所以 Lv1 和 Lv2 都会写得比较简略（事实上，这两个 level 本来也比较简单，所以本来要梳理的就不多），Lv3 也会写得比较简单。

Lv1 和 Lv2 主要就是帮助大家入门语法/词法分析器怎么用、AST 怎么定义、IR 怎么生成、目标代码怎么生成。

作者使用 `lalrpop` 作为语法/词法分析器，并且在生成 IR 时是直接生成的内存模式。

作者的时间投入分布大致如下：

1. 花了一些精力大致浏览了 `lalrpop` 库的文档，学习了如何使用 `lalrpop` 语法/词法分析器。
2. 很快写掉了 AST 的定义。
3. 花了很多精力看了 Koopa IR 的文档（然后发现其文档十分不完备），在破防边缘时在文档的样例代码里找到了如何添加 Koopa IR 值的大概样例，成功把 Koopa IR 的 `ret` 语句生成了出来。
4. 花了很多精力接着研究 Koopa IR 的文档找例子，并且成功把 `ret` 语句的汇编生成了出来。
5. 花了很多精力研究 `main` 函数怎么写（比如用什么接口根据内存形式的 Koopa IR 生成文本形式的、比如错误处理的框架）。
6. 最初实现的版本是不会在前/后端的生成函数中涵盖任何关于全局环境的参数的。但是这样之所以对于 Lv1 和 Lv2 能 work 纯粹是因为只有一个 `return` 语句的主函数实在是结构太简单了。后来意识到，后面不可避免地会需要在逐层调用生成函数时读取一些全局的信息，所以在前/后端的实现中新增了 `IrgenEnv` 和 `CodegenEnv` 两个结构，在里面存储一些前/后端生成时会用的全局信息。

## Lv3. 表达式 Frontend

本章中，需要实现一个能够处理表达式 (一元/二元) 的编译器。

说实话，回头来看觉得本章挺简单的。作为使用 Rust 的用户，本章最大的难度在于需要理解 Koopa IR 的代码结构，理解之后就都很顺利了。

在此就简单提一些对 Koopa IR 的认识：

1. Koopa IR 中，控制流和数据流是分开的。`Function` 、 `Value` 等数据结构其实都是指针，通过这些指针可以获得其指向的实际数据 `FunctionData` 、 `ValueData` 等。这是出于 Rust 的所有权机制考虑的。这些表示指针的数据类型都很轻量级，并且也都是有 `Copy` Trait 的，所以在选择存储什么全局信息时，作者认为的原则是存指针而不存数据，需要数据时再根据指针去找。
2. Koopa IR 中，把数值和指令进行了统一抽象，都是 Koopa IR 中的“值”。一个指令（比如 `add` 、`ret`）的操作数可以是数值，也可以是其他指令。这使得在生成内存形式的 IR 时代码可以写得很简洁。

在有了以上的认识之后，本 level 的前端部分具体的实现方案就比较简单，这里就不再仔细介绍了。

Lv3 的后端暂时先跳过了，根据树洞给出的建议，Lv3 的后端和 Lv4 一起写比较好。

## Lv4. 常量和变量 Frontend

本章中，需要实现一个能够处理常量/变量定义和赋值语句的编译器。

作者在完成本 level 时直接把 Lv4.1 和 Lv4.2 放在一起梳理和实现了。

### 功能梳理和实现步骤

#### 1. 修改词法/语法分析和 AST

* [X] 按照要求修改 `sysy.lalrpop` 文件和 `ast.rs` 文件

在处理本节新增的 EBNF `{...}` 时，参考了 kira-rs 中的方法，使用 `<mut const_def_list: ("," <ConstDef>``)*>` 这种方式可以接受任意多个项并且把结果存为一个 `Vec` 。

#### 2. 符号表数据结构

本节最主要的更改就是需要维护符号表。符号表维护了从符号的名字到符号的值的对应。

符号的名字用某种字符串表示。这里作者选择了用 `&str` 。（ `&str` 和 `String` 都实现了 `Hash` 和 `Eq` Trait，所以这里用 `String` 应该也是可以的。）

> 对 Rust 的一个体悟：写 Rust ，不论用引用还是不用引用，都可以写得是否丑陋。对于设计符号表的这个例子，如果用 `String` ，那有可能会在设计表操作时牵扯到一些我意料之外的所有权转移问题，想要规避这些所有权转移问题的话，滥用  `.clone()` 就好了，但这可以说是相当丑陋；如果用 `&str` ，那就牵涉到怎么设计生命周期的问题，而最愚蠢却又能过编译的方式就是给它一个很长的（比逻辑上的存在时间更长）生命周期，但这其实也相当丑陋。经过权衡，我感觉用丑陋的生命周期看起来可能略好一些（因为 `.clone()` 是肉眼可见的丑，而赋予长的生命周期只是逻辑上的丑。

> 对 Rust 的另一个体悟：对于本人这种又菜又有代码洁癖的人，写 Rust 简直就是自我心理折磨。

符号的值怎么表示则需要好好设计一番：

* 常量符号：常量符号的值一定是能在编译期就计算出来的整型常量，所以可以用 `i32` 表示
* 变量符号：怎么表示变量符号的值则是设计符号表的最大难点。在 Koopa IR 中，会用 `alloc` 语句来为一个变量“申请内存”，之后每一次需要读取/写入这一变量时，其实都是对 `alloc` 语句所表示的“变量内存”进行操作。比如下面的例子中，在使用 `load` 操作符想要读取 `x` 的值时，使用的操作数实则就是 `alloc i32` 这句语句！想清楚这一点之后就可以发现，变量符号的值可以用“为这个变量符号申请内存时使用的 `alloc` 语句来表示。（悟到这点之后，就会觉得 Koopa IR 里面对语句和数值进行统一抽象的策略是真的有道理）
  ```
    // int x = 10;
    @x = alloc i32
    store 10, @x

    // x = x + 1;
    %0 = load @x
    %1 = add %0, 1
    store %1, @x

    // return x;
    %2 = load @x
    ret %2
  ```

所以符号表的数据结构可以设计成下面这样：

```rust
pub enum SymbolInfo {
    Const(i32),
    Variable(Value), // This value should point to an "Alloc" in the IR.
}

pub struct SymbolTable<'s> {
    table: HashMap<&'s str, SymbolInfo>,
}
```

然后在 `IrgenEnv` 中加入用于表示符号表的字段即可。不过这会导致 `IrgenEnv` 这个数据结构就此也有了一个生命期参数，之后会牵涉到一些关于生命期的其他地方的修改。

#### 3. 符号表支持的操作和错误处理

本节涉及到符号表操作的地方和相应的行为逻辑如下：

1. 定义常量：
   行为：检查符号名是否已存在，如果已存在，报错（符号重名）；否则，把键值对插入符号表中
2. 定义变量：
   行为：检查符号名是否已存在，如果已存在，报错（符号重名）；否则，把键值对插入符号表中
3. 赋值变量：
   行为：检查符号名是否已存在，如果不存在，报错（未定义符号）；如果已存在并且是常量符号，报错（给常量赋值）；否则，进行正常的后续处理
4. 常量初始化时的常量求值：
   行为：进行常量求值时可能会遇到符号，如果遇到符号，就从检查符号表中是否存在符号名，如果不存在，报错（未定义符号）；如果已存在并且是变量符号，报错（用变量初始化常量）；否则，正常进行求值操作

所以符号表需要支持的操作为：

* [X] 插入新的常量符号
* [X] 插入新的变量符号
* [X] 检查一个符号名是否存在
* [X] 给定符号名，查询其对应的值

进行错误处理时需要新增的错误类型有：

* [X] 符号重名
* [X] 未定义符号
* [X] 给常量赋值
* [X] 用变量初始化常量

#### 4. 生成 `alloc` `store` `load` IR 语句

* [X] `alloc` ：在生成 `VarDecl` 的 IR 时，创建 `alloc` 语句，并且把 `<var_name, alloc>` 键值对插入到符号表中。
* [X] `load` ：在生成 `LVal` 的 IR 时，创建 `load` 语句，`load` 语句的操作数即为该符号对应的 `alloc` 值
* [X] `store` ：在生成 `VarDef` 和 `Stmt::Assign` 的 IR 时，创建 `store` 语句，把需要存储的操作值存到符号对应的 `alloc` 里面去

#### 5. 编译期求常量的值

本编译器需要能做到在定义常量时就把其数值给计算出来。

在 lab 要求中，只要求了对常量的初始化需要做到编译期求出数值，但是变量的初始化是不用做这个的（也做不了）。所以这意味着，在这里需要在一定程度上把常量和变量分开对待。

我在这里想到了三种解决方案：

1. 在 `IrgenEnv` 中新增一个指示变量，用于判断当前是否处在求常量的值的过程中，如果是，那么就对所有的表达式在 `generate_koopa` 中直接进行求值，返回 `Integer` 值。
2. 专门设计一系列新函数用于仅处理求表达式的数值结果的问题，并且在 `ConstDef::generate_koopa` 中调用这类函数来求 `const_init_val` 的值。
3. 在所有表达式的 `generate_koopa` 中都无脑进行一个判断，如果发现所有的操作数都是数值，那么就直接进行计算并返回数值。这样的优化程度最高（会把所有能在编译期计算出来的数值都计算出来，不仅仅是常量初始值），但是感觉要多写一堆 `if` 有点小小丑

看了一下发现 `kira-rs` 用的是第二种策略，于是作者保守起见也用了第二种策略（但是第三种策略的优化方案可以考虑在之后实现一下）。设计了一个新的 Trait 专门用于进行表达式的常量求值，放在了 `irgen::eval` 模块里，所有 `ConstInitVal` 及其衍生出的词法单元都需要实现这个 Trait 。

```rust
pub trait Evaluate {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError>;
}
```

### 其他

* 因为需要在 `generate_koopa` 中调用 `env` 的关于符号表的操作，而符号表有生命周期参数，所以把 `generate_koopa` 函数的声明改成了 `fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError>` （就是在 `IrgenEnv` 后面加上了一个和 `self` 一样的生命周期参数 `'ast`）。我觉得这其实是丑陋的做法，因为这给予了符号表中的字符串引用一个可能远超其实际被用到的实际的生命周期，但是暂时不想改了，之后有精力的话再做一下这里生命周期更精细的管理。
* 幽默时刻：作者发现自己之前几个 level 中一直在用 `Ok(self.xxx.generate_koopa()?)` 这种写法作为返回值。（直接写成 `self.xxx.generate_koopa()` 不就好了吗喂！）给全改过来了
* lv4前端这个 `multiple_returns` 的用例好坑……文档里没说清楚，对于出现多个 `return` 语句的情况，第一次 `return` 之后的所有指令都应该被屏蔽（指直接不生成对应的 Koopa IR 语句）

非常好，Lv4 前端顺利过了！

## Lv.4 常量和变量 Backend

看完文档，作者觉得听树洞的话先跳过 Lv3 后端真是太明智的决定了。如果先做 Lv3 后端的话到 Lv4 后端还得把代码重写一遍。

原来 Lv4 只要实现一种最最最简单的寄存器分配方案（全放在栈上），更 fancy 的寄存器分配方案是要等到 Lv9+ 才写的（虽然能预感到，届时会写得极度痛苦，很可能要重构整个后端代码）。

### 功能梳理

因为还不涉及到“函数参数”和“保存的寄存器”，所以只需要考虑局部变量就可以。

需要为每个变量（即 `alloc` 语句）和 Koopa IR 的有返回值的指令分配栈上的内存空间。

需要为函数生成 Prologue 代码和 Epilogue 代码，用于操作栈指针 `sp` ，分配栈帧。

需要解决的问题如下：

* [X] **栈帧的大小如何计算？** 栈帧大小是针对函数而言的，所以这个计算一定是在 `FunctionData` 一层做的。需要在这一层遍历一个函数的所有指令以计算出栈帧大小。具体来说，每个 `alloc` 分配 4 字节，其他有返回值的指令（文档中用 `is_unit`() 判断，我后来觉得用 `match value_data.ty().king()` 来看也行）也分配 4 字节。最后把分配的字节数向上取整到 16 的倍数。
* [X] **如何管理 “变量（即  `alloc` 语句）和 Koopa IR 指令” 与其存储位置的对应关系？** 需要一个记录了 Koopa IR 指令和其存储位置在栈中的偏移量的对应的哈希表。
* [X] **怎么生成 Prologue 代码？** 需要在 `FunctionData` 这层生成，因为 Prologue 是针对函数而言的。计算出来栈帧大小之后直接就可以生成 Prologue 需要的更新栈指针的语句。需要特别注意 `addi` 的立即数范围限制！
* [X] **怎么生成 Epilogue 代码？** 直接在处理 `ret` 时生成 Epilogue 就行。Epilogue 用于复原栈指针的值。
* [X] **如何为每个 Koopa 语句生成汇编代码？** 指导思想是：如果 Koopa 语句要用到别的语句的返回值，就用一个 `lw` 从对应语句的内存位置取值；如果 Koopa 语句本身有返回值，那就生成一个 `sw` 到本语句对应的内存位置。同样需要注意 `lw` 和 `sw` 的立即数范围！

所以 Koopa 语句与栈偏移量的对应信息也需要放在 `CodegenEnv` 里面作为环境的一部分，因为随时都可能会使用到。

### 数据结构设计

需要为每个函数维护一个记录内存栈使用信息的数据结构，称其为 `StackInfo` 好了。

`StackInfo` 里面存了当前函数的栈帧大小，以及 Koopa IR 语句与栈偏移量对应的哈希表。Lv4 只有一个函数，所以目前只要在 `CodegenEnv` 里面存一个 `StackInfo` 就行。之后可能需要按照调用层次组织成一个 `Vec<StackInfo>` 。

更新之后的数据结构如下：

```rust
pub struct StackInfo {
    frame_size: i32,
    offset_table: HashMap<Value, i32>,
}

pub struct CodegenEnv<'p> {
    program: &'p Program,
    cur_func: Option<Function>,
    stack_info: StackInfo,
}
```

### 其他

因为把 `le` 和 `ge` 的汇编生成方式写错导致浪费了半天时间……

以 `le` 为例，正确的方法应该是先 `sgt` 然后再 `seqz`，作者一开始居然天真地直接把两个操作数顺序对换然后 `sgt` 了，这样会导致在两个操作数相等的时候 `le` 操作返回了 `0` ，这太蠢了，鉴定为 ljl 体系结构（也可能是小学数学）没学好导致的。

## Lv.5 语句块和作用域

本 level 需要实现处理语句块和作用域的功能。

本 level 只需要修改前端的代码，后端不用动。

### 功能梳理

#### 1. 修改词法/语法分析和 AST

需要支持仅由 0 或 1 个 `stmt` 构成的语句。

再记录一个相关 `ret` 的细节：现在 ret 允许直接 `return ;` 这样不指定返回值。这应该是 SysY 里的 UB。在实现上，最好一开始搞一个 `%ret = alloc i32`，然后如果有返回值的话再把返回值存到 `%ret` 里面，最后 `load` 这个 `%ret` 里面的值作为最终 `ret` 的值。（不过这种情况直接补上一个   `return 0` 应该也行，这也算是 UB 的一种实现方式吧。）

#### 2. 多重符号表

需要为每个语句块都维护一个自己的符号表。

支持多重的符号表：把现有的 `sym_tab` 字段改成一个栈的数据结构，并且在进出语句块时更新这个栈，插入/弹出相应的符号表。因为 Rust 里面没有原生的 `Stack` 实现，所以用 `Vec` 来模拟栈。

插入符号：只在当前语句块的符号表中插入新的键值对。

查询符号定义：需要支持跨符号表查询。如果当前符号表中有这个符号，那么就找到了；否则，就一层一层往更大的语句块中找，直到找到符号/在最大的语句块中也找不到符号为止。

> 特别注意：Lv5 中所说的语句块/作用域对应的是 AST 中的 `Block` ，而非 Koopa IR 中的 `BasicBlock` ！AST 中的 `Block` 和 Koopa IR 中的 `BasicBlock` 不是一回事！前者的作用仅限于不同的语句块表示了不同的作用域，后者则会在之后处理 `if-else` 这类语句的时候用到，更像是每个 `BasicBlock` 对应一个 Label 。`BasicBlock` 的特点是，每个 `BasicBlock` 最后都要以 `ret` / `jmp` / `br` 结尾。

### 数据结构设计

综合上面的需求，修改后的数据结构如下：

```rust
pub struct IrgenEnv<'s> {
    cur_func: Option<Function>,
    cur_bb: Option<BasicBlock>,
    cur_bb_returned: bool,
    sym_tab: Vec<Box<SymbolTable<'s>>>,
    cur_scope_id: i32,
}
```

其中 `sym_tab` 存储了每个作用域自己的符号表。 `cur_scope_id` 用于给每个作用域都分配一个唯一的编号，这个编号会被放在 Koopa IR 中变量符号的名字中，用来避免不同作用域中同名的符号在 IR 中重名。

### 进入/退出作用域

每当进入一个新的作用域时，调用 `IrgenEnv::push_scope()` 方法创建一个新的符号表。

```rust
    pub fn push_scope(&mut self) {
        self.sym_tab.push(Box::new(SymbolTable::new()));
        self.cur_scope_id += 1;
    }
```

每当退出一个作用域时，调用 `IrgenEnv::pop_scope()` 方法删掉末尾的符号表。

```rust
    pub fn pop_scope(&mut self) {
        self.sym_tab.pop();
    }
```

进入/退出作用域的发生时间：

1. 在 `Funcdef` 中创建最初时的作用域并进入这个作用域，然后生成这个函数的 IR，最后依然是在 `Funcdef` 中退出这个最初的作用域
2. 在处理 `Stmt::Block` 时，进入新的作用域，生成这个 `Block` 的 IR，并退出这个作用域。

   ```rust
               Self::Block(block) => {
                   env.push_scope();
                   block.generate_koopa(program, env)?;
                   env.pop_scope();
               },
   ```

别的代码都很好写了。

### 其他

把前端的生成的部分中常用到的重复代码给提炼成了 `IrgenEnv` 的成员函数：

```rust
    pub fn new_value(&self, program: &'s mut Program) -> LocalBuilder<'s> {
        let cur_func = self.cur_func.unwrap();
        let cur_func_data = program.func_mut(cur_func);
        cur_func_data.dfg_mut().new_value()
    }

    pub fn dfg_mut(&self, program: &'s mut Program) -> &'s mut DataFlowGraph {
        let cur_func = self.cur_func.unwrap();
        let cur_func_data = program.func_mut(cur_func);
        cur_func_data.dfg_mut()
    }

    pub fn new_inst(&self, program: &'s mut Program) -> &'s mut InstList {
        let cur_func = self.cur_func.unwrap();
        let cur_bb = self.cur_bb.unwrap();
        let cur_func_data = program.func_mut(cur_func);
        cur_func_data.layout_mut().bb_mut(cur_bb).insts_mut()
    }
```

这大大简化了在 Koopa IR 中创建新的值和指令的代码，比如现在创建一个 `alloc` 语句只需要：

```rust
        let alloc = env.new_value(program).alloc(ty);
        env.dfg_mut(program).set_value_name(alloc, Some(format!("@{}_{}", self.ident, env.get_cur_scope_id())));
        env.new_inst(program).push_key_back(alloc).unwrap(); 
```

Lv5 只需要修改前端，不用修改后端。顺利通过！

## Lv.6 `if` 语句

本 level 需要实现处理 `if/else` 语句的功能。

### 功能梳理

#### 1. 语法/词法分析

如何解析 `if/else` 时的二义性问题？

在 [Wikipedia](https://en.wikipedia.org/wiki/Dangling_else) 上找到了这种解决方法（甚至把 `while` 语句的处理也涵盖了） ：

```plaintext
statement: open_statement
         | closed_statement
         ;

open_statement: IF '(' expression ')' statement
              | IF '(' expression ')' closed_statement ELSE open_statement
              | WHILE '(' expression ')' open_statement
              ;

closed_statement: simple_statement
                | IF '(' expression ')' closed_statement ELSE closed_statement
                | WHILE '(' expression ')' closed_statement
                ;

simple_statement: ...
                ;
```

基于这个思想修改 EBNF 如下（ `while` 语句的暂时等到下个 level 再做吧 ）：

```plaintext
// Stmt          ::= OpenStmt | ClosedStmt;
// OpenStmt      ::= "if" "(" Exp ")" Stmt
//                 | "if" "(" Exp ")" ClosedStmt "else" OpenStmt;
// ClosedStmt    ::= SimpleStmt
//                 | "if" "(" Exp ")" ClosedStmt "else" ClosedStmt;
// SimpleStmt    ::= LVal "=" Exp ";"
//                 | [Exp] ";"
//                 | Block
//                 | "return" [Exp] ";";
```

直观理解的话， `ClosedStmt` 表示的是一个已经闭合了的 `if/else` 或者一个非条件分支语句，而 `OpenStmt` 则表示未闭合的分支语句。

#### 2. 基本块和控制转移指令

这里好像分析起来真得用到上理论课的时候说的 SDT 的思想，看来理论课的知识也不是完全没用的。

先讨论前端的生成方案（这应该也是本 level 的重点）。

对于有 `if/else` 两个分支的语句，生成 IR 的流程如下：

* 生成三个 `BasicBlock` 基本块（也可以说是标签），分别代表 `then` 分支、`else` 分支、`end`
* 生成 `Exp` 的 IR，并生成 `br` 指令。`br` 指令的三个操作数为：计算出的 `Exp` 的值、`then` 分支、`else` 分支
* 切换当前基本块到 `then` ，生成该分支内的语句的 IR。分支以 `jump end` 作为结尾。
* 切换当前基本块到 `else` ，生成该分支内的语句的 IR。分支以 `jump end` 作为结尾。
* 切换当前基本块到 `end` 。

对于只有 `if` 一个分支的语句，则只要 `then` 和 `end` 两个基本块即可。其余的大致思路和上面相似。

#### 3. 处理 `ret`

之前的 level 中，直接在遇到 `SimpleStmt::return` 时生成 `ret` 语句。但是这样在有分支的情况下会出现问题。如果分支中有 `Return` 语句，那么这样的话就会在分支中生成 `ret` 语句，这会很难处理。

作者的解决方案是：在函数的一开始就创建一个 `%exit` 基本块，这个基本块专门用来生成 `ret %ret` 语句，并且把这个 `%exit` 基本块直接放在 `IrgenEnv` 结构体中，作为全局环境的一部分。然后，如果在分支中，如果有 `Return` 语句，就把返回值加载到 `%ret` 中，然后直接  `jump %exit` 。还需要注意的是，如果一个分支基本块中已经有了 `Return` （即会在 IR 中生成 `jump %exit` 了），就不要再生成 `jump %end` 语句了，每个基本块最后只能有且仅有一个跳转指令。

用这种策略之后，下面的 SysY 代码会被翻译成这样的 Koopa IR：

SysY：

```plaintext
int main() {
  int a = 1;
  if (a) {
    return a + 1;
  } else {
    return a + 2;
  }
  return a;
}
```

对应的 Koopa IR：

```plaintext
fun @main(): i32 {
%entry:
  %ret = alloc i32
  @a_1 = alloc i32
  store 1, @a_1
  %0 = load @a_1
  br %0, %then_0, %else_0

%then_0:
  %1 = load @a_1
  %2 = add %1, 1
  store %2, %ret
  jump %exit

%else_0:
  %3 = load @a_1
  %4 = add %3, 2
  store %4, %ret
  jump %exit

%end_0:
  %5 = load @a_1
  store %5, %ret
  jump %exit

%exit:
  %6 = load %ret
  ret %6
}

```

#### 4. 处理短路求值

针对 `or` 和 `and` 需要进行短路求值，这是通过类似分支的方法来做的。

可以把 `or` 和 `and` 语句翻译成如下的分支形态。

求 `lhs || rhs` 的值时，可以翻译成下面的模式：

```plaintext
  alloc %res
  store (lhs == 0), %res
  branch (lhs == 0), %or_rhs, %or_end

%or_rhs:
  store (rhs == 0), %res
  jump %or_end

%or_end:
  load %res
```

求 `lhs && rhs` 的值时，可以翻译成下面的模式：

```
  alloc %res
  store (lhs == 0), %res
  branch (lhs == 0), %or_end, %or_rhs

%or_rhs:
  store (rhs == 0), %res
  jump %or_end

%or_end:
  load %res
```

这种分支的情况应该比较简单，所以在处理这里的基本块时没有考虑会有 `Return` 语句的问题了（逻辑表达式里应该不会有返回值什么的吧……？）。在这里记录一下，万一以后被打脸了。

### 其他

最终还是决定给标签起名字的时候加全局编号（主要是分支语句有else有的没有，不人为设置编号的话没法让所有统一分支中的标签名字中有一样的编号），然后给变量起名字的时候就不要全局编号了（没必要）。
