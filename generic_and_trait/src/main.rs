fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let ref mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            *largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

struct Point1<T> {
    x: T,
    y: T,
}

impl<T, U> Point<T, U> {
    // 能在该结构体的方法中定义额外的泛型参数
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let res = add(6, 10);
    println!("{}", res);
    let arr: [i32; 5] = [2, 4, 6, 8, 10];
    let res = largest(&arr);
    println!("{}", res);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    
    let p3 = p1.mixup(p2);
    
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let pt1 = Point1 { x: 3.14, y: 1.14 };
    let res = pt1.distance_from_origin();
    println!("distance from origin: {}", res);
}