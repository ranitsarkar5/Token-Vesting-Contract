#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct VestingData {
    pub beneficiary: Address,
    pub total_amount: i128,
    pub released_amount: i128,
    pub start_time: u64,
    pub duration: u64,
}

#[contract]
pub struct TokenVestingContract;

#[contractimpl]
impl TokenVestingContract {

    // Initialize vesting contract
    pub fn init(
        env: Env,
        beneficiary: Address,
        total_amount: i128,
        start_time: u64,
        duration: u64,
    ) {
        let data = VestingData {
            beneficiary,
            total_amount,
            released_amount: 0,
            start_time,
            duration,
        };

        env.storage().instance().set(&Symbol::short("VEST"), &data);
    }

    // Calculate vested amount
    pub fn vested_amount(env: Env, current_time: u64) -> i128 {
        let data: VestingData = env.storage().instance().get(&Symbol::short("VEST")).unwrap();

        if current_time <= data.start_time {
            return 0;
        }

        let elapsed = current_time - data.start_time;

        if elapsed >= data.duration {
            return data.total_amount;
        }

        data.total_amount * elapsed as i128 / data.duration as i128
    }

    // Release tokens
    pub fn release(env: Env, current_time: u64) -> i128 {
        let mut data: VestingData = env.storage().instance().get(&Symbol::short("VEST")).unwrap();

        let vested = Self::vested_amount(env.clone(), current_time);
        let releasable = vested - data.released_amount;

        if releasable > 0 {
            data.released_amount += releasable;
            env.storage().instance().set(&Symbol::short("VEST"), &data);
        }

        releasable
    }

    // View contract data
    pub fn get_data(env: Env) -> VestingData {
        env.storage().instance().get(&Symbol::short("VEST")).unwrap()
    }
}