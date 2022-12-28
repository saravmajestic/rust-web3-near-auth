use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

const PASSWORD_NUMBER: u8 = 1;
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    password_solution: String,
}

#[near_bindgen]

impl Contract {
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            password_solution: solution,
        }
    }

    pub fn get_solution(&self) -> String {
        self.password_solution.clone()
    }
    pub fn get_password_number(&self) -> u8 {
        PASSWORD_NUMBER
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(hashed_input);
        if hashed_input_hex == self.password_solution {
            env::log_str("Right password");
            true
        } else {
            env::log_str("Wrong password");
            false
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());
        let debug_solution = "sarav";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Debug: {:?}", debug_hash_string);
    }

    #[test]
    fn check_guess_solution() {
        let accountId = AccountId::new_unchecked("saravmajestic.testnet".to_string());
        let context = get_context(accountId);
        testing_env!(context.build());

        let mut contract = Contract::new(
            "f84967f8893494c0723cb7d8c9360cbe6e9a77267f701ab55408e4b1cf1856e7".to_string(),
        );
        let mut guess_result = contract.guess_solution("wrong answer".to_string());

        assert!(!guess_result, "Expectation: This is incorrect");
        assert_eq!(get_logs(), ["Wrong password"], "Expected a failure in logs");

        guess_result = contract.guess_solution("sarav".to_string());
        assert!(guess_result, "Expectation: This is correct");
        assert_eq!(
            get_logs(),
            ["Wrong password", "Right password"],
            "Passed successfully"
        );
    }
}
