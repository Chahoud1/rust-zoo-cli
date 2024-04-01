use std::io::{self, Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Animal {
    id: u64,
    name: String,
    age: u64,
    vaccinated: bool
}

impl Animal {
    fn serialize(&self) {
        let json = serde_json::to_string(&self).expect("Failed to serialize the animal");

        println!("{}", json);
    }

    fn create_animal() -> Animal {
        
        let mut input_id: String = String::new();
        let mut input_name: String = String::new();
        let mut input_age: String = String::new();
        let mut input_vaccinated: String = String::new();
        let msg = "Failed to read the input";

        println!("Insert the animal ID:");
        io::stdin().read_line(&mut input_id).expect(msg);
        let input_id: u64 = input_id.trim().parse().expect("Invalid input format");

        println!("Insert the animal Name:");
        io::stdin().read_line(&mut input_name).expect(msg);
        
        println!("Insert the animal Age (Only Numbers):");
        io::stdin().read_line(&mut input_age).expect(msg);
        let input_age: u64 = input_age.trim().parse().expect(msg);
        
        println!("Is the Animal vaccinated? (Y/N):");
        io::stdin().read_line(&mut input_vaccinated).expect(msg);
        let input_vaccinated: bool = input_vaccinated.trim().to_uppercase() == "Y";

        Animal {
            id: input_id,
            name: input_name,
            age: input_age,
            vaccinated: input_vaccinated
        }
    }

    fn display(&self) {
        println!("
                You have registered the following animal:
                ID: {}
                Name: {}
                Age: {}
                Vaccinated: {}", 
                self.id, self.name, self.age, self.vaccinated);
    }
}

enum Local {
    NorthZone,
    SouthZone,
    EastZone,
    WestZone
}
impl Local {
    fn description(&self) -> &str {
        match self {
            Local::NorthZone => "Uma ala situada no Norte",
            Local::SouthZone => "Uma ala situada no Sul",
            Local::EastZone => "Uma ala situada no Leste",
            Local::WestZone => "Uma ala situada no Oeste",
        }
    }
}

fn menu() {

    loop {

        println!("Menu:\n1-View Animals\n2-Register a new Animal");

        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let input: u64 = input.trim().parse().expect("Failed");

        let a2: Animal;

        match input {
            1 => println!("sem implementação no momento"),
            2 => {
                a2 = Animal::create_animal();
                a2.serialize();
                a2.display();
            }
            _ => {}
        };
    }
}

fn main() {
    
    menu();

    let local1: Local = Local::NorthZone;
    println!("{}", local1.description());

    //dbg!(a1);
}