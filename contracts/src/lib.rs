// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::prelude::*;
use stylus_sdk::alloy_primitives::Address;

use alloy_primitives::I128;

// Estrutura principal do contrato
sol_storage! {
    #[entrypoint]
    pub struct Group {
        address[] participants;
        mapping(address => string) names;
        mapping(address => int128) balances;
        mapping(address => mapping(address => int128)) transactions;
    }
}


#[public]
impl Group {

    fn add_participant(&mut self, wallet: Address) -> Result<(), Vec<u8>> {
    
        // Criando um novo participante
        self.participants.push(wallet);
     
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

    // Função para registrar uma despesa e ajustar os saldos
    fn split_equally(&mut self, payer: Address, borrowers: Vec<Address>, amount: I128) -> Result<(), Vec<u8>> {
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

    //----------------------------------------------------------------------------------------------------------------------------

     // Função principal para calcular transações a partir de saldos e endereços
     pub fn simplify_debts(&mut self) {

        // 1. Coleta os saldos dos participantes
        let mut balances: Vec<I128> = Vec::new();
        let mut addresses = Vec::new();
        for i in 0..self.participants.len() {
            let addr = self.participants.get(i).unwrap();
            let balance = self.balances.get(addr); // 0 se não houver saldo
            balances.push(balance);
            addresses.push(addr);
        }

        // Separa credores e devedores com seus índices originais
        let mut creditors: Vec<(usize, I128)> = balances
            .iter()
            .enumerate()
            .filter(|(_, &b)| b > I128::ZERO)
            .map(|(i, &b)| (i, b)) // Converte i128 positivo para u128
            .collect();
        let mut debtors: Vec<(usize, I128)> = balances
            .iter()
            .enumerate()
            .filter(|(_, &b)| b < I128::ZERO)
            .map(|(i, &b)| (i, (-b))) // Converte i128 negativo para u128 (valor absoluto)
            .collect();

        // Ordena em ordem decrescente para priorizar grandes transações
        creditors.sort_by(|a, b| b.1.cmp(&a.1));
        debtors.sort_by(|a, b| b.1.cmp(&a.1));

        let mut transactions = Vec::new();
        let mut creditor_idx = 0;
        let mut debtor_idx = 0;

        // Processa transações até zerar credores ou devedores
        while creditor_idx < creditors.len() && debtor_idx < debtors.len() {
            let creditor = creditors[creditor_idx];
            let debtor = debtors[debtor_idx];
            let amount = creditor.1.min(debtor.1); // Menor valor entre crédito e dívida

            // Adiciona a transação: (credor -> devedor, valor)
            transactions.push((
                addresses[creditor.0], // Endereço do credor
                addresses[debtor.0],   // Endereço do devedor
                amount,
            ));

            // Ajusta os saldos restantes
            creditors[creditor_idx].1 -= amount;
            debtors[debtor_idx].1 -= amount;

            // Avança para o próximo credor ou devedor se o saldo zerar
            if creditors[creditor_idx].1 == I128::ZERO {
                creditor_idx += 1;
            }
            if debtors[debtor_idx].1 == I128::ZERO {
                debtor_idx += 1;
            }

            
        }

        // 3. Armazena as transações no mapping transactions
        for (creditor, debtor, amount) in transactions {
            // Usar setter para o nível externo e configurar o valor no nível interno
            let mut inner_map = self.transactions.setter(creditor);
            // O StorageGuardMut não tem set, então usamos o método correto
            inner_map.insert(debtor, amount);
        }
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
