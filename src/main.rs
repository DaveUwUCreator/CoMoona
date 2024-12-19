use CoMoona::transaction_pool::Transaction;
use CoMoona::transaction_pool::TransactionPool;
use CoMoona::lunar_module::{LunarPhase, Validator};
use CoMoona::consensus::LunarConsensus;
use std::time::{SystemTime, UNIX_EPOCH};


fn main(){
    
    let validator = Validator::new(1000, [0;32]);
    let mut consensus = LunarConsensus::new(500);
    println!("🌒 Bienvendos a la CoMoona ! 🌒");
    println!("=================================");

    match consensus.register_validator(validator){
        Ok(_) => print!("✨ Validator registered successfully!"),
        Err(e) => println!("❌ Failed to register validator: {:?}", e),
    }


    //Empezamos el entorno
    let pool_size = 5;

    println!("Intializing TransactionPool with size: {}", pool_size);

    let mut pool = TransactionPool::new(pool_size);
    run_demo(&mut pool);
    //Pendientes de funcionalidad

    println!("\nMoona está lista para operar! 🚀");

   

}


fn run_demo(pool:&mut TransactionPool){
    //create and add a test transacton
    let test_tx = Transaction {
        id: [1;32],
        sender: [1;32],
        receiver: [2;32],
        amount: 100,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        signature: [0;64],
        fee: 10,
    
    };
    
    match pool.add_transaction(test_tx) {
        Ok(_) => println!("✨ Transaction added successfully!"),
        Err(e) => println!("❌ Error adding ransaction {:?}", e),
    }
}
