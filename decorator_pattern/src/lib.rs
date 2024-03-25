trait Pizza {
    fn get_cost(&self) -> f64;
}

struct SimplePizza {
    cost: f64
}

impl Pizza for SimplePizza {
    fn get_cost(&self) -> f64 {
        self.cost
    }
}

struct ToppingDecorator<'a> {
    pizza: &'a dyn Pizza,
    topping_cost: f64
}

impl<'a> Pizza for ToppingDecorator<'a> {
    fn get_cost(&self) -> f64 {
        self.pizza.get_cost() + self.topping_cost
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pizza = SimplePizza { cost: 10.0 };
        let pizza = ToppingDecorator { 
            pizza: &pizza, 
            topping_cost: 2.0 
        };
        assert_eq!(pizza.get_cost(), 12.0);
    }
}
