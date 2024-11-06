// 如果你想要让这些变量成为可以多次赋值的，就要设置称为mut变量
fn main() {
    let mut x = 5; // mutable variable
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
}
