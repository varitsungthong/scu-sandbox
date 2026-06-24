#[derive(Debug)]
struct Rectangle {
    width :u32,
    height :u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("The area of rectangle is {rect1:#?} square pixels.");
    dbg!(&rect1)
}