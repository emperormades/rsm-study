use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: CheckedAdd + CheckedSub + Zero + Copy
{
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, account: &AccountId, amount: Balance) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&self, account: &AccountId) -> Balance {
        *self.balances.get(account).unwrap_or(&Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` fas at least `amount` balance to transfer
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_balances() {
        let daniel = String::from("daniel");
        let vini = String::from("vini");

        let mut balances = Pallet::new();

        assert_eq!(balances.balance(&daniel), 0);
        balances.set_balance(&daniel, 10);

        assert_eq!(balances.balance(&daniel), 10);
        assert_eq!(balances.balance(&vini), 0);
    }

    #[test]
    fn transfer_balance() {
        let daniel = String::from("daniel");
        let vini = String::from("vini");
        let mut balances = Pallet::new();

        assert_eq!(balances.balance(&vini), 0);
        assert_eq!(
            balances.transfer(daniel.clone(), vini.clone(), 10),
            Err("Insufficient balance")
        );

        balances.set_balance(&daniel, 10);
        assert_eq!(
            balances.transfer(daniel.clone(), vini.clone(), 3),
            Ok(())
        );

        assert_eq!(balances.balance(&daniel), 7);
        assert_eq!(balances.balance(&vini), 3);

        balances.set_balance(&vini, u128::MAX);
        assert_eq!(
            balances.transfer(daniel, vini, 3),
            Err("Overflow")
        )
    }
}