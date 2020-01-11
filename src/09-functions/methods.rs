struct Point {
    x: f64,
    y: f64,
}

// Implementation block. All `Point` methods go here.
impl Point {
    // static method. Doesn't need to be called by an instance.
    // static methods are generally used as constructors.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // instance method. `&self` is sugar for `self: &Self` where `Self` is the type of
    // the caller object. In this case `Self` = `Rectangle`.
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs()` is a method on `f64`
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // Requires the caller object to be mutable
    // `&mut self` is sugar for `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

// Pair owns two heap-allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object.
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // destructure `self`
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // first and second go out of scope here and get freed
    }
}

fn main() {
    let rect = Rectangle {
        // static methods are called using ::
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // instance methods are called using . (dot operator)
    // note that first argument &self is implicitly passed:
    //     `rect.perimeter()` == `Rectangle::perimeter(&rect)`
    println!("Rectangle perimeter: {}", rect.perimeter());
    println!("Rectangle area: {}", rect.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // error! rect was declared as immutable.
    // rect.translate(1.0, 0.0);

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // error! previous `destroy` call consumed `pair`
    // pair.destroy();
}
