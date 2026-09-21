use soroban_sdk::{symbol_short, Address, Env};

pub fn emit_stream_created(
    env: &Env,
    sender: &Address,
    recipient: &Address,
    stream_id: u64,
    token: &Address,
    deposit_amount: i128,
    start_time: u64,
    stop_time: u64,
) {
    env.events().publish(
        (symbol_short!("created"), sender.clone(), recipient.clone()),
        (stream_id, token.clone(), deposit_amount, start_time, stop_time),
    );
}

pub fn emit_tokens_withdrawn(
    env: &Env,
    recipient: &Address,
    stream_id: u64,
    amount: i128,
    remaining_balance: i128,
) {
    env.events().publish(
        (symbol_short!("withdraw"), recipient.clone()),
        (stream_id, amount, remaining_balance),
    );
}

pub fn emit_stream_canceled(
    env: &Env,
    stream_id: u64,
    sender_refund: i128,
    recipient_payout: i128,
) {
    env.events().publish(
        (symbol_short!("canceled"), stream_id),
        (sender_refund, recipient_payout),
    );
}
