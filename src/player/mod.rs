use crate::account::*;
use crate::inventory::*;
use rand::Rng;
use std::cmp::Ordering;

pub struct Player {
    pub account: Account,
    pub inventory: Inventory,
    health: u8,
    depth: u8,
}

impl Eq for Player {}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.account.username == other.account.username
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.account.money.partial_cmp(&other.account.money)
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.account.money.cmp(&other.account.money)
    }
}

impl Player {
    pub fn spend(&mut self, amount: u32) -> Result<(), &'static str> {
        if amount <= self.account.money {
            self.account.money -= amount;
            Ok(())
        } else {
            Err("Error: Not enough money!")
        }
    }

    pub fn take_damage(&mut self, damage: u8) {
        let (new_health, overflowed) = self.health.overflowing_sub(damage);

        if overflowed {
            self.health = 0;
        } else {
            self.health = new_health;
        }
    }

    pub fn heal(&mut self, heal: u8) {
        self.health += heal;

        if self.health > 100 {
            self.health = 100;
            return;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }

    pub fn new(inventory: Inventory, account: Account) -> Player {
        Player {
            account,
            inventory,
            health: 100,
            depth: 1,
        }
    }

    pub fn stop_mining(&mut self) {
        self.depth = 1;
    }

    pub fn get_depth(&self) -> u8 {
        self.depth
    }

    pub fn go_deeper(&mut self) {
        self.depth += 1;
    }

    pub fn mine_ore(&mut self) {
        let (iron_chance, gold_chance) = match self.account.pickaxe_level {
            3 => (60, 40),
            2 => (54, 36),
            1 | _ => (57, 28),
        };

        for _ in 1..3 {
            if rand::random() {
                self.inventory.push_ore(mine(iron_chance, gold_chance));
            }
        }
    }

    pub fn can_eat(&self, index: i32) -> bool {
        if !(index >= 1 && index <= 6) {
            return false;
        }
        let index: usize = index as usize;
        match self.inventory.foods[index - 1] {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn eat(&mut self, index: i32) {
        if !self.can_eat(index) {
            return;
        }

        let index: usize = (index - 1) as usize;
        let food = &self.inventory.foods[index];
        match food.unwrap() {
            Food::Apple => self.heal(10),
            Food::Chicken => self.heal(30),
            Food::Beef => self.heal(40),
        }
        self.inventory.foods[index] = None;
    }

    pub fn purge_inventory(&mut self) {
        for ore in &mut self.inventory.ores {
            *ore = None;
        }
    }

    pub fn is_pickaxe_maxed(&self) -> bool {
        self.account.pickaxe_level >= 3
    }

    pub fn upgrade_pickaxe_cost(&self) -> u32 {
        let pickaxe_level = self.account.pickaxe_level as u32;
        let base_cost = 100;
        let cost_per_level = 200;

        let total_cost = pickaxe_level * cost_per_level + base_cost;

        total_cost
    }

    pub fn upgrade_pickaxe(&mut self) {
        if self.account.pickaxe_level < 3 {
            self.account.pickaxe_level += 1;
        }
    }
}

fn mine(iron_chance: u8, gold_chance: u8) -> Ore {
    let mut rng = rand::thread_rng();
    let random_num: u8 = rng.gen_range(1..=100);

    if random_num < iron_chance {
        return Ore::IronOre;
    } else if random_num < iron_chance + gold_chance {
        return Ore::GoldOre;
    } else {
        return Ore::Diamond;
    }
}
