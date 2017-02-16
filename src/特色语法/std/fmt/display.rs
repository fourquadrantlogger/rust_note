use std::fmt;
// 从 `fmt::Debug` 获得实现给 `Structure`。
// `Structure` 是一个包含`i32`基本类型的结构体。
#[derive(Debug)]
struct Structure(i32);
// 为了使用 `{}` 标记，必须手动实现 `fmt::Display` trait 来支持相应类型。
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    // 这个 trait 要求 `fmt` 带有正确的标记
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 严格将第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此结果表明操作成功
        // 或失败。注意这里的 `write!` 用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}
// 将 `Structure` 放到结构体 `Deep` 中。使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // 打印操作使用 `{:?}` 和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` 是能够打印的类型。
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7`？
    println!("Now {:?} will print!", Deep(Structure(7)));
}