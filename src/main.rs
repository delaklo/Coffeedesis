use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use colored::Colorize;


#[derive(Clone, Debug, PartialEq)]
enum CoffeeRarity {
    Common,     
    Uncommon,   
    Rare,       
    Legendary, 
}


struct Achievement {
    id: String,
    name: String,
    description: String,
    unlocked: bool,
    reward: u32,
}

struct CoffeeTransaction {
    id: usize,
    coffee_type: String,
    timestamp: u64,
    proof_of_coffee_hash: String,
    rarity: CoffeeRarity,
    brewer: String,
}

struct CoffeeLedger {
    transactions: VecDeque<CoffeeTransaction>,
    current_id: usize,
    achievements: HashMap<String, Achievement>,
    user_balance: HashMap<String, u32>,
    current_user: String,
}

impl CoffeeLedger {
    fn new() -> Self {
       
        let mut achievements = HashMap::new();
        achievements.insert("first_brew".to_string(), Achievement {
            id: "first_brew".to_string(),
            name: "First Cup".to_string(),
            description: "Brew your first coffee on the chain".to_string(),
            unlocked: false,
            reward: 50,
        });
        achievements.insert("rare_find".to_string(), Achievement {
            id: "rare_find".to_string(),
            name: "Rare Find".to_string(),
            description: "Discover a rare coffee blend".to_string(),
            unlocked: false,
            reward: 100,
        });
        achievements.insert("legendary_barista".to_string(), Achievement {
            id: "legendary_barista".to_string(),
            name: "Legendary Barista".to_string(),
            description: "Brew 10 different types of coffee".to_string(),
            unlocked: false,
            reward: 500,
        });

        CoffeeLedger {
            transactions: VecDeque::new(),
            current_id: 0,
            achievements,
            user_balance: HashMap::new(),
            current_user: "CoffeeChain".to_string(),
        }
    }

    fn mine_coffee(&mut self, coffee_type: &str) {
        self.current_id += 1;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let fake_hash = generate_fake_hash();
        
     
        let rarity = match rand::thread_rng().gen_range(0..100) {
            0..=1 => CoffeeRarity::Legendary,  
            2..=14 => CoffeeRarity::Rare,      
            15..=39 => CoffeeRarity::Uncommon,
            _ => CoffeeRarity::Common,         
        };
        
        let transaction = CoffeeTransaction {
            id: self.current_id,
            coffee_type: coffee_type.to_string(),
            timestamp,
            proof_of_coffee_hash: fake_hash,
            rarity: rarity.clone(),
            brewer: self.current_user.clone(),
        };
        
     
        self.print_transaction(&transaction);
        
 
        self.transactions.push_back(transaction);
        
     
        self.check_achievements(&rarity);
        
  
        let reward = match rarity {
            CoffeeRarity::Common => 5,
            CoffeeRarity::Uncommon => 10,
            CoffeeRarity::Rare => 25,
            CoffeeRarity::Legendary => 50,
        };
        
        *self.user_balance.entry(self.current_user.clone()).or_insert(0) += reward;
        println!("\n{} {} {} for mining this coffee!", "You earned".green(), reward.to_string().yellow(), "CoffeeCoins".green());
    }
    
    fn print_transaction(&self, transaction: &CoffeeTransaction) {
        
        let rarity_display = match transaction.rarity {
            CoffeeRarity::Common => "Common".normal(),
            CoffeeRarity::Uncommon => "Uncommon".cyan(),
            CoffeeRarity::Rare => "Rare".yellow(),
            CoffeeRarity::Legendary => "★ Legendary ★".magenta(),
        };
        
        println!("\n{}", "☕ COFFEE MINED ☕".green());
        println!("Transaction ID: {}", transaction.id);
        println!("Coffee Type: {}", transaction.coffee_type);
        println!("Rarity: {}", rarity_display);
        println!("Brewer: {}", transaction.brewer);
        println!("Timestamp: {}", transaction.timestamp);
        println!("Hash: {}", transaction.proof_of_coffee_hash);
    }

