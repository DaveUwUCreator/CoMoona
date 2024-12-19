use rand_chacha::ChaChaRng;
use rand_core::{RngCore, SeedableRng};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

use super::lunar_module::{LunarPhase, Validator};




pub struct LunarConsensus{
    validators: HashMap<[u8; 32], Validator>,
    current_phase: LunarPhase,
    vrf_seed: [u8; 32],
    minimum_stake: u64,
    
}

impl LunarConsensus{
    pub fn new(minimum_stake: u64) -> Self {
        LunarConsensus{
            validators: HashMap::new(),
            current_phase: LunarPhase::NewMoon,
            vrf_seed:[0;32],
            minimum_stake,
        }
    }
    pub fn register_validator(&mut self, validator: Validator) -> Result<(), ConsensusError>{
        if validator.get_stake() < self.minimum_stake{
            return Err(ConsensusError::InsufficientStake);
        }

        self.validators.insert(validator.get_public_key(), validator);
        Ok(())
    }
    
    pub fn select_validator(&self, block_number: u64) -> Option<&Validator>{
        let phase_weight = self.calculate_phase_weight();
        let mut total_weighted_stake = 0;

        //calculate total wighted stake
        for validator in self.validators.values(){
            total_weighted_stake += self.calculate_weighted_stake(validator, phase_weight);
        }

        //Generate random number based on block number and VRF seed
        let random_value = self.generate_random_value(block_number);
        let mut accumulator = 0;

        // select validator based on wghted probability

        for validator in self.validators.values(){
            accumulator += self.calculate_weighted_stake(validator, phase_weight);
            if random_value < accumulator{
                return Some(validator);
            }
        }
        None
    }

    fn calculate_phase_weight(&self) -> f64 {
        match self.current_phase {
            LunarPhase::NewMoon => 2.0, //favor smaller validators
            LunarPhase::FullMoon => 1.0, // Neutral weight
            _ => 1.5,                   //balanced weight
        }
    }

    fn calculate_weighted_stake(&self, validator: &Validator, phase_weight: f64) -> u64{
        let base_stake = validator.get_stake();
        let weight = if base_stake < self.minimum_stake * 2 {
            phase_weight
        } else {
            1.0 / phase_weight
        };
        (base_stake as f64 * weight) as u64
    }

    fn generate_random_value (&self , block_number: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(&self.vrf_seed);
        hasher.update(&block_number.to_le_bytes());
        let result = hasher.finalize();
        let mut rng = ChaChaRng::from_seed (result.into());
        rng.next_u64()
    }


    pub fn update_phase (&mut self, block_number: u64){
        self.current_phase = LunarPhase::from_block_number(block_number);
        //Generate new VRF seed every lunar cycle (8blocks)
        if block_number % 8 == 0 {
            let mut rng = ChaChaRng::from_entropy();
            rng.fill_bytes(&mut self.vrf_seed);
        }
    }

}

#[derive(Debug)]
pub enum ConsensusError{
    InsufficientStake,
    InvalidValidator,
    ConsensusFailure,
}