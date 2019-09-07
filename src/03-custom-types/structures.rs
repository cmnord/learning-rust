#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Point { x: x1, y: y1 } = rect.p1;
    let Point { x: x2, y: y2 } = rect.p2;
    (y2 - y1).abs() * (x2 - x1).abs()
}

fn square(point: Point, side_len: f32) -> Rectangle {
    let Point { x: x1, y: y1 } = point;
    return Rectangle {
        p1: point,
        p2: Point {
            x: x1 + side_len,
            y: y1 + side_len,
        },
    };
}

fn main() {
    let name = "Claire";
    let age = 21;
    let claire = Person { name, age };

    println!("{:?}", claire);

    let point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // struct update
    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("area is {}", rect_area(_rectangle));

    println!("square is {:?}", square(new_point, 1.0));
}
