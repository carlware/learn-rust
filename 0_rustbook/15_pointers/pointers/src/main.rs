use std::ops::Deref;
use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct Smart {
    data: String,
}

impl Drop for Smart {
    fn drop(&mut self) {
        println!("dropping value `{}`", self.data);
    }
}


fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let _data1 = Smart{
        data: String::from("hello data1"),
    };
    let _data2 = Smart{
        data: String::from("hello data2"),
    };

    // dropping early
    drop(_data1);
}
