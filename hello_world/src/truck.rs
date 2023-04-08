use crate::{person::Person, vehicule::Vehicule};

pub struct Truck {
    conductor: Person,
    wheel_number: usize,
}

impl Truck {
    pub fn new(conductor: Person) -> Self {
        Truck { conductor: conductor, wheel_number: 6 }
    }

    pub fn display(&self) {
        let nb_wheel = self.wheel_number;
        println!("Camion qui a {nb_wheel} roues");
    }
}

impl Vehicule for Truck {
    fn display(&self) {
        let nb_wheel = self.wheel_number;
        println!("Camion qui a {nb_wheel} roues");
    }
}

#[test]
fn test_truck() {
    let mut v = Truck::new(Person { name: "test".to_string() });
    v.conductor.name.remove(0);
    Truck::display(&v);
    v.display();
    let e = v;

    let f: Box<dyn Fn(&Truck)>  = Box::new(Truck::display);
    f(&e);
}
