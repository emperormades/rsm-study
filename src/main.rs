mod balances;
mod system;

mod types {
    pub type Balance = u128;
    pub type AccountId = String;
}
#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<types::AccountId, types::Balance>,
    system: system::Pallet
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    // start emulating a block
    runtime.system.increment_block_number();
    assert!(runtime.system.block_number() == 1);

    // first transaction
    runtime.system.inc_nounce(&alice);
    let _res = runtime.balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| println!("{}", e));

    // first transaction
    runtime.system.inc_nounce(&alice);
    let _res = runtime.balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("{}", e));

    println!("{:#?}", runtime);
}
