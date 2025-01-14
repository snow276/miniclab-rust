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

本 level 需要实现处理表达式 (一元/二元) 的功能。

说实话，回头来看觉得本章挺简单的。作为使用 Rust 的用户，本章最大的难度在于需要理解 Koopa IR 的代码结构，理解之后就都很顺利了。

在此就简单提一些对 Koopa IR 的认识：

1. Koopa IR 中，控制流和数据流是分开的。`Function` 、 `Value` 等数据结构其实都是指针，通过这些指针可以获得其指向的实际数据 `FunctionData` 、 `ValueData` 等。这是出于 Rust 的所有权机制考虑的。这些表示指针的数据类型都很轻量级，并且也都是有 `Copy` Trait 的，所以在选择存储什么全局信息时，作者认为的原则是存指针而不存数据，需要数据时再根据指针去找。
2. Koopa IR 中，把数值和指令进行了统一抽象，都是 Koopa IR 中的“值”。一个指令（比如 `add` 、`ret`）的操作数可以是数值，也可以是其他指令。这使得在生成内存形式的 IR 时代码可以写得很简洁。

在有了以上的认识之后，本 level 的前端部分具体的实现方案就比较简单，这里就不再仔细介绍了。

Lv3 的后端暂时先跳过了，根据树洞给出的建议，Lv3 的后端和 Lv4 一起写比较好。

## Lv4. 常量和变量 Frontend

本 level 需要实现处理常量/变量定义和赋值语句的功能。

作者在完成本 level 时直接把 Lv4.1 和 Lv4.2 放在一起梳理和实现了。

### 功能梳理和实现步骤

#### 1. 修改词法/语法分析和 AST

* [X] 按照要求修改 `sysy.lalrpop` 文件和 `ast.rs` 文件

在处理本节新增的 EBNF `{...}` 时，参考了 kira-rs 中的方法，使用 `<mut const_def_list: ("," <ConstDef>``)*>` 这种方式可以接受任意多个项并且把结果存为一个 `Vec` 。

#### 2. 符号表数据结构

本节最主要的更改就是需要维护符号表。符号表维护了从符号的名字到符号的值的对应。

符号的名字用某种字符串表示。这里作者选择了用 `&str` 。（ `&str` 和 `String` 都实现了 `Hash` 和 `Eq` Trait，所以这里用 `String` 应该也是可以的。）

> 对 Rust 的一个体悟：写 Rust ，不论用引用还是不用引用，都可以写得是否丑陋。对于设计符号表的这个例子，如果用 `String` ，那有可能会在设计表操作时牵扯到一些作者意料之外的所有权转移问题，想要规避这些所有权转移问题的话，滥用  `.clone()` 就好了，但这可以说是相当丑陋；如果用 `&str` ，那就牵涉到怎么设计生命周期的问题，而最愚蠢却又能过编译的方式就是给它一个很长的（比逻辑上的存在时间更长）生命周期，但这其实也相当丑陋。经过权衡，作者感觉用丑陋的生命周期看起来可能略好一些（因为 `.clone()` 是肉眼可见的丑，而赋予长的生命周期只是逻辑上的丑。

> 对 Rust 的另一个体悟：对于作者本人这种又菜又有代码洁癖的人，写 Rust 简直就是自我心理折磨。

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

## Lv4. 常量和变量 Backend

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

## Lv5. 语句块和作用域

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

## Lv6. `if` 语句 Frontend

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

