struct Bird {
    name: String,
    attack: u64,
}

impl Bird {
    fn print_details(&self) {
        println!("Name: {}", self.name);
        println!("Attack: {}", self.attack);
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        false
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
    fn is_animal(&self) -> bool {
        true
    }
}

pub fn structures() {
    let name: String = String::from("Pigeon");
    let bird = Bird {
        name: name,
        attack: 5,
    };
    bird.print_details();
    println!("Is Animal : {}", bird.is_animal());
    println!("Can Fly : {}", bird.can_fly());
}
