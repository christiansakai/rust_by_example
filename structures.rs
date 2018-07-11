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

fn square(point: Point, f: f32) -> Rectangle {
    let x = point.x;
    let y = point.y;

    Rectangle {
        p1: Point {
            x: x,
            y: y,
        },
        p2: Point {
            x: x * f,
            y: y * f,
        },
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Point { x: x1, y: y1 } = rectangle.p1;
    let Point { x: x2, y: y2 } = rectangle.p2;

    ((x1 - x2).abs() * (y1 - y2).abs())
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point {
        x: 0.3,
        y: 0.4,
    };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let Point { x: my_x, y: my_y } = point;

    let rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rectangle area {}", rect_area(rectangle));

    let point2 = Point {
        x: 0.5,
        y: 0.6,
    };

    println!("square {:?}", square(point2, 3.0));
}