* 最终还是决定给标签起名字的时候加全局编号（主要是分支语句有else有的没有，不人为设置编号的话没法让所有统一分支中的标签名字中有一样的编号），然后给变量起名字的时候就不要全局编号了（没必要）。
* 又一个很坑的地方！提交之后在 Lv6 隐藏测试中的 `13_branch2` 这个 case 错了。还好树洞上有好心人提示了是以下的情况导致的：

  ```plaintext
  int main() {
    if (1) return 0;
    else return 1;
  }
  ```

  这种情况下作者最开始交上去的代码会生成如下的 IR：

  ```
  fun @main(): i32 {
  %entry:
    %ret = alloc i32
    br 1, %then_0, %else_0

  %then_0:
    store 0, %ret
    jump %exit

  %else_0:
    store 1, %ret
    jump %exit

  %end_0:

  %exit:
    %0 = load %ret
    ret %0
  }

  ```

  可以发现问题出现在 `%end_0` 这个分支。解决方案是，在这种情况下给这个分支手动加上一条 `jump %exit` 指令。至于怎么加呢？通过一些分析可以发现，对于一个函数而言，这种情况只可能出现在 `%exit` 之前的最后一个基本块中。所以只需要在 `FuncDef::generate_koopa()` 中加上以下的代码即可：具体而言，在生成了 `Block` 的代码之后，这之后的 `env.cur_bb` 指向的基本块就是 `%end` 之前的最后一个基本块，所以只需要在此时判断当前的基本块是否已经有过 `Return` 语句，如果没有的话，就手动加上一条 `jump %exit` 即可。

  ```rust
  impl<'ast> GenerateKoopa<'ast> for FuncDef {
      type Out = ();

      fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
  	// ...

          self.block.generate_koopa(program, env)?;
          env.pop_scope();

  	// 增加的语句从这里开始
          if !env.is_cur_bb_returned() {
              let jump = env.new_value(program).jump(exit);
              env.new_inst(program).push_key_back(jump).unwrap();
          }
  	// 增加的语句到这里结束

          env.layout_mut(program).bbs_mut().extend([exit]);
  	// ...
      }
  }

  ```

## Lv6. `if` 语句 Backend

### 功能梳理和实现

Lv6 的后端相比之前的后端多出来的只有  `jump` 和 `br` 两类 IR 指令、以及需要在汇编里加上一些标签。

写后端的时候会涉及到怎么查看一个 IR 基本块的名称（用来生成汇编中的标签名称），稍微看看 Koopa 源码就发现可以这么处理（作者将其作为 `CodegenEnv` 的一个成员函数）：

```rust
    pub fn get_label(&self, bb: BasicBlock) -> &str {
        let cur_func = self.cur_func.unwrap();
        let cur_func_data = self.program.func(cur_func);
        cur_func_data.dfg().bb(bb).name().as_ref().unwrap()
    }
```

这样使用 `let label: &str = env.get_label(bb)[1..]` 就能够获得一个 IR 基本块对应的汇编标签名了（就是把基本块的名字的第一个字符 `%` 给去掉）。

处理 `jump` 和 `br` 很简单，看看文档里的例子就能看出来方法了，不多赘述。

### 其他

一个有点坑的地方：如果在 IR 生成的时候给两个基本块一样的名字，那么在输出 IR 时两个基本块的名字后面会被自动加上编号，然而在用上面的方法获取基本块的名字时居然并不会自动加区分的编号，而是会直接得到一样的名字。作者在 Lv6 前端生成逻辑运算语句的短路逻辑时给所有的 `||` 和 `&&` 的基本块用了一样的名字，在前端没事，在后端就报错了……

于是作者又在前端里给逻辑运算语句的基本块都加了独立的编号，解决了这个问题。

> 感想：最开始设计一个数据结构时，总是希望它能越好看越好、越简洁越好。可是当它里面的东西越来越多越来越丑之后……再往里面加新东西时，心里就没有什么波澜了。

好耶！至此为止，Lv6 的前后端的所有测试全部通过了！

## Lv7. `while` 语句

本 level 需要实现处理 `while` 语句的功能。

本 level 只需要改前端，后端不用动。

### 功能梳理

#### 1. 语法/词法分析

在语法上处理 `while` 的思想在本文档的 Lv6 前端部分已经写出来了。这里直接记录一下按照此思想修改的 EBNF。

```plaintext
// Stmt          ::= OpenStmt | ClosedStmt;
// OpenStmt      ::= "if" "(" Exp ")" Stmt
//                 | "if" "(" Exp ")" ClosedStmt "else" OpenStmt;
//                 | "while" "(" Exp ")" OpenStmt;
// ClosedStmt    ::= SimpleStmt
//                 | "if" "(" Exp ")" ClosedStmt "else" ClosedStmt;
//                 | "while" "(" Exp ")" ClosedStmt;
// SimpleStmt    ::= LVal "=" Exp ";"
//                 | [Exp] ";"
//                 | Block
//                 | "break" ";"
//                 | "continue" ";"
//                 | "return" [Exp] ";";
```

