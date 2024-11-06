
# How to start
使用二进制 Crate - 在 src/bin 目录下创建多个文件
这是最常用的方法，我来展示如何修改项目结构：
首先，将现有的 main.rs 移动到一个新的文件中：
Bash
rs
然后可以创建更多的程序文件，例如：
```
fn main() {
    println!("Hello from another binary!");
}
```

```rust
fn main() {
    let x = 15;
    let y = 3;
    
    println!("加法: {} + {} = {}", x, y, x + y);
    println!("减法: {} - {} = {}", x, y, x - y);
    println!("乘法: {} * {} = {}", x, y, x * y);
    println!("除法: {} / {} = {}", x, y, x / y);
}
```

现在你可以通过以下方式运行不同的程序：
Bash
calculator
其他建议：
1. 如果这些程序之间需要共享代码，你可以：
在 src/lib.rs 中定义共享函数和模块
使用 mod 关键字在 src 目录下创建共享模块
每个二进制文件都需要有自己的 main() 函数作为入口点
在 Cargo.toml 中，这些二进制文件会自动被识别，不需要特别配置
这种组织方式可以让你在同一个项目中维护多个相关的程序，同时共享代码和依赖项。

