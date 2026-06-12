#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, token, Address, Env, String};

// 1. Define Contract Errors
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    BountyNotFound = 1,
    BountyAlreadyResolved = 2,
    InvalidAmount = 3,
}

// 2. Storage Structures
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bounty {
    pub creator: Address,      
    pub token_address: Address, 
    pub amount: i128,   // Đồng bộ tên biến thành 'amount' theo đúng README và Lệnh CLI
    pub description: String,   
    pub is_resolved: bool,     
}

#[contracttype]
pub enum DataKey {
    Bounty(u32),        
    BountyCounter,      
}

// 3. Contract Logic
#[contract]
pub struct BountyContract;

#[contractimpl]
impl BountyContract {

    pub fn create_bounty(env: Env, creator: Address, token_address: Address, amount: i128, description: String) -> Result<u32, Error> {
        creator.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Transfer tokens from the creator's wallet into this escrow contract
        let client = token::Client::new(&env, &token_address);
        let contract_address = env.current_contract_address();
        client.transfer(&creator, &contract_address, &amount);

        // Increment global bounty counter ID
        let mut counter: u32 = env.storage().instance().get(&DataKey::BountyCounter).unwrap_or(0);
        counter += 1;
        env.storage().instance().set(&DataKey::BountyCounter, &counter);

        let new_bounty = Bounty {
            creator: creator.clone(),
            token_address,
            amount,
            description,
            is_resolved: false,
        };

        env.storage().instance().set(&DataKey::Bounty(counter), &new_bounty);
        Ok(counter)
    }

    pub fn get_bounty(env: Env, bounty_id: u32) -> Option<Bounty> {
        env.storage().instance().get(&DataKey::Bounty(bounty_id))
    }

    pub fn submit_and_claim(env: Env, participant: Address, bounty_id: u32, solution: String) -> Result<bool, Error> {
        participant.require_auth();

        let mut bounty: Bounty = env.storage().instance().get(&DataKey::Bounty(bounty_id))
            .ok_or(Error::BountyNotFound)?;

        if bounty.is_resolved {
            return Err(Error::BountyAlreadyResolved);
        }

        let correct_answer = String::from_str(&env, "soroban_is_awesome");
        
        if solution == correct_answer {
            // Update state first to prevent re-entrancy attacks
            bounty.is_resolved = true;
            env.storage().instance().set(&DataKey::Bounty(bounty_id), &bounty);

            // Release escrowed tokens directly to the successful developer
            let client = token::Client::new(&env, &bounty.token_address);
            let contract_address = env.current_contract_address();
            client.transfer(&contract_address, &participant, &bounty.amount);

            return Ok(true); 
        }

        Ok(false) 
    }
}

// 4. Inline Test Module
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::token::StellarAssetClient;

    #[test]
    fn test_bounty_flow() {
        let env = Env::default();
        env.mock_all_auths(); 

        let contract_id = env.register_contract(None, BountyContract);
        let client = BountyContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let developer = Address::generate(&env);

        // Initialize a mock Stellar Asset Token (e.g., XLM)
        let token_admin = Address::generate(&env);
        let token_contract_id = env.register_stellar_asset_contract(token_admin);
        let token_client = token::Client::new(&env, &token_contract_id);
        let token_admin_client = StellarAssetClient::new(&env, &token_contract_id);

        // Mint 5,000 tokens to the creator's wallet balance
        token_admin_client.mint(&creator, &5000);
        assert_eq!(token_client.balance(&creator), 5000);

        // Test: Create Bounty (Escrows 1,000 tokens into the contract)
        let description = String::from_str(&env, "Solve this puzzle");
        let bounty_id = client.create_bounty(&creator, &token_contract_id, &1000, &description);
        assert_eq!(bounty_id, 1);

        // Verify ledger balances after initialization
        assert_eq!(token_client.balance(&creator), 4000);
        assert_eq!(token_client.balance(&contract_id), 1000);

        // Test: Wrong Submission -> State unchanged, no funds released
        let wrong_solution = String::from_str(&env, "wrong_answer");
        let is_success_wrong = client.submit_and_claim(&developer, &bounty_id, &wrong_solution).unwrap();
        assert_eq!(is_success_wrong, false);
        assert_eq!(token_client.balance(&developer), 0); 

        // Test: Valid Submission -> State switches to resolved, releases 1,000 tokens to developer
        let right_solution = String::from_str(&env, "soroban_is_awesome");
        let is_success_right = client.submit_and_claim(&developer, &bounty_id, &right_solution).unwrap();
        assert_eq!(is_success_right, true);

        // Verify final ledger balances
        assert_eq!(token_client.balance(&developer), 1000);
        assert_eq!(token_client.balance(&contract_id), 0);

        // Verify global state persistence
        let updated_bounty = client.get_bounty(&bounty_id).unwrap();
        assert_eq!(updated_bounty.is_resolved, true);
    }
}