#### 2. 处理 `while` 的基本快和控制转移

对于 `while` 语句，用下面的方式处理：

* 生成三个基本块 `while_cond` 、 `while_body` 和 `while_end` 。
* 在进入 `while_cond` 基本块之前，先在当前基本块中生成一句跳转到 `while_cond` 的跳转指令。
* `while_cond` 进行条件判断，如果条件为真，跳转到 `while_body` ；如果条件为假，跳转到 `while_end` 。
* `while_body` 生成循环体中的语句，并在最后无条件跳转至 `while_cond` 。
* `while_end` 什么事也不用干。

#### 3. 处理 `break` 和 `continue` 语句

从行为上看，`break` 就是要生成一条跳转到 `while_end` 的跳转指令，`continue` 就是要生产一条跳转到 `while_cond` 的跳转指令。

在设计处理思路之前，作者意识到有如下几个需要注意的地方：

1. `while` 循环可能是嵌套的，而 `break` 和 `continue` 语句应该作用于最近的一个 `while` 循环。
2. `break` 和 `continue` 语句不一定在 `while_body` 基本块中，比如如果 `while` 中有一个 `if` 语句，而 `break` 在这个 `if` 的某个分支中，那么 `break` 在 IR 中其实就不在 `while_body` 基本块里而是在 `if` 分支的基本块里了。

考虑到以上两点，作者想到了一种简单的处理方案：在全局环境 `IrgenEnv` 中新增  `cur_while_cond: Option<BasicBlock> `和 `cur_while_end: Option<BasicBlock>` 两个成员变量，表示当前“最近的 `while` 循环”的 `while_cond` 和 `while_end` 基本块。然后，只需要在每次处理 `while` 语句时，先保存这两个量的旧值，然后把这两个量的值设置为当前 `while` 创建出的相应基本块，最后再把这两个量的值恢复成旧值就行。

在 `OpenStmt` 和 `ClosedStmt` 的 `While` 分支中，与这部分相关的代码如下：

```rust
            Self::While(exp, stmt) => {
                let old_while_cond_bb = env.get_cur_while_cond_bb();
                let old_while_end_bb = env.get_cur_while_end_bb();

                let wid = env.new_while_id();
                let cond_bb = env.new_bb(program).basic_block(Some(format!("%while_cond_{}", wid).into()));
                let body_bb = env.new_bb(program).basic_block(Some(format!("%while_body_{}", wid).into()));
                let end_bb = env.new_bb(program).basic_block(Some(format!("%while_end_{}", wid).into()));
		// ...

                env.set_cur_while_cond_bb(Some(cond_bb));
                env.set_cur_while_end_bb(Some(end_bb));

		// ...

                env.set_cur_while_cond_bb(old_while_cond_bb);
                env.set_cur_while_end_bb(old_while_end_bb);
                Ok(())
            }
```

在 `SimpleStmt` 的 `Break` 和 `Continue` 分支中，只需要用以下简单的方法生成相应的跳转语句即可。（这样的设计顺便还能进行错误处理，能够报出 `break` 与 `continue` 语句不在循环内部的错误，非常好！）

```rust
            Self::Break => {
                if let Some(while_end_bb) = env.get_cur_while_end_bb() {
                    let jump = env.new_value(program).jump(while_end_bb);
                    env.new_inst(program).push_key_back(jump).unwrap();
                    env.set_cur_bb_returned(true);
                } else {
                    return Err(IrgenError::BreakOutsideLoop);
                }
            },
            Self::Continue => {
                if let Some(while_cond_bb) = env.get_cur_while_cond_bb() {
                    let jump = env.new_value(program).jump(while_cond_bb);
                    env.new_inst(program).push_key_back(jump).unwrap();
                    env.set_cur_bb_returned(true);
                } else {
                    return Err(IrgenError::ContinueOutsideLoop);
                }
            },
```

在此还想顺便谈谈 `IrgenEnv` 的 `cur_bb_returned: bool` 这个成员变量。最初加上这个成员变量是为了应付 Lv4 的隐藏测试样例 `multiple_returns` ，当时为了判断是否已经出现过 `Return` 语句而把这个成员变量加到了全局环境中，没想到后面居然这么有用。

