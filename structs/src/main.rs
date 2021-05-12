#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;

    (y1 - y2) * (x2 - x1)
}

fn square(point: &Point, ratio: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            y: point.y - ratio,
            ..*point
        },
        bottom_right: Point {
            x: point.x + ratio,
            y: point.y - ratio,
        },
    }
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 2.3, y: 5.3 },
        bottom_right: Point { x: 4.0, y: 2.7 },
    };

    println!("Rect area is {:.2}", rect_area(&rect));
    println!(
        "Square rect is {:?}",
        square(&Point { x: 2.0, y: 5.5 }, 2.0)
    );
}
