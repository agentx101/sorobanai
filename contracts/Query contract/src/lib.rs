#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct aiquery;

#[contractimpl]
impl aiquery {
    pub fn intro(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("This is the AI query bot"), to]
    }
}

mod test;
