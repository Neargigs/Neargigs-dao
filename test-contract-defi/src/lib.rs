// Import necessary modules and macros
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};
use near_sdk::json_types::U128;
use near_sdk::collections::LazyOption;

// Define the contract structure
#[near_bindgen]
#[derive(PanicOnDefault)]
pub struct TokenContract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

#[near_bindgen]
impl TokenContract {
    // Initialize the contract with the total supply and token details
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        // Create a new FungibleToken instance
        let mut token = FungibleToken::new(b"t".to_vec());

        // Mint the initial supply to the owner's account
        token.internal_register_account(&owner_id);
        token.internal_deposit(&owner_id, total_supply.0);

        // Set the token metadata (name, symbol, etc.)
        let metadata = FungibleTokenMetadata {
            spec: "ft-1.0.0".to_string(),
            name: "NGIG".to_string(), // <-- Set your token name here
            symbol: "NGIG".to_string(), // <-- Set your token symbol here
            decimals: 18, // Typically 18 decimals for fungible tokens
            icon: None, // Optionally, add a link to a token icon
            reference: None,
            reference_hash: None,
        };

        // Return the new TokenContract instance
        Self {
            token,
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        }
    }

    // Function to get the total supply of the token
    pub fn total_supply(&self) -> U128 {
        self.token.total_supply().into()
    }

    // Function to transfer tokens
    pub fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128) {
        self.token.internal_transfer(&env::predecessor_account_id(), &receiver_id, amount.0, None);
    }
}

// Implement the FungibleTokenMetadataProvider to provide metadata info
#[near_bindgen]
impl FungibleTokenMetadataProvider for TokenContract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

