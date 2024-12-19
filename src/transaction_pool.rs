use std::collections::{HashMap, HashSet};

#[derive (Clone, Debug)]

pub struct Transaction {
    pub id: [u8; 32],
    pub sender: [u8; 32],
    pub receiver: [u8; 32],
    pub amount: u64,
    pub timestamp: u64,
    pub signature: [u8; 64],
    pub fee: u64,
}

pub struct TransactionPool{
    pending: HashMap<[u8; 32],Transaction>,
    ordered_by_fee: Vec<[u8; 32]>,
    processed: HashSet<[u8;32]>,
    max_pool_size: usize,
}


#[derive(Debug)]
pub enum PoolError{
    AlreadyProcessed,
    PoolFull,
    InvalidTransaction,
}

impl TransactionPool {
    pub fn new(max_pool_size: usize) -> Self {
        TransactionPool {
            pending: HashMap::new(),
            ordered_by_fee: Vec::new(),
            processed: HashSet::new(),
            max_pool_size,
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result <(), PoolError>{
        //Basic validation
        if self.processed.contains(&transaction.id){
            return Err(PoolError::AlreadyProcessed);
        }

        if self.pending.len() >= self.max_pool_size {
            //Remove lowest fee transaction if pool is full
            if let Some(lowest_fee_id) = self.ordered_by_fee.first().cloned(){
                if self.pending[&lowest_fee_id].fee >= transaction.fee{
                    return Err(PoolError::PoolFull);
                }
                self.remove_transaction(&lowest_fee_id);
            }
        }

        //Insert new transaction
        let tx_id = transaction.id;
        let insert_pos = self.find_fee_position(transaction.fee);
        self.ordered_by_fee.insert(insert_pos, tx_id);
        self.pending.insert(tx_id, transaction);
        Ok(())
    }

    fn  find_fee_position (&self, fee: u64) -> usize {
        self.ordered_by_fee
            .binary_search_by_key(&fee, |id| {
                self.pending.get(id).map(|tx| tx.fee).unwrap_or(0)
            })
            .unwrap_or_else(|pos| pos)


    }



    pub fn get_best_transactions(&self, max_count: usize) -> Vec<Transaction>{
        self.ordered_by_fee
            .iter()
            .rev() //Start from highest fee
            .take(max_count)
            .filter_map(|id| self.pending.get(id))
            .cloned()
            .collect()
    }

    pub fn remove_transaction(&mut self, tx_id: &[u8; 32]){
        if let Some(pos) = self.ordered_by_fee.iter().position(|id| id == tx_id){
            self.ordered_by_fee.remove(pos);
        }
        self.pending.remove(tx_id);
        self.processed.insert(*tx_id);
    }

}
