use std::fs::File;
use std::io::{self, BufRead, Write};

#[derive(Clone)]
pub struct Account {
    pub username: String,
    pub money: u32,
    pub pickaxe_level: u8,
}

impl Account {
    pub fn new_account(username: String) -> Account {
        Account {
            username,
            money: 100,
            pickaxe_level: 1,
        }
    }
}

pub fn parse_account_string(account_string: &str) -> Result<Account, &'static str> {
    let tokens: Vec<&str> = account_string.split(',').collect();
    if tokens.len() != 3 {
        return Err("Invalid number of tokens");
    }

    let username = tokens[0].to_string();
    let money = tokens[1].trim().parse().unwrap_or(0);
    let pickaxe_level = tokens[2].trim().parse().unwrap_or(1);

    Ok(Account {
        username,
        money,
        pickaxe_level,
    })
}

pub fn parse_account_file(file_path: &str) -> Result<Vec<Account>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut accounts = Vec::new();

    for line in reader.lines() {
        if let Ok(line_content) = line {
            if let Ok(account) = parse_account_string(&line_content) {
                accounts.push(account);
            }
        }
    }

    Ok(accounts)
}

pub fn update_account_file(accounts: &Vec<Account>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for account in accounts {
        let account_string = format!(
            "{},{},{}\n",
            account.username, account.money, account.pickaxe_level
        );

        file.write_all(account_string.as_bytes())?;
    }

    Ok(())
}
