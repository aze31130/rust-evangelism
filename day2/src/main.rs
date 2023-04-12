pub mod vroumvroum;

pub struct Car {
    name: String,
    speed: i32
}

pub struct Truck {
    name: String,
    speed: i32
}

trait Vehicle {
    fn name(&self) -> String;
    fn speed(&self) -> i32;
    fn drive(&self) {
        let name = self.name();
        let speed = self.speed();
        println!("Driving a {name} at {speed}");
    }
}

impl Vehicle for Car {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn speed(&self) -> i32 {
        self.speed
    }
}
impl Vehicle for Truck {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn speed(&self) -> i32 {
        self.speed
    }
}

impl<T> Vehicle for Vec<T> {

    fn name(&self) -> String {
        format!("anonymous")
    }

    fn speed(&self) -> i32 {
        self.len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}



trait Emojiable {
    fn emoji(&self) -> String;
    fn to_string(&self) -> String;
}

impl Emojiable for String {
    fn emoji(&self) -> String {
        match self.as_str() {
            "happy" => "😀".into(),
            "sad" => "😢".into(),
            _ => "😐".into(),
        }
    }

    fn to_string(&self) -> String {
        self.clone()
    }
}

struct Inner {}

impl Inner {
    fn machin(&self) {
        // fais un truc
    }
}

struct Outer {
    inner: Inner,
}

impl Outer {
    fn machin(&self) {
        // fais un autre truc
        self.inner.machin(); // possible
    }
}

#[test]
fn test_emoji() {
    assert_eq!(&"happy".to_string().emoji(), "😀");
    assert_eq!(&"sad".to_string().emoji(), "😢");
    assert_eq!(&"neutral".to_string().emoji(), "😐");
}
