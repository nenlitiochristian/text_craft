use crate::player::*;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub struct Inventory {
    pub ores: [Option<Ore>; 20],
    pub foods: [Option<Food>; 6],
}

#[derive(Clone, Copy)]
pub enum Ore {
    IronOre,
    GoldOre,
    Diamond,
}

#[derive(Clone, Copy)]
pub enum Food {
    Apple,
    Chicken,
    Beef,
}

impl Inventory {
    pub fn create_empty() -> Self {
        Inventory {
            ores: [None; 20],
            foods: [None; 6],
        }
    }

    pub fn push_food(&mut self, food: Food) {
        for i in 0..self.foods.len() {
            if self.foods[i].is_none() {
                self.foods[i] = Some(food);
                break;
            }
        }
    }

    pub fn push_ore(&mut self, ore: Ore) {
        for i in 0..self.ores.len() {
            if self.ores[i].is_none() {
                self.ores[i] = Some(ore);
                break;
            }
        }
    }

    pub fn print_ores(&self) {
        for (i, ores) in self.ores.iter().enumerate() {
            let string = match ores {
                Some(ore) => ore.to_string(),
                _ => "Empty",
            };
            println!("{}. {}", i + 1, string);
        }
    }

    pub fn print_food(&self) {
        for (i, food) in self.foods.iter().enumerate() {
            let string = match food {
                Some(food) => food.to_string(),
                _ => "Empty",
            };
            println!("{}. {}", i + 1, string);
        }
    }
}

fn parse_inventory_string(inventory_string: &str) -> Result<Inventory, &'static str> {
    let tokens: Vec<&str> = inventory_string.split(';').collect();
    let mut inventory = Inventory::create_empty();
    for token in tokens {
        if let Some(food) = Food::from_string(token) {
            inventory.push_food(food);
        } else if let Some(ore) = Ore::from_string(token) {
            inventory.push_ore(ore);
        }
    }

    Ok(inventory)
}

pub fn search_inventory_file(file_path: &str, username: &str) -> Result<Inventory, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line_content) = line {
            if !line_content.starts_with(username) {
                continue;
            }

            if let Ok(inventory) = parse_inventory_string(&line_content) {
                return Ok(inventory);
            }
        }
    }

    Ok(Inventory::create_empty())
}

impl Food {
    pub fn to_string(&self) -> &str {
        match self {
            Food::Apple => "Apple",
            Food::Chicken => "Chicken",
            Food::Beef => "Beef",
        }
    }

    fn from_string(string: &str) -> Option<Food> {
        match string {
            "Apple" => Some(Food::Apple),
            "Chicken" => Some(Food::Chicken),
            "Beef" => Some(Food::Beef),
            _ => None,
        }
    }
}

impl Ore {
    fn to_string(&self) -> &str {
        match self {
            Ore::IronOre => "Iron Ore",
            Ore::GoldOre => "Gold Ore",
            Ore::Diamond => "Diamond",
        }
    }

    fn from_string(string: &str) -> Option<Ore> {
        match string {
            "Iron Ore" => Some(Ore::IronOre),
            "Gold Ore" => Some(Ore::GoldOre),
            "Diamond" => Some(Ore::Diamond),
            _ => None,
        }
    }
}

pub fn update_inventory_file(players: &Vec<Player>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for player in players {
        let mut inventory_string = String::new();
        inventory_string.push_str(&player.account.username);
        inventory_string.push_str(";");

        for ore in player.inventory.ores {
            if let Some(ore) = ore {
                inventory_string.push_str(ore.to_string());
                inventory_string.push_str(";");
            }
        }

        inventory_string.push_str("\n");
        file.write_all(inventory_string.as_bytes())?;
    }

    Ok(())
}
