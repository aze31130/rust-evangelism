use crate::{person::Person, truck::Truck, voiture::Voiture};

pub trait Vehicule {
    fn display(&self);
}

#[test]
fn test_vehicule() {
    let t = Truck::new(Person { name: "Bob".to_string() });
    let v = Voiture::new(Person { name: "Jeff".to_string() });
    
    
    // Vehicule::display(&v);
    // v.display();
    
    let mut arr: Vec<Box<dyn Vehicule>> = Vec::new();
    arr.push(Box::new(t));
    arr.push(Box::new(v));

    for vehicule in arr {
        vehicule.display();
    }

}

// fn montre_le_vehicule<T: Vehicule>(v: &T) {
//     v.display();
// }

// fn montre_le_vehicule(v: &impl Vehicule) {
//     v.display();
// }

/*

struct DynVehicule {
    ptr: T;
    display: Fn(T);
    compter_les_roues: Fn(T) -> i32;
}

*/
