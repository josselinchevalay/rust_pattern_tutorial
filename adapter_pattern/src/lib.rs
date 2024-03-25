// Target (client) interface
trait Lion {
    fn roar(&self);
}

// Adaptee (incompatible interface)
struct Dog {
    name: String,
}

impl Dog {
    fn bark(&self) {
        println!("{} barks!", self.name);
    }
}

// Adapter
struct DogAdapter {
    dog: Dog,
}

impl Lion for DogAdapter {
    fn roar(&self) {
        self.dog.bark();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dog = Dog { name: "Jack".to_string() };
        let adapter = DogAdapter { dog }; 
    
        let lions: [DogAdapter; 1] = [adapter];
        lions[0].roar();
        assert_eq!(true, true);
    }
}
