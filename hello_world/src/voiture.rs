use crate::{person::Person, vehicule::Vehicule};

pub struct Voiture {
    conductor: Person,
    wheel_number: usize,
}

impl Voiture {
    pub fn new(conductor: Person) -> Self {
        Voiture { conductor: conductor, wheel_number: 4 }
    }

    pub fn afficher(&self) {
        let nb_wheel = self.wheel_number;
        println!("voiture qui a {nb_wheel} roues");
    }
}

impl Vehicule for Voiture {
    fn display(&self) {
        self.afficher();
    }
}

#[test]
fn test_voiture() {
    let mut v = Voiture::new(Person { name: "test".to_string() });
    v.conductor.name.remove(0);
    Voiture::display(&v);
    v.display();
    let e = v;

    let f: Box<dyn Fn(&Voiture)>  = Box::new(Voiture::display);
    f(&e);
}
