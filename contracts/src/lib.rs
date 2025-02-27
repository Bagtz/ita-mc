// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::prelude::*;
use stylus_sdk::{msg, alloy_primitives::Address};
use stylus_sdk::storage::{StorageMap, StorageVec, StorageString};

use alloy_primitives::{U64, Uint};

// Estrutura principal do contrato
sol_storage! {
    #[entrypoint]
    pub struct Splitwise {
        /// Contador para IDs de grupos
        uint64 group_counter;
        /// Mapeia ID do grupo para o grupo
        mapping(uint64 => Group) groups;
    }

    struct Group {
        string name;
        Participant[] participants;
        Transaction[] transactions;
        mapping(address => int128) balances;
    }

    struct Participant {
        string name;
        address wallet;
    }

    struct Transaction {
        address payer;
        uint128 amount;
        string description;
        address[] participants;
    }
}


#[public]
impl Splitwise {
    // Função para criar um novo grupo
    fn create_group(&mut self, name: String) -> Result<u64, Vec<u8>> {
        // Obtém o valor atual do contador
        let group_id = self.group_counter.get();
        // Incrementa o contador diretamente
        self.group_counter.set(group_id.checked_add(Uint::from(1)).unwrap());

        let mut group = Group::
        group.name.set_str(&name);

        self.groups.insert(U64::from(group_id), group);
        Ok(group_id)
    }

    /// Função para adicionar uma pessoa a um grupo
    fn add_participant(&mut self, group_id: u64, name: String, wallet: Address) -> Result<(), Vec<u8>> {
        let group_id_key = U64::from(group_id);
        if !self.groups.contains_key(&group_id_key) {
            return Err("Group does not exist".into());
        }

        let mut group = self.groups.get_mut(&group_id_key).unwrap();
        let mut participant = Participant::new();
        participant.name.set_str(&name);
        participant.wallet.set(wallet);

        // Verificar se a carteira já está no grupo
        for i in 0..group.participants.len() {
            let participant_wallet = group.participants.get(i).unwrap().wallet.get();
            if participant_wallet == wallet {
                return Err("Wallet already in group".into());
            }
        }

        group.participants.push(participant);
        Ok(())
    }

    /// Função para dividir uma despesa igualmente entre os participantes de um grupo
    fn split_equally(
        &mut self,
        group_id: u64,
        amount: u128,
        description: String,
        participants: Vec<Address>,
    ) -> Result<(), Vec<u8>> {
        let group_id_key = U64::from(group_id);
        if !self.groups.contains_key(&group_id_key) {
            return Err("Group does not exist".into());
        }

        let payer = msg::sender();
        let mut group = self.groups.get_mut(&group_id_key).unwrap();

        if !self.is_participant(group_id, payer) {
            return Err("Payer not in group".into());
        }
        for &participant in &participants {
            if !self.is_participant(group_id, participant) {
                return Err("A participant is not in group".into());
            }
        }

        let num_participants = participants.len() as u128;
        if num_participants == 0 {
            return Err("No participants specified".into());
        }
        let split_amount = amount / num_participants;

        // Atualizar saldo do pagador (negativo)
        let mut payer_balance = group.balances.get(&payer).unwrap_or(0);
        payer_balance -= amount as i128;
        group.balances.insert(payer, payer_balance);

        // Atualizar saldos dos participantes (positivo)
        for &participant in &participants {
            let mut balance = group.balances.get(&participant).unwrap_or(0);
            balance += split_amount as i128;
            group.balances.insert(participant, balance);
        }

        // Registrar a transação
        let mut transaction = Transaction::new();
        transaction.payer.set(payer);
        transaction.amount.set(amount);
        transaction.description.set_str(&description);
        for &participant in &participants {
            transaction.participants.push(participant);
        }
        group.transactions.push(transaction);

        Ok(())
    }

    /// Função auxiliar para verificar se uma carteira está no grupo
    fn is_participant(&self, group_id: u64, wallet: Address) -> bool {
        let group_id_key = U64::from(group_id);
        if let Some(group) = self.groups.get(&group_id_key) {
            for i in 0..group.participants.len() {
                if group.participants.get(i).unwrap().wallet.get() == wallet {
                    return true;
                }
            }
        }
        false
    }

    /// Função para consultar o saldo de uma pessoa em um grupo
    fn get_balance(&self, group_id: u64, wallet: Address) -> Result<i128, Vec<u8>> {
        let group_id_key = U64::from(group_id);
        if !self.groups.contains_key(&group_id_key) {
            return Err("Group does not exist".into());
        }

        if !self.is_participant(group_id, wallet) {
            return Err("Wallet not in group".into());
        }

        let group = self.groups.get(&group_id_key).unwrap();
        let balance = group.balances.get(&wallet).unwrap_or(0);
        Ok(balance)
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
