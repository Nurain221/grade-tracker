#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct GradeTracker;

#[contractimpl]
impl GradeTracker {
    // 1. Create or Update a grade
    pub fn save_grade(env: Env, subject: String, grade: u32) {
        env.storage().instance().set(&subject, &grade);
    }

    // 2. Read a grade
    pub fn get_grade(env: Env, subject: String) -> u32 {
        env.storage().instance().get(&subject).unwrap_or(0)
    }

    // 3. Delete a grade
    pub fn delete_grade(env: Env, subject: String) {
        env.storage().instance().remove(&subject);
    }
}