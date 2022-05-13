//! deref/as_ref/as_mut

// describes all the behavior of a type:
// - area: calculates the area of a shape
// - zoom: used for mutating the type's fields
trait Shape {
    fn area(&self) -> f64;

    fn zoom(&mut self, factor: f64);
}

// concrete type #1
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn zoom(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

// impl AsMut<dyn Shape>
// lifetime parameter 'a is required for `dyn Shape`
impl<'a> AsMut<dyn Shape + 'a> for Rectangle {
    fn as_mut(&mut self) -> &mut (dyn Shape + 'a) {
        self
    }
}

// concrete type #2
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }

    fn zoom(&mut self, factor: f64) {
        self.base *= factor;
        self.height *= factor;
    }
}

impl<'a> AsMut<dyn Shape + 'a> for Triangle {
    fn as_mut(&mut self) -> &mut (dyn Shape + 'a) {
        self
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn zoom(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

impl<'a> AsMut<dyn Shape + 'a> for Circle {
    fn as_mut(&mut self) -> &mut (dyn Shape + 'a) {
        self
    }
}

// zoom_1 is more flexible than zoom_2
#[allow(dead_code)]
fn zoom_1<T: AsMut<dyn Shape>>(shape: &mut T, factor: f64) {
    shape.as_mut().zoom(factor);
}

// zoom_2 only accepts `Box<dyn Shape>`
#[allow(dead_code)]
fn zoom_2(shape: &mut Box<dyn Shape>, factor: f64) {
    shape.zoom(factor);
}

#[test]
fn test_zoom_1() {
    let mut rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    zoom_1(&mut rectangle, 2.0);
}

#[test]
fn test_zoom_2() {
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    let mut rectangle = Box::new(rectangle) as Box<dyn Shape>;

    zoom_2(&mut rectangle, 2.0);
}

#[test]
fn test_zoom_vec_shape() {
    let mut vec: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        }),
        Box::new(Triangle {
            base: 10.0,
            height: 20.0,
        }),
        Box::new(Circle { radius: 10.0 }),
    ];

    // here we can use both zoom_1 and zoom_2
    vec.iter_mut().for_each(|shape| {
        zoom_1(shape, 2.0);
        zoom_2(shape, 2.0);
    });

    for item in vec.iter() {
        let area = item.area();

        println!("{:?}", area);
    }
}

#[test]
fn test_zoom_vec_rectangle() {
    let mut vec = vec![
        Rectangle {
            width: 10.0,
            height: 20.0,
        },
        Rectangle {
            width: 12.0,
            height: 18.0,
        },
        Rectangle {
            width: 14.0,
            height: 16.0,
        },
    ];

    // zoom_2 is no longer applicable
    vec.iter_mut().for_each(|shape| {
        zoom_1(shape, 2.0);
        // no more allowed here
        // zoom_2(shape, 2.0);
    });

    for item in vec.iter() {
        let area = item.area();

        println!("{:?}", area);
    }
}
