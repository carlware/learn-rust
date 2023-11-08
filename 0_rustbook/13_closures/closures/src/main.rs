use std::arch::asm;
use std::thread;

fn main() {
    // same meaning
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    // only_borrows();
    // mutate();
    // with_thread();
    // sort();
    // iter();
    map();
}


fn only_borrows() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("only borrows {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn mutate(){
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn with_thread(){
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("from thread {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sort() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}", list);
}

fn iter() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn map() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|v| v + 1).collect();
    assert_eq!(v2, vec![2, 3, 4])
}