    fn check_achievements(&mut self, rarity: &CoffeeRarity) {
        let brewer = self.current_user.clone();
        
    
        if !self.achievements["first_brew"].unlocked && self.current_id == 1 {
            self.unlock_achievement("first_brew", &brewer);
        }
        
    
        if !self.achievements["rare_find"].unlocked && 
           (*rarity == CoffeeRarity::Rare || *rarity == CoffeeRarity::Legendary) {
            self.unlock_achievement("rare_find", &brewer);
        }
        

        let unique_coffees: Vec<String> = self.transactions.iter()
            .filter(|t| t.brewer == brewer)
            .map(|t| t.coffee_type.clone())
            .collect::<std::collections::HashSet<String>>()
            .into_iter()
            .collect();
        
        if !self.achievements["legendary_barista"].unlocked && unique_coffees.len() >= 3 {
            self.unlock_achievement("legendary_barista", &brewer);
        }
    }
    
    fn unlock_achievement(&mut self, achievement_id: &str, brewer_name: &str) {
        if let Some(achievement) = self.achievements.get_mut(achievement_id) {
            achievement.unlocked = true;
            
            println!("\n{} {} {}", "🏆".yellow(), "ACHIEVEMENT UNLOCKED:".yellow(), achievement.name.yellow());
            println!("{}", achievement.description);
            
            let amount = achievement.reward;
            *self.user_balance.entry(brewer_name.to_string()).or_insert(0) += amount;
            println!("You received {} CoffeeCoins as reward!", amount);
        }
    }

    fn show_ledger(&self) {
        println!("\n{}", "======= COFFEE LEDGER =======".green());
        println!("Chain Length: {} blocks", self.transactions.len());
        
      // lsat 5 tnxs
        let transactions_to_show = self.transactions.iter().rev().take(5);
        for transaction in transactions_to_show {
            
            let rarity_display = match transaction.rarity {
                CoffeeRarity::Common => "Common".normal(),
                CoffeeRarity::Uncommon => "Uncommon".cyan(),
                CoffeeRarity::Rare => "Rare".yellow(),
                CoffeeRarity::Legendary => "★ Legendary ★".magenta(),
            };
            
            println!("\nID: {}", transaction.id);
            println!("Coffee: {} ({})", transaction.coffee_type, rarity_display);
            println!("Brewer: {}", transaction.brewer);
            println!("Timestamp: {}", transaction.timestamp);
            println!("Hash: {}", transaction.proof_of_coffee_hash);
            println!("------------------------------");
        }
        
        println!("\nTotal transactions: {}", self.transactions.len());
    }
    
    fn show_achievements(&self) {
        println!("\n{}", "===== ACHIEVEMENTS =====".yellow());
        for (_, achievement) in &self.achievements {
            let status = if achievement.unlocked {
                "✓".green()
            } else {
                "☐".red()
            };
            
            println!("\n{} {}", status, achievement.name.yellow());
            println!("{}", achievement.description);
            println!("Reward: {} CoffeeCoins", achievement.reward);
        }
        println!("=======================");
    }
    
    fn show_wallet(&self) {
        println!("\n{}", "===== COFFEE WALLET =====".green());
        println!("Username: {}", self.current_user);
        
        let balance = self.user_balance.get(&self.current_user).unwrap_or(&0);
        println!("Balance: {} CoffeeCoins", balance);
        println!("=======================");
    }
    
    fn set_username(&mut self, username: String) {
        if !username.is_empty() {
            self.current_user = username;
        } else {
            self.current_user = "Anonymous".to_string();
        }
        
        self.user_balance.entry(self.current_user.clone()).or_insert(10);
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
    
   
    println!("\nEnter your coffee brewer name:");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    ledger.set_username(username.trim().to_string());
    
    println!("\nWelcome, {}! You received 10 initial CoffeeCoins.", ledger.current_user);
    
    loop {
        println!("\n1. Brew Coffee (Mine)");
        println!("2. Show Coffee Ledger");
        println!("3. Show Achievements");
        println!("4. Show Wallet");
        println!("5. Exit");
        
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
            "3" => ledger.show_achievements(),
            "4" => ledger.show_wallet(),
            "5" => {
                println!("Exiting Proof-of-Coffee Simulator. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}