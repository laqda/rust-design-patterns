#[allow(dead_code)]
pub fn test() {
    let b1 = BurgerBuilder::new()
        .with_bread(Breads::Italian)
        .with_cheese(Cheeses::Cheddar)
        .with_sauces(Sauces::Ketchup)
        .with_salad()
        .with_tomato()
        .build();
    println!("First burger {:#?}", b1);
    let b2 = BurgerBuilder::new()
        .with_bread(Breads::Normal)
        .with_cheese(Cheeses::Emmental)
        .with_sauces(Sauces::Mayonnaise)
        .build();
    println!("Second burger {:#?}", b2);
}

#[derive(Debug)]
enum Breads {
    Normal,
    Italian,
}

#[derive(Debug)]
enum Cheeses {
    Emmental,
    Cheddar,
}

#[derive(Debug)]
enum Sauces {
    Ketchup,
    Mayonnaise,
}

// Burger

#[derive(Debug)]
struct Burger {
    bread: Option<Breads>,
    cheese: Option<Cheeses>,
    sauces: Option<Sauces>,
    salad: bool,
    tomato: bool,
}

impl Burger {
    fn new() -> Self {
        Burger {
            bread: None,
            cheese: None,
            sauces: None,
            salad: false,
            tomato: false,
        }
    }
}

// BurgerBuilder

struct BurgerBuilder {
    burger: Burger,
}

impl BurgerBuilder {
    fn new() -> Self {
        BurgerBuilder { burger: Burger::new() }
    }

    fn with_bread(mut self, b: Breads) -> Self {
        self.burger.bread = Some(b);
        self
    }

    fn with_cheese(mut self, c: Cheeses) -> Self {
        self.burger.cheese = Some(c);
        self
    }

    fn with_sauces(mut self, s: Sauces) -> Self {
        self.burger.sauces = Some(s);
        self
    }

    fn with_salad(mut self) -> Self {
        self.burger.salad = true;
        self
    }

    fn with_tomato(mut self) -> Self {
        self.burger.tomato = true;
        self
    }

    fn build(self) -> Burger {
        self.burger
    }
}
