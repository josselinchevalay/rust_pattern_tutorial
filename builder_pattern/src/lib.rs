
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    city: String, 
}

struct PersonBuilder {
    first_name: String,
    last_name: String,
    age: u8,
    city: String, 
}

impl PersonBuilder {
    fn new() -> Self {
        PersonBuilder {
            first_name: String::new(),
            last_name: String::new(), 
            age: 0,
            city: String::new(),
        }
    }

    fn first_name(mut self, first_name: String) -> Self {
        self.first_name = first_name;
        self
    }

    fn last_name(mut self, last_name: String) -> Self {
        self.last_name = last_name;
        self 
    }

    fn age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    fn city(mut self, city: String) -> Self {
        self.city = city;
        self
    }

    fn build(self) -> Person {
        Person {
            first_name: self.first_name,
            last_name: self.last_name,
            age: self.age,
            city: self.city,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let john = PersonBuilder::new()
                    .first_name("John".to_string())
                    .last_name("Doe".to_string())
                    .age(30)
                    .city("New York".to_string())
                    .build();
        assert_eq!(john.first_name, "John".to_string());
    }
}
