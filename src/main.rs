use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const FILE_PATH: &str = "animals.json";

#[derive(Debug, Serialize, Deserialize)]
struct Animal {
    id: u64,
    name: String,
    age: u64,
    vaccinated: bool,
}

impl Animal {
    fn read_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    fn read_u64(prompt: &str) -> u64 {
        loop {
            let input = Self::read_input(prompt);
            if let Ok(num) = input.parse::<u64>() {
                return num;
            }
            println!("Invalid input, please enter a number.");
        }
    }

    fn read_bool(prompt: &str) -> bool {
        let input = Self::read_input(prompt);
        input.to_uppercase() == "Y"
    }

    fn create_animal() -> Animal {
        let id = Self::read_u64("Insert the animal ID: ");
        let name = Self::read_input("Insert the animal Name: ");
        let age = Self::read_u64("Insert the animal Age (Only Numbers): ");
        let vaccinated = Self::read_bool("Is the Animal vaccinated? (Y/N): ");
        Animal { id, name, age, vaccinated }
    }

    fn list_animals(animals: &[Animal]) {
        for animal in animals {
            println!("ID: {}\nName: {}\nAge: {}\nVaccinated: {}", animal.id, animal.name, animal.age, animal.vaccinated);
        }
    }

    fn read_animals() -> Vec<Animal> {
        if Path::new(FILE_PATH).exists() {
            let content = fs::read_to_string(FILE_PATH).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            vec![]
        }
    }

    fn write_animals(animals: &[Animal]) {
        let json = serde_json::to_string(animals).expect("Failed to serialize the animal list");
        fs::write(FILE_PATH, json).expect("Failed to write to file");
    }

    fn find_animal_index(animals: &[Animal], id: u64) -> Option<usize> {
        animals.iter().position(|animal| animal.id == id)
    }

    fn update_or_delete_animal(animals: &mut Vec<Animal>, id: u64, updated_animal: Option<Animal>) {
        if let Some(index) = Self::find_animal_index(animals, id) {
            match updated_animal {
                Some(animal) => animals[index] = animal,
                None => { animals.remove(index); },
            }
            Self::write_animals(animals);
        } else {
            println!("Animal not found");
        }
    }
}

fn main() {
    let mut running = true;
    while running {
        println!("Menu:\n1-Register a new Animal\n2-List Animals\n3-Update an Animal\n4-Delete Animal\n5-Exit");
        match Animal::read_u64("Choose an option: ") {
            1 => {
                let animal = Animal::create_animal();
                let mut animals = Animal::read_animals();
                animals.push(animal);
                Animal::write_animals(&animals);
            },
            2 => {
                let animals = Animal::read_animals();
                Animal::list_animals(&animals);
            },
            3 => {
                let id = Animal::read_u64("Enter the ID of the animal to update: ");
                let updated_animal = Animal::create_animal();
                let mut animals = Animal::read_animals();
                Animal::update_or_delete_animal(&mut animals, id, Some(updated_animal));
            },
            4 => {
                let id = Animal::read_u64("Enter the ID of the animal to delete: ");
                let mut animals = Animal::read_animals();
                Animal::update_or_delete_animal(&mut animals, id, None);
            },
            5 => running = false,
            _ => println!("Invalid option, please try again."),
        }
    }
}