在 Koopa IR 中规定，每个基本块都有且仅有一条控制转移指令（ `br` 或 `jump` 或 `ret` ），并且这个控制转移指令一定是基本块的最后一条指令。

在完成本 level 时，我发现这个 `cur_bb_returned` 的意义可以进行扩展，不仅用于表示当前基本块是否已经存在了 `Return` 语句，而是直接表示当前基本块是否已经存在了控制转移语句。这样的话，就可以把控制转移相关的逻辑给统一起来。比方说，在处理完 `break` 和 `continue` 语句后，因为处理这些语句已经给当前基本块加上了控制转移语句，所以就可以把 `cur_bb_returned` 设计成 `true` 。而在基本块的生成逻辑中（即处理 `if` 和 `while` 的逻辑中），都可以用形如下面的代码来给还没有生成控制转移语句的基本块加上符合预期行为的控制转移指令（比如下面的例子就是给 `while_body` 的最后加上一句跳转到 `while_cond` 的跳转语句）。

```rust
                env.layout_mut(program).bbs_mut().extend([body_bb]);
                env.set_cur_bb(body_bb);
                env.set_cur_bb_returned(false);
                stmt.generate_koopa(program, env)?;
                if !env.is_cur_bb_returned() {
                    let jump = env.new_value(program).jump(cond_bb);
                    env.new_inst(program).push_key_back(jump).unwrap();
                }
```

所以说，做完 Lv7，作者觉得 `cur_bb_returned` 这个变量名已经不太合适了，应该改成 `cur_bb_control_transferred` 或者 `cur_bb_ended` 才对）。

另一点很奇妙的是，虽然基本块可能有很多个，但是实际编程时只要用这一个 `bool` 就足够应付所有的基本块了。这是因为 Koopa IR 生成基本块的过程是线性的，当一个基本块遇到控制转移语句结束了之后，就再也不会用到这个基本块了，一定会进入一个新的基本块，所以在生成不同基本块的过程中复用这一个 `bool` 是完全可以的。

### 其他

用作者的方法生成出来的 `while` 的 IR 表示和 Lv7.2 lab 文档中助教写的并不太一样，但作者实在没想出来助教那样的写法是怎么弄出来的，不管了。

## Lv8.1 函数和全局变量 Frontend

Lv8 的内容非常多，拆成一个个小章节一点点来吧。

Lv8.1 需要实现函数的定义和调用。

### 功能梳理

#### 1. 语法/词法分析

按照要求修改 `sysy.lalrpop` 和 `ast.rs` 就行，这部分不麻烦。

修改以后，原来的 `CompUnit` 被替换为了 `SimpleCompUnit` 这个结构（存放单个函数or全局定义），而 `CompUnit` 则存放了 `SimpleCompUnit` 的列表。

#### 2. 符号表和作用域

首先，根据文档的建议，作者把全局范围内所有的函数 (包括之后章节中会出现的全局变量) 都放在同一个作用域内，即全局作用域，并让这个全局作用域位于所有局部作用域的外层。具体而言，在 `CompUnit` 中创建一个全局作用域，然后在处理每个 `FuncDef` 的时候在创建每个函数自己的作用域即可。

然后，需要确认是否应该把函数符号和变量/常量符号统一存在一张符号表里。在做这个选择的时候，作者考虑到了以下几点：

1. SysY 是允许函数内的局部变量名和函数名相同的。比如以下的程序是合法的：

   ```plaintext
   int f() {
       return;
   }

   int main() {
       int f = 1;
       f();
       return 0;
   }
   ```
2. SysY 要求 `CompUnit` 的顶层变量/常量声明语句（对应 `Decl` ），函数定义（对应 `FuncDef` ）都不可以重复定义同名标识符（ `IDENT` ），即使标识符的类型不同也不允许。
3. SysY 程序不存在“在函数中定义函数”的情况，所以所有的函数符号一定是在全局作用域中。
4. 局部变量会覆盖同名全局变量。

考虑到这几点，作者暂时选择了把函数符号和变量/常量统一存在一张符号表中。这样的话，函数符号必然是只存在于全局作用域的符号表的。不过这样的话，需要在处理函数调用的时候特别注意一下，在检索函数符号的时候只从全局作用域中搜索（否则可能查到局部符号，导致出错）。

