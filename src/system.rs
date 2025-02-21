use std::collections::BTreeMap;

type BlockNumber = u64;
type Nonce = u32;
type AccountId = String;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn increment_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nounce(&mut self, account: &AccountId) {
        let nonce = self.nonce.get(account).unwrap_or(&0) + 1;
        self.nonce.insert(account.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_system() {
        let daniel = String::from("daniel");
        let mut system = Pallet::new();

        assert_eq!(system.block_number(), 0);
        assert_eq!(system.nonce.get(&daniel), None);

        system.increment_block_number();
        assert_eq!(system.block_number(), 1);

        system.inc_nounce(&daniel);
        assert_eq!(system.nonce.get(&daniel).unwrap(), &1);
    }
}