// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::prelude::*;
use stylus_sdk::{msg, alloy_primitives::Address};
//use stylus_sdk::storage::{StorageMap, StorageVec, StorageString};
//use stylus_sdk::storage::{StorageAddress, StorageU64, StorageU128, StorageI128};

use alloy_primitives::{I128};

// Estrutura principal do contrato
sol_storage! {
    #[entrypoint]
    pub struct Group {
        address[] participants;
        mapping(address => string) names;
        mapping(address => int128) balances;
        mapping(address => mapping(address => int128)) debts;
    }
}


#[public]
impl Group {

    fn add_participant(&mut self, name: String, wallet: Address) -> Result<(), Vec<u8>> {
    
        // Criando um novo participante
        self.participants.push(wallet);
        self.names.setter(wallet).set_str(&name);
     
        // Inicializando o saldo do participante em 0
        self.balances.insert(wallet, I128::ZERO);

        Ok(())
    }

    fn get_balance(&self, wallet: Address) -> I128 {
        if !self.is_participant(wallet) {
            panic!("Wallet is not a participant");
        }

        return self.balances.get(wallet);
    }

    // Função para registrar uma despesa e ajustar os saldos
    fn add_expense(&mut self, payer: Address, borrowers: Vec<Address>, amount: I128) -> Result<(), Vec<u8>> {
        if borrowers.is_empty() {
            panic!("Borrowers array cannot be empty");
        }

        // Verifica se payer e todos os borrowers são participantes
        let mut payer_exists = false;
        for i in 0..self.participants.len() {
            let participant = self.participants.get(i).unwrap();
            if participant == payer {
                payer_exists = true;
            }
        }
        if !payer_exists {
            panic!("Payer is not a participant");
        }

        for borrower in &borrowers {
            let mut borrower_exists = false;
            for i in 0..self.participants.len() {
                if self.participants.get(i).unwrap() == *borrower {
                    borrower_exists = true;
                    break;
                }
            }
            if !borrower_exists {
                panic!("A borrower is not a participant");
            }
        }

        // Garante que o amount é positivo
        if amount < I128::ZERO {
            panic!("Amount must be positive");
        }

        // Calcula o valor por borrower
        let num_borrowers = I128::try_from(borrowers.len()).unwrap();
        let amount_per_borrower = amount / num_borrowers; // Divisão inteira

        // Ajusta os saldos
        let payer_balance = self.balances.get(payer);
        self.balances.insert(payer, payer_balance + amount);

        // Ajusta o saldo de cada borrower (diminui pela parte proporcional)
        for borrower in borrowers {
            let borrower_balance = self.balances.get(borrower);
            self.balances.insert(borrower, borrower_balance - amount_per_borrower);
        }

        Ok(())
    }

    /// Função auxiliar para verificar se uma carteira está no grupo
    fn is_participant(&self, wallet: Address) -> bool {
        let mut is_participant = false;
        for i in 0..self.participants.len() {
            if self.participants.get(i).unwrap() == wallet {
                is_participant = true;
                break;
            }
        }
        return is_participant;
    }

}


// /// Declare that `Counter` is a contract with the following external methods.
// #[public]
// impl Counter {
//     /// Gets the number from storage.
//     pub fn number(&self) -> U256 {
//         self.number.get()
//     }

//     /// Sets a number in storage to a user-specified value.
//     pub fn set_number(&mut self, new_number: U256) {
//         self.number.set(new_number);
//     }

//     /// Sets a number in storage to a user-specified value.
//     pub fn mul_number(&mut self, new_number: U256) {
//         self.number.set(new_number * self.number.get());
//     }

//     /// Sets a number in storage to a user-specified value.
//     pub fn add_number(&mut self, new_number: U256) {
//         self.number.set(new_number + self.number.get());
//     }

//     /// Increments `number` and updates its value in storage.
//     pub fn increment(&mut self) {
//         let number = self.number.get();
//         self.set_number(number + U256::from(1));
//     }

//     /// Adds the wei value from msg_value to the number in storage.
//     #[payable]
//     pub fn add_from_msg_value(&mut self) {
//         let number = self.number.get();
//         self.set_number(number + self.vm().msg_value());
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_counter() {
//         use stylus_sdk::testing::*;
//         let vm = TestVM::default();
//         let mut contract = Counter::from(&vm);

//         assert_eq!(U256::ZERO, contract.number());

//         contract.increment();
//         assert_eq!(U256::from(1), contract.number());

//         contract.add_number(U256::from(3));
//         assert_eq!(U256::from(4), contract.number());

//         contract.mul_number(U256::from(2));
//         assert_eq!(U256::from(8), contract.number());

//         contract.set_number(U256::from(100));
//         assert_eq!(U256::from(100), contract.number());

//         // Override the msg value for future contract method invocations.
//         vm.set_value(U256::from(2));

//         contract.add_from_msg_value();
//         assert_eq!(U256::from(102), contract.number());
//     }
// }
