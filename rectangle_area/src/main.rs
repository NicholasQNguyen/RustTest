struct Rectangle {
        width: u32,
        length: u32
    }

fn calculate_area(rect: Rectangle) -> u32 {
    rect.width * rect.length
    }

fn main() {
    let rect1 = Rectangle {
        width: 10,
        length: 50
    };

    println!("Width {}", rect1.width);
    println!("Length {}", rect1.length);
    println!("Area: {}", calculate_area(rect1));
}


