fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    return largest
}

fn largest_ref<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    return largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);

    let x: f32 = 90.0;
    println!("{}", x.powi(2));

    let p = Point{x: 10.0, y: 12.0};
    println!("p.x = {} dis = {}", p.x(), p.distance_from_origin());
}