还有一些别的要解决的小问题，解决方案列在下面：

在符号表中用什么代表函数？作者使用了 Koopa IR 中的 `Function` 指针，通过这个 `Function` 指针可以获得 `FunctionData` ，里面含有所有关于这个函数需要的信息。

是否要在符号表中存函数的形参列表？不需要，因为可以通过 `Function` 指针获取 `FunctionData` ，而其中已经存了函数的形参信息。

所以，修改后的符号表长这样：

```rust
pub enum SymbolInfo {
    Const(i32),
    Variable(Value), // This value should point to an "Alloc" in the IR.
    Function(Function),
}
```

在插入和查询函数时，都直接从全局作用域的符号表中查询。

#### 3. 处理函数调用指令

在处理 `UnaryExp::FuncCall` 时生成 `call` 指令并将其加入指令列表中。

创建 `call` 这一 Koopa IR 值时，需要使用的参数是表示被调用函数的 `Function` 和表示实参列表的 `Vec<Value>` ，前者可以从符号表中查出来，后者队实参列表依次求值即可。

#### 4. 处理函数参数

这里主要讨论的是，在 `FuncDef` 中怎么处理函数的形参，以及怎么体现”把函数的实参存入对应的形参中“这个过程。

以下面这段 SysY 程序为例（其中调用了有两个参数的函数 `add()` ）。

```plaintext
int add(int a, int b) {
  return a + b;
}

int main() {
  return add(1, 2);
}
```

要想知道怎么处理函数的参数，需要理解 Koopa IR 中是怎么处理函数的。

在 Koopa IR 中，在使用 `program.new_func()` 创建函数时，需要传入的信息包括：1）函数在 IR 表示中的名字 2）函数的形参列表（用一个 `params_ty: Vec<Type>` 表示，毕竟形参列表其实就是一个类型列表，里面按顺序存放了各个形参的类型） 3）返回类型（用一个 `ret_ty: Type` 表示）。作者在 `FuncDef::generator_koopa` 中，用类似下面的代码在 Koopa IR 中创建函数：

```rust
        let mut params_ty = vec![];
        if let Some(func_f_params) = self.func_f_params.as_ref() {
            for func_f_param in &func_f_params.func_f_param_list {
                let ty = func_f_param.b_type.generate_koopa(program, env)?;
                params_ty.push(ty);
            }
        }
        let ret_ty = self.func_type.generate_koopa(program, env)?;
        let func = program.new_func(FunctionData::new(
            format!("@{}", self.ident), 
            params_ty.clone(), 
            ret_ty.clone()
        ));
```

在创建了 Koopa IR 内存形式的函数之后，Koopa IR 会自动为这个函数创建若干个 `Value` ，存储 `FunctionData` 的 `params: &[Value]` 字段中，分别用来存放函数的实参，函数调用时，函数的实参就会被存在这些用来 `Value` 中。作者这里的处理方式时：在函数的开头，就为每个形参都分配一块内存空间，并且把实参的值存到相应的为形参分配的空间中，这就完成了传参的过程。之后在函数体中，正常地使用这些为形参分配的空间即可。相关的代码和如下：

```rust
        if let Some(func_f_params) = self.func_f_params.as_ref() {
            let params = program.func(func).params().to_vec();
            for ((func_f_param, param_ty), param) in func_f_params.func_f_param_list.iter().zip(params_ty.iter()).zip(params.iter()) {
                let alloc_param = env.new_value(program).alloc(param_ty.clone());
                env.dfg_mut(program).set_value_name(alloc_param, Some(format!("@{}", func_f_param.ident)));
                env.new_inst(program).push_key_back(alloc_param).unwrap();
                env.new_symbol_var(&func_f_param.ident, alloc_param);
                let store_param = env.new_value(program).store(*param, alloc_param);
                env.new_inst(program).push_key_back(store_param).unwrap();
            }
        }
```

最终，上面的 SysY 程序翻译出的 IR 形式结果类似如下。

```plaintext
fun @add(%0: i32, %1: i32): i32 {
%entry:
  %ret = alloc i32
  @a = alloc i32
  store %0, @a
  @b = alloc i32
  store %1, @b
  %2 = load @a
  %3 = load @b
  %4 = add %2, %3
  store %4, %ret
  jump %exit

%exit:
  %5 = load %ret
  ret %5
}

fun @main(): i32 {
%entry:
  %ret = alloc i32
  %6 = call @add(1, 2)
  store %6, %ret
  jump %exit

%exit:
  %7 = load %ret
  ret %7
}

```

