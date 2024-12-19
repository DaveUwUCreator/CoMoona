// src/tests/transaction_test.rs

use CoMoona::lunar_module::{LunarPhase, Validator}; 
use CoMoona::TransactionPool;  
use CoMoona::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};
use hex; // Add this to Cargo.toml: hex = "0.4.3"

// First, let's update our Cargo.toml
/*
[package]
name = "comoona"
version = "0.1.0"
edition = "2021"

[dependencies]
hex = "0.4.3"
rand = "0.8"
sha2 = "0.10"
*/

pub fn run_transaction_tests() {
// Initialize our test environment with detailed logging
println!("🌒 Initializing CoMoona Transaction Test Environment 🌒");
println!("═══════════════════════════════════════════════════");

// Create transaction pool with detailed explanation
let pool_size = 5;
println!("Creating TransactionPool with size: {}", pool_size);
let mut transaction_pool = TransactionPool::new(pool_size);

// Create test transactions with detailed metadata
println!("\n📝 Creating test transactions...");
let transactions = create_test_transactions();

// Process each transaction with detailed logging
process_transactions(&mut transaction_pool, &transactions);

// Test pool overflow scenario
test_pool_overflow(&mut transaction_pool);

// Demonstrate block creation selection
demonstrate_block_selection(&transaction_pool);
}

fn create_test_transactions() -> Vec<Transaction> {
vec![
create_transaction(1, 100, 5, "Low fee transaction"),
create_transaction(2, 200, 15, "High fee transaction"),
create_transaction(3, 150, 10, "Medium fee transaction"),
]
}

fn create_transaction(id: u8, amount: u64, fee: u64, description: &str) -> Transaction {
let tx = Transaction {
id: [id; 32],
sender: [id; 32],
receiver: [id + 1; 32],
amount,
timestamp: SystemTime::now()
.duration_since(UNIX_EPOCH)
.unwrap()
.as_secs(),
signature: [0; 64],
fee,
};

println!("Created transaction:");
println!(" ID: {}", hex::encode(&tx.id[0..4]));
println!(" Amount: {} Moona Coins", amount);
println!(" Fee: {} Moona Coins", fee);
println!(" Description: {}", description);
println!(" ───────────────────────");

tx
}

fn process_transactions(pool: &mut TransactionPool, transactions: &[Transaction]) {
println!("\n🌓 Adding transactions to pool...");
println!("═══════════════════════════════");

for tx in transactions {
match pool.add_transaction(tx.clone()) {
Ok(_) => {
println!("✨ Transaction successfully added:");
println!(" ID: {}", hex::encode(&tx.id[0..4]));
println!(" Fee: {} Moona Coins", tx.fee);
println!(" Status: Accepted ✅");
},
Err(e) => {
println!("❌ Transaction failed:");
println!(" ID: {}", hex::encode(&tx.id[0..4]));
println!(" Error: {:?}", e);
}
}
println!(" ───────────────────────");
}
}

fn test_pool_overflow(pool: &mut TransactionPool) {
println!("\n🌔 Testing pool overflow scenario...");
println!("═══════════════════════════════════");

let overflow_tx = create_transaction(4, 50, 3, "Very low fee transaction (overflow test)");

println!("\nAttempting to add transaction to full pool...");
match pool.add_transaction(overflow_tx.clone()) {
Ok(_) => {
println!(" Status: Unexpectedly accepted ❓");
},
Err(e) => {
println!(" Status: Rejected as expected ✅");
println!(" Reason: {:?}", e);
}
}
}

fn demonstrate_block_selection(pool: &TransactionPool) {
println!("\n🌕 Selecting transactions for next block...");
println!("═════════════════════════════════════════");

let best_transactions = pool.get_best_transactions(2);
println!("Selected top {} transactions:", best_transactions.len());

for (index, tx) in best_transactions.iter().enumerate() {
println!("\nTransaction #{}", index + 1);
println!(" ID: {}", hex::encode(&tx.id[0..4]));
println!(" Fee: {} Moona Coins", tx.fee);
println!(" Amount: {} Moona Coins", tx.amount);
println!(" Priority: High ⭐");
}
}

// In src/main.rs, you would call it like this:
fn main() {
println!("Welcome to CoMoona Test Suite!");
println!("==============================\n");

run_transaction_tests();
}
