use rand::Rng;
use std::io::{self, Write};

mod account;
use account::{parse_account_file, update_account_file, Account};

mod inventory;
use inventory::{search_inventory_file, Food, Inventory};

mod player;
use player::*;

use crate::inventory::update_inventory_file;

fn main() -> Result<(), io::Error> {
    let account_file_path = "src/account.txt";
    let inventory_file_path = "src/inventory.txt";
    let accounts = parse_account_file(account_file_path)?;
    let mut players: Vec<Player> = Vec::new();
    for account in accounts {
        players.push(Player::new(
            search_inventory_file(inventory_file_path, &account.username)
                .unwrap_or(Inventory::create_empty()),
            account,
        ));
    }

    loop {
        println!("Welcome to Textcraft!");
        println!("1. Continue");
        println!("2. New Game");
        println!("3. Exit");
        print!(">> ");
        unsafe_stdout_flush();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<i32>().unwrap_or(-1) {
            1 => login_menu(&mut players),
            2 => register_menu(&mut players),
            3 => return Ok(()),
            _ => (),
        }

        let mut accounts: Vec<Account> = Vec::new();
        for player in &players {
            accounts.push(player.account.clone());
        }
        update_account_file(&accounts, account_file_path)?;
        update_inventory_file(&players, inventory_file_path)?;
    }
}

// only for windows and linux
// fn clear_screen() {
//     assert!(std::process::Command::new("cls")
//         .status()
//         .or_else(|_| std::process::Command::new("clear").status())
//         .unwrap()
//         .success());
// }

fn is_alphanumeric_string(input: &str) -> bool {
    input.chars().all(|c| c.is_alphanumeric())
}

fn unsafe_stdout_flush() {
    io::stdout().flush().unwrap();
}

fn register_menu(players: &mut Vec<Player>) {
    println!("Creating a new account:");
    let mut input = String::new();
    loop {
        print!("Enter your username (Must be alphanumeric): ");
        unsafe_stdout_flush();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input = input.trim().to_string();
        if is_alphanumeric_string(&input) {
            break;
        }
    }

    let player = Player::new(Inventory::create_empty(), Account::new_account(input));
    println!(
        "Made an account with username: {}",
        &player.account.username
    );

    players.push(player);
    wait_for_enter();
}

fn wait_for_enter() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn login_menu(players: &mut Vec<Player>) {
    if players.len() == 0 {
        println!("No account found!");
        wait_for_enter();
        return;
    }

    loop {
        println!("Choose an account! (0 to return)");
        let mut index = 1;
        players.sort_by(|a, b| b.cmp(a));
        for player in players.iter() {
            println!(
                "{}. {}, Money: {}",
                index, player.account.username, player.account.money
            );
            index += 1;
        }

        print!(">> ");
        unsafe_stdout_flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = input.trim().parse().unwrap_or(-1);
        if input == 0 {
            return;
        }

        if input < 0 {
            continue;
        }
        let index: usize = (input - 1).try_into().unwrap();
        if index >= players.len() {
            continue;
        }
        play_game(&mut players[index]);
    }
}

fn play_game(player: &mut Player) {
    loop {
        println!("Welcome, {}!", player.account.username);
        println!("1. Go mining\n2. Go shopping\n3. Back");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<i32>().unwrap_or(-1) {
            1 => go_mining(player),
            2 => go_shopping(player),
            3 => return,
            _ => (),
        }
    }
}

fn go_shopping(player: &mut Player) {
    loop {
        println!("Welcome to the shop!");
        println!("Money: {}", player.account.money);
        println!("1. Sell ores\n2. Buy items\n3. Back");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<i32>().unwrap_or(-1) {
            1 => sell_ores_menu(player),
            2 => buy_item_menu(player),
            3 => return,
            _ => (),
        }
    }
}

fn buy_item_menu(player: &mut Player) {
    loop {
        println!("Your food bag:");
        player.inventory.print_food();
        println!("Your money: {}", player.account.money);
        println!("=====================");
        println!("1. Buy Apple - $30\n2. Buy Chicken - $70\n3. Buy Beef - $90");
        if !player.is_pickaxe_maxed() {
            println!("4. Upgrade Pickaxe - ${}", player.upgrade_pickaxe_cost());
            print!("5. Return\n>> ");
        } else {
            print!("4. Return\n>> ");
        }
        unsafe_stdout_flush();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice = choice.trim().parse::<i32>().unwrap_or(-1);
        if choice == 4 && !player.is_pickaxe_maxed() {
            match player.spend(player.upgrade_pickaxe_cost()) {
                Ok(_) => {
                    player.upgrade_pickaxe();
                    println!("Upgraded pickaxe level!");
                    wait_for_enter();
                    continue;
                }
                Err(msg) => {
                    println!("{}", msg);
                    wait_for_enter();
                    continue;
                }
            };
        }

        let (price, food) = match choice {
            1 => (30, Food::Apple),
            2 => (70, Food::Chicken),
            3 => (90, Food::Beef),
            4 | 5 => return,
            _ => continue,
        };

        if !player.inventory.foods.iter().any(|&x| x.is_none()) {
            println!("You have no free space!");
            continue;
        }

        if let Err(msg) = player.spend(price) {
            println!("{}", msg);
            wait_for_enter();
            continue;
        }

        println!("Buying {} at {}$", food.to_string(), price);
        player.inventory.push_food(food);
    }
}

fn sell_ores_menu(player: &mut Player) {
    loop {
        // count amount of each ore
        let mut iron_count = 0;
        let mut gold_count = 0;
        let mut diamond_count = 0;
        for ore in player.inventory.ores {
            if let Some(ore) = ore {
                match ore {
                    inventory::Ore::IronOre => iron_count += 1,
                    inventory::Ore::Diamond => diamond_count += 1,
                    inventory::Ore::GoldOre => gold_count += 1,
                }
            }
        }
        println!("Your money: {}", player.account.money);
        println!("Your ores:");
        println!("- Iron ores: {} @ 20$ per piece", iron_count);
        println!("- Gold ores: {} @ 50$ per piece", gold_count);
        println!("- Diamonds: {} @ 120$ per piece", diamond_count);
        println!("1. Sell all\n2. Back");
        print!(">> ");
        unsafe_stdout_flush();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice = choice.trim().parse::<i32>().unwrap_or(-1);
        if choice == 2 {
            return;
        }
        if choice != 1 {
            return;
        }

        let total_added_money = iron_count * 20 + gold_count * 50 + diamond_count * 120;
        player.account.money += total_added_money;
        player.purge_inventory();
    }
}

fn go_mining(player: &mut Player) {
    player.stop_mining();
    loop {
        println!("Your inventory:");
        player.inventory.print_ores();
        println!("You're on depth: {}", player.get_depth());
        println!("Health: {}", player.get_health());
        println!("What to do?");
        print!("1. Go Deeper\n2. Eat Food\n3. Return\n>> ");
        unsafe_stdout_flush();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<i32>().unwrap_or(-1) {
            1 => {
                if player.is_alive() {
                    dig(player);
                } else {
                    println!("You don't have enough health!");
                }
            }
            2 => eat_food(player),
            3 => return,
            _ => (),
        }
    }
}

fn try_starting_event(chance: u8) -> bool {
    let mut rng = rand::thread_rng();
    let random_num: u8 = rng.gen_range(1..=100);
    random_num <= chance
}

fn dig(player: &mut Player) {
    // cause a random event
    // Successfully dig
    if try_starting_event(40) {
        println!("You successfully dug deeper!");
        player.go_deeper();
    }
    // get hungry
    else if try_starting_event(20) {
        player.take_damage(10);
    }
    // creeper explode
    else if try_starting_event(10) {
        player.take_damage(30);
    }

    // mine ores
    player.mine_ore();
}

fn eat_food(player: &mut Player) {
    loop {
        println!("Your food sack: ");
        player.inventory.print_food();
        print!("Enter the index of the food you want to eat (0 to cancel): ");
        unsafe_stdout_flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().parse::<i32>().unwrap_or(-1);
        if input == 0 {
            wait_for_enter();
            return;
        }

        if player.can_eat(input) {
            player.eat(input);
            println!("You regenerated some health!");
        } else {
            println!("Please choose a valid food!");
        }
        wait_for_enter();
    }
}