#### 5. 函数返回值类型对函数的调用指令和返回指令的影响

首先看返回值类型对返回指令的影响。

函数有 `int` 和 `void` 两种，其中 `void` 是本节新增的，其相比 `int` 返回值的特殊之处在于：返回值类型为 `void` 的函数，在结束时的 `ret` 中不需要任何的参数。

在之前处理 `int` 返回值的函数时，作者是先在函数中创建一个 `%ret` 变量符号，然后再遇到 `Return` 指令时把返回值给 `store` 进 `%ret` 中，最后再无脑补一句 `ret %ret` 。然而，这套操作对于 `void` 返回值的函数是行不通的。

解决方案也很简单。作者在全局环境中新增了一个成员变量 `cur_func_type: Option<Type>` ，用于表示当前函数的返回值类型。如果是 `void` 类型的函数，一开始就不用在函数的符号表中创建 `%ret` 符号，之后在每次需要生成 `ret` 语句时也不用加上任何参数。比如，修改后创建 `ret` 语句的代码大概就长这样：

```rust
        match env.get_cur_func_type().unwrap().kind() {
            TypeKind::Int32 => {
                let alloc_ret = match env.get_symbol("%ret").unwrap() {
                    SymbolInfo::Variable(alloc) => *alloc,
                    _ => unreachable!()
                };
                let load = env.new_value(program).load(alloc_ret);
                let ret = env.new_value(program).ret(Some(load));
                env.new_inst(program).push_key_back(load).unwrap();
                env.new_inst(program).push_key_back(ret).unwrap();
            },
            TypeKind::Unit => {
                let ret = env.new_value(program).ret(None);
                env.new_inst(program).push_key_back(ret).unwrap();
            },
            _ => unreachable!()
        }
```

另一个需要注意的就是，SysY 中规定了“试图使用返回类型为 `void` 的函数的返回值是未定义行为”。所以，如果想要优雅地让编译器报出这个未定义行为，就不能再让 `generate_koopa` 中函数调用的返回值一定是 `Value` 了（因为函数调用的返回值可能啥都不是）。为此，可以参考 `kira-rs` 中的设计，抽象出一个 `ExpValue` 的 `Enum` 类，用于存放各种可能的表达式的值。

```rust
pub enum ExpType {
    Int(Value),
    Void,
}
```

把这个作为表达式相关的词法成分的 `generate_koopa` 的返回值类型，就可以在生成 IR 过程中报出使用类型为 `void` 的函数的返回值这个 UB 啦。

## Lv8.2 SysY 库函数 Frontend

Lv8.2 需要支持调用 SysY 库函数。

很好写，只需要在 `CompUnit` 中新增这些库函数的声明，并把库函数加入到全局作用域的符号表中就可以了。唯一需要动动脑子的就是去找 Koopa IR 中怎么创建一个函数声明。

在 `CompUnit::generate_koopa()` 中新增的代码大概长这样：

```rust
        let mut new_decl = |name: &'ast str , params_ty, ret_ty| {
            let func = program.new_func(FunctionData::new_decl(
                format!("@{}", name), 
                params_ty, 
                ret_ty
            ));
            env.new_func(name, func);
        };
        new_decl("getint", vec![], Type::get_i32());
        new_decl("getch", vec![], Type::get_i32());
        new_decl("getarray", vec![Type::get_pointer(Type::get_i32())], Type::get_i32());
        new_decl("putint", vec![Type::get_i32()], Type::get_unit());
        new_decl("putch", vec![Type::get_i32()], Type::get_unit());
        new_decl("putarray", vec![Type::get_i32(), Type::get_pointer(Type::get_i32())], Type::get_unit());
        new_decl("starttime", vec![], Type::get_unit());
        new_decl("stoptime", vec![], Type::get_unit());
```

## Lv8.3 全局变量和常量 Frontend

Lv8.3 需要支持全局的变量和常量。

### 功能梳理

#### 1. 语法/词法分析

