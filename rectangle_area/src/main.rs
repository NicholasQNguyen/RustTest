struct Rectangle {
        width: u32,
        length: u32
    }

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.length
    }

    fn perimeter(&self) -> u32 {
        self.width * 2 + self.length *2 
    }
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
    }

fn main() {
    let rect1 = Rectangle {
        width: 10,
        length: 50
    };

    println!("Width {}", rect1.width);
    println!("Length {}", rect1.length);
    println!("Area: {}", calculate_area(&rect1));
    println!("Area: {}", rect1.area());
    println!("Perimeter: {}", rect1.perimeter());
}
