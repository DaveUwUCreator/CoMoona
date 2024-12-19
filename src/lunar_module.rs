
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Clone, Debug)]
pub enum LunarPhase {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

impl LunarPhase {
    pub fn from_block_number (block_number: u64) -> Self {
        match block_number % 8 {
            0 => LunarPhase::NewMoon,
            1 => LunarPhase::WaxingCrescent,
            2 => LunarPhase::FirstQuarter,
            3 => LunarPhase::WaxingGibbous,
            4 => LunarPhase::FullMoon,
            5 => LunarPhase::WaningGibbous,
            6 => LunarPhase::LastQuarter,
            _ => LunarPhase::WaningCrescent,
        }
    }
}
#[derive(Clone)]
pub struct Validator{
    pub(crate) public_key: [u8; 32],
    pub(crate) stake: u64,
    //other validator fields will go here
}

impl Validator {

    pub fn new(stake: u64, public_key: [u8; 32]) -> Self {
        Validator {stake, public_key}
    }

    pub fn get_stake(&self) -> u64 {
        self.stake
    }

    pub fn get_public_key(&self) -> [u8; 32]{
        self.public_key
    }
    
}

pub struct Header {
    block_number: u64,
    timestamp: u64,
    previous_hash: [u8; 32],
    merkle_root: [u8; 32],
    validator_signature: [u8; 64],
    lunar_phase: LunarPhase,
}

pub struct Body {
    transactions: Vec<Transaction>,
    //We'll add smart contract executions and state transitions later
}

pub struct ValidatorInfo {
    public_key:  [u8; 32],
    stake_amount: u64,
    selection_proof: [u8; 32],
}

pub struct LunarModule{
    header: Header,
    body: Body,
    validator_info: ValidatorInfo,
    moondust: u64,
}



impl LunarModule {
    pub fn new(prev_block: &LunarModule, transactions: Vec<Transaction>, validator: &Validator) -> Self {
        let header = Header {
            block_number: prev_block.header.block_number + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            previous_hash: Self::calculate_hash(&prev_block.header),
            merkle_root: Self::calculate_merkle_root(&transactions),
            validator_signature: [0; 64], //implementado por el validator
            lunar_phase: Self::calculate_lunar_phase(prev_block.header.block_number + 1),
        };
        let body = Body { transactions };
        let validator_info = ValidatorInfo {
            public_key: validator.public_key,
            stake_amount: validator.stake,
            selection_proof: [0; 32], //implementado con prueba VRF
        }; 
        LunarModule{
            header,
            body,
            validator_info,
            moondust: Self::generate_moondust(),
        }
 

    }
    fn calculate_hash(header: &Header) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(header.block_number.to_le_bytes());
        hasher.update(header.timestamp.to_le_bytes());
        hasher.update(&header.previous_hash);
        hasher.update(&header.merkle_root);
        hasher.finalize().into()
    }
    
    fn calculate_merkle_root(transactions: &[Transaction]) -> [u8; 32]{
        // Se necesita agregar complejidad basado en las fases lunares
        let mut hasher = Sha256::new();
        for tx in transactions {
            hasher.update(tx.id.as_bytes());
        }
        hasher.finalize().into()
    }

    fn calculate_lunar_phase(block_number: u64) -> LunarPhase{
        match block_number % 8 {
            0 => LunarPhase::NewMoon,
            1 => LunarPhase::WaxingCrescent,
            2 => LunarPhase::FirstQuarter,
            3 => LunarPhase::WaxingGibbous,
            4 => LunarPhase::FullMoon,
            5 => LunarPhase::WaningGibbous,
            6 => LunarPhase::LastQuarter,
            7 => LunarPhase::WaningCrescent,
            _ => unreachable!(),
        }
    }

    fn generate_moondust () -> u64{
        //la implementación del polvo lunar dependera de las primeras pruebas transaccionales.
        42
    }
}

//Placeholder structures, to be implemented later
pub struct Transaction {
    id: String,
    // Other transaction fields will go here
}

