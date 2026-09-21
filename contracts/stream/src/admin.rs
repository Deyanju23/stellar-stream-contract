use soroban_sdk::{Address, Env};

use crate::error::StreamError;
use crate::storage;

pub fn initialize(env: &Env, admin: &Address) -> Result<(), StreamError> {
    if storage::has_admin(env) {
        return Err(StreamError::AlreadyInitialized);
    }
    storage::set_admin(env, admin);
    storage::set_next_stream_id(env, 0);
    Ok(())
}

pub fn require_admin(env: &Env, caller: &Address) -> Result<(), StreamError> {
    let admin = storage::get_admin(env)?;
    if *caller != admin {
        return Err(StreamError::Unauthorized);
    }
    caller.require_auth();
    Ok(())
}
