use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

struct CoffeeTransaction {
    id: usize,
    coffee_type: String,
    timestamp: u64,
    proof_of_coffee_hash: String,
}

struct CoffeeLedger {
    transactions: VecDeque<CoffeeTransaction>,
    current_id: usize,
}

impl CoffeeLedger {
    fn new() -> Self {
        CoffeeLedger {
            transactions: VecDeque::new(),
            current_id: 0,
        }
    }

    fn mine_coffee(&mut self, coffee_type: &str) {
        self.current_id += 1;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let fake_hash = generate_fake_hash();
        
        let transaction = CoffeeTransaction {
            id: self.current_id,
            coffee_type: coffee_type.to_string(),
            timestamp,
            proof_of_coffee_hash: fake_hash,
        };

        println!(
            "\nBrewing your coffee... ☕\nTransaction ID: {}\nType: {}\nTimestamp: {}\nHash: {}\n",
            transaction.id, transaction.coffee_type, transaction.timestamp, transaction.proof_of_coffee_hash
        );

        self.transactions.push_back(transaction);
    }

    fn show_ledger(&self) {
        println!("\n=== Coffee Ledger ===");
        for transaction in &self.transactions {
            println!(
                "ID: {}, Type: {}, Timestamp: {}, Hash: {}",
                transaction.id, transaction.coffee_type, transaction.timestamp, transaction.proof_of_coffee_hash
            );
        }
        println!("======================");
    }
}

fn generate_fake_hash() -> String {
    let mut rng = rand::thread_rng();
    let random_hash: String = (0..10)
        .map(|_| rng.gen_range(0..16))
        .map(|v| format!("{:x}", v))
        .collect();
    format!("proof-of-coffee-{}", random_hash)
}

fn main() {
    let mut ledger = CoffeeLedger::new();
    println!("Welcome to the Proof-of-Coffee Simulator!");

    loop {
        println!("\n1. Drink Coffee\n2. Show Coffee Ledger\n3. Exit");
        let mut choice = String::new();

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter your coffee type (e.g., Espresso, Latte):");
                let mut coffee_type = String::new();
                std::io::stdin()
                    .read_line(&mut coffee_type)
                    .expect("Failed to read line");

                ledger.mine_coffee(coffee_type.trim());
                println!("Successfully logged your coffee transaction!");
            }
            "2" => ledger.show_ledger(),
            "3" => {
                println!("Exiting Proof-of-Coffee Simulator. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
