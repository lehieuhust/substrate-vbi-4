

use std::hash::Hash;


use std::collections::HashMap;


use num::traits::{Zero, CheckedSub, CheckedAdd};
pub struct BalancesModule<AccountId, Balance> {
	balances: HashMap<AccountId, Balance>,
}


// impl Hash for AccountId (generic type) -> impl Hash for u32, impl Hash for u64
// impl Eq for AccountId (generic type)  -> impl Eq for u32, impl Eq for u32
impl<AccountId: Eq+ Hash, Balance :Zero + CheckedSub + CheckedAdd +Copy> BalancesModule<AccountId, Balance> {
	pub fn new() -> Self {
		Self {
			balances: HashMap::new()
		}
	}

	pub fn set_balance(&mut self, who: AccountId, amount: Balance) {
		self.balances.insert(who, amount);
	}

	pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
		let from_balance = self.balances.get(&from).ok_or("from user does not exist")?;
        let zero_balance = Balance::zero();
		let to_balance = self.balances.get(&to).unwrap_or(&zero_balance);

		let new_from_balance = from_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(from, new_from_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}


    pub fn get_balance(&self, who:AccountId) -> Balance {
    
        *self.balances.get(&who).unwrap_or(&Balance::zero())
    }

}











pub struct VotingModule<AccountId, VoteIndex> {

    votes: HashMap<(AccountId, VoteIndex), bool>
}


impl<AccountId: Eq + Hash , VoteIndex: Eq + Hash> VotingModule<AccountId, VoteIndex> {
    pub fn new() -> Self {
    
        Self {
            votes: HashMap::new()
        
        }
    }


    pub fn vote(&mut self, who: AccountId, index: VoteIndex, vote:bool){
    
        self.votes.insert((who,index), vote);
    }

    pub fn get_vote(&self, who:AccountId, index:VoteIndex) -> bool {
    
        *self.votes.get(&(who,index)).unwrap_or(&false)
    }
}


