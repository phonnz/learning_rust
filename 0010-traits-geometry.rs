trait Area {
    fn area(&self) -> i32;
}

trait Perimeter{
    fn perimeter(&self) -> i32;
}

struct Rectangle{
    width: i32,
    height: i32
}

struct Square{
    width: i32,
    height: i32
}

struct Triangle{
    width: i32,
    height: i32
}

impl Area for Rectangle{
    fn area(&self) -> i32{
         self.width * self.height
    }
}

impl Area for Square{
    fn area(&self) -> i32{
        self.width * self.height
    }
}

impl Area for Triangle{
    fn area(&self) -> i32{
        (self.width * self.height) / 2
    }
}

impl Perimeter for Square{
    fn perimeter(&self) -> i32{
        self.width * 4
    }
}

impl Perimeter for Rectangle{
    fn perimeter(&self) -> i32{
       (self.width * 2) + (self.height * 2)
    }
}

fn main(){
    let square = Square{ width: 2, height: 2};
    let rectangle = Rectangle{width: 4, height:2};
    let triangle = Triangle{width: 2, height: 3};

    println!("Square {:?} x {:?} ", square.width, square.height);
    println!("Area: {:?}", square.area());
    println!("Perimeter {:?}", square.perimeter());
    println!("Rectangle {:?} x {:?}", rectangle.width, rectangle.height);
    println!("Area: {:?}", rectangle.area());
    println!("Perimeter {:?}", rectangle.perimeter());
    println!("Triangle {:?} x {:?}", triangle.width, triangle.height);
    println!("Area : {:?}", triangle.area());
}
