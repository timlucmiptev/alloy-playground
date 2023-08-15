#![allow(clippy::assertions_on_constants)]

use alloy_primitives::hex_literal::hex;
use alloy_primitives::{keccak256, U256};
use alloy_sol_types::{sol, SolCall, SolEvent, token::WordToken};

sol! {
    function foo(uint256 a, uint256 b) external view returns (uint256);

    #[derive(Default)]
    event MyEvent(bytes32 indexed a, uint256 b, string indexed c, bytes d);
}

fn main() {
    let call = fooCall {
        a: U256::from(1),
        b: U256::from(2),
    };
    let call_data = call.encode();
    println!("call data: {}", hex::encode(&call_data));

    let event = MyEvent {
        a: [0x11; 32],
        b: U256::from(1u64),
        c: keccak256("Hello World").into(),
        d: Vec::new(),
    };

    println!("event signature: {}", hex::encode(MyEvent::SIGNATURE_HASH));

    for topic in event.encode_topics() {
        println!("topic: {}", hex::encode(topic));
    }
}
