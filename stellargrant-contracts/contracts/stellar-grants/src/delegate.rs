use soroban_sdk::{Address, Env};

use crate::types::ContractError;

pub fn placeholder(_env: &Env, _address: &Address) -> Result<(), ContractError> {
    Ok(())
}
