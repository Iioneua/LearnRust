#![deny(clippy::all)]

struct Food {
    id: u32,
    name: String,
    price: u32,
    is_on_promotion: bool,
}

impl Food {
    fn display(&self) {
        println!("{}", self.name);
        println!("{}", self.price);
        println!("{}", self.is_on_promotion);
        println!("{}", self.id);
    }
}

fn main() {
    let pizza = Food {
        id: 1,
        name: String::from("pizza"),
        price: 10,
        is_on_promotion: true,
    };
    let tacos = Food {
        id: 2,
        name: String::from("tacos"),
        ..pizza
    };
    pizza.display();
    tacos.display();
}
