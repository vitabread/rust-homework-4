// question 2

use std::ops::Add;

struct MyType {
    value: i32,
}

impl Add for MyType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        MyType {
            value: self.value + other.value,
        }
    }
}

trait MyTrait {
    fn method(&self) -> i32;
}

impl MyTrait for MyType {
    fn method(&self) -> i32 {
        self.value
    }
}

fn call_method(obj: &dyn MyTrait) {
    println!("{}", obj.method());
}

fn main() {
    let a = MyType { value: 5 };
    let b = MyType { value: 3 };
    let c = a + b;
    call_method(&c);
}