本来以为按照本节新增的更改语法/词法分析的部分就行，结果没想到写完之后一运行出现了这个错误，具体来说是发生了归约-归约冲突：

```plaintext
    The problem arises after having observed the following symbols in the input:
      "int"
    At that point, if the next token is a `r#"[_a-zA-Z][_a-zA-Z0-9]*"#`, then the parser can
    proceed in two different ways.

    First, the parser could execute the production at
    /home/rocky/miniclab/sysy_compiler/src/sysy.lalrpop:38:16: 38:34, which would consume
    the top 1 token(s) from the stack and produce a `BType`. This might then yield a parse
    tree like
      "int"   ╷ VarDef ";"
      ├─BType─┘          │
      └─VarDecl──────────┘

    Alternatively, the parser could execute the production at
    /home/rocky/miniclab/sysy_compiler/src/sysy.lalrpop:81:3: 81:24, which would consume the
    top 1 token(s) from the stack and produce a `FuncType`. This might then yield a parse
    tree like
      "int"      ╷ Ident "(" ")" Block
      ├─FuncType─┘                   │
      └─FuncDef──────────────────────┘

    See the LALRPOP manual for advice on making your grammar LR(1).
```

为了解决这个冲突，我把 `FuncType` 这个类型直接删掉了，所有类型一律都用 `BType` 表示。

#### 2. 符号表

本节对符号表没有什么需要修改的地方（因为之前的设计已经够好了x）。

全局变量和全局常量都被存放在全局作用域的符号表中，其中全局变量关联到 `Value` （并且一定是 `GlobalAlloc` 类型的），全局常量关联到整数，都可以直接使用之前设计的数据结构。

#### 3. 生成全局内存分配指令

需要注意到，处理全局变量和常量与处理局部变量和常量存在以下的几个区别：

1. 全局变量对应的 `Value` 需要用  `program.new_value().global_alloc()` 这一套接口来在 Koopa IR 中生成
2. 在 SysY 中，全局变量也只能用常量初始化（这点与 C/C++ 是不一样的）。为此，需要为 `InitVal` 类型也实现 `Evaluate` 这个 Trait，并且在初始化全局变量的时候直接借此进行编译期间求值

注意到，生成全局/局部的变量/常量的语句都对应到 AST 中的 `Decl` 这一类型，所以要解决的问题就是，在生成 `Decl` 对应的 Koopa IR 时，怎么指定当前正在声明的东西是全局的还是局部的？需要根据这一信息来决定生成代码的方法。

作者觉得这个问题有两种解决方案：

1. 在 AST 中定义一个新的类型 `GlobalDecl` （并且定义 `GlobalConstDef` 、`GloablVarDef` 等配套的类型），让全局的声明全部关联到这些类型上
2. 在全局环境中新增一个变量，用于指示当前是否处在声明全局/局部符号的过程中

前一种实现方案会导致新增很多重复代码，所以作者选择了第二种方案。

具体而言，作者在 `IrgenEnv` 中新增了一个成员变量 `cur_decl_type: Option<DeclType>` ，其中 `DeclType` 这一类型的定义如下：

```rust
#[derive(Debug, Clone, Copy)]
pub enum DeclType {
    Global,
    Local,
}
```

如果正在处于声明全局符号的过程，就把 `cur_decl_type` 设置为 `Some(Global)` ；如果正在处于声明局部变量的过程，就把 `cur_decl_type` 设置为 `Some(Local)` ；其余情况均将其设置为 `None` 。

然后，在生成 `ConstDef` 和 `VarDef` 中，根据当前 `cur_decl_type` 的值，判断应该生成声明全局符号的 IR 还是生成声明局部符号的 IR 。

具体而言，对于 `ConstDef` 来说，如果当前 `cur_decl_type` 为 `Some(Global)` ，就把符号插入到全局的符号表中；如果为 `Some(Local)` ，就和之前一样插入到当前作用域的符号表中。对于 `VarDef` 来说，如果当前 `cur_decl_type` 为 `Some(Global)` ，就在编译器计算出初始值，使用 `global_alloc()` 接口生成对应的 IR，并把符号加入到全局的符号表中；如果是 `Some(Local)` ，那么就还是和之前的做法一样。

至此，Lv8 的前端就全部通过啦！
