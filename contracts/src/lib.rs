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

    pub fn get_transaction(&self, creditor: Address, debtor: Address) -> I128 {
        self.transactions.get(creditor).get(debtor)
    }

}

// Seção de testes
#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::alloy_primitives::Address;
    use stylus_sdk::testing::*;

    // Função auxiliar para criar endereços de teste
    fn mock_address(id: u8) -> Address {
        Address::from([id; 20])
    }

    #[test]
    fn test_add_participant() {
        let vm = TestVM::default();
        let mut contract = Group::from(&vm);
        let wallet = mock_address(1);

        contract.add_participant(wallet).unwrap();
        assert_eq!(contract.participants.len(), 1);
        assert_eq!(contract.participants.get(0).unwrap(), wallet);
        assert_eq!(contract.balances.get(wallet), I128::ZERO);
    }

    #[test]
    fn test_is_participant() {
        let vm = TestVM::default();
        let mut contract = Group::from(&vm);
        let wallet1 = mock_address(1);
        let wallet2 = mock_address(2);

        contract.add_participant(wallet1).unwrap();
        assert!(contract.is_participant(wallet1));
        assert!(!contract.is_participant(wallet2));
    }

    #[test]
    #[should_panic(expected = "Wallet is not a participant")]
    fn test_get_balance_non_participant() {
        let vm = TestVM::default();
        let contract = Group::from(&vm);
        let wallet = mock_address(1);
        contract.get_balance(wallet); // Deve entrar em pânico
    }

    #[test]
    fn test_get_balance() {
        let vm = TestVM::default();
        let mut contract = Group::from(&vm);
        let wallet = mock_address(1);

        contract.add_participant(wallet).unwrap();
        assert_eq!(contract.get_balance(wallet), I128::ZERO);
    }

    #[test]
    fn test_split_equally() {
        let vm = TestVM::default();
        let mut contract = Group::from(&vm);
        let payer = mock_address(1);
        let borrower1 = mock_address(2);
        let borrower2 = mock_address(3);

        contract.add_participant(payer).unwrap();
        contract.add_participant(borrower1).unwrap();
        contract.add_participant(borrower2).unwrap();

        let amount = I128::from_dec_str("100").unwrap();
        let borrowers = vec![borrower1, borrower2];
        contract.split_equally(payer, borrowers, amount).unwrap();

        assert_eq!(contract.get_balance(payer), I128::from_dec_str("100").unwrap());
        assert_eq!(contract.get_balance(borrower1), I128::from_dec_str("-50").unwrap());
        assert_eq!(contract.get_balance(borrower2), I128::from_dec_str("-50").unwrap());
    }

    #[test]
    #[should_panic(expected = "Borrowers array cannot be empty")]
    fn test_split_equally_empty_borrowers() {
        let vm = TestVM::default();
        let mut contract = Group::from(&vm);
        let payer = mock_address(1);
        contract.add_participant(payer).unwrap();
        contract.split_equally(payer, vec![], I128::from_dec_str("100").unwrap()).unwrap();
    }

    #[test]
    fn test_simplify_debts() {
        let vm = TestVM::default();
        let mut contract = Group::from(&vm);
        let wallet1 = mock_address(1); // Devedor
        let wallet2 = mock_address(2); // Credor
        let wallet3 = mock_address(3); // Devedor

        contract.add_participant(wallet1).unwrap();
        contract.add_participant(wallet2).unwrap();
        contract.add_participant(wallet3).unwrap();

        let borrowers = vec![wallet1, wallet3];
        contract.split_equally(wallet2, borrowers, I128::from_dec_str("100").unwrap()).unwrap();

        contract.simplify_debts();

        assert_eq!(contract.get_transaction(wallet2, wallet1), I128::from_dec_str("50").unwrap());
        assert_eq!(contract.get_transaction(wallet2, wallet3), I128::from_dec_str("50").unwrap());
        assert_eq!(contract.get_transaction(wallet1, wallet2), I128::ZERO);
    }

    #[test]
    fn test_get_transaction() {
        let vm = TestVM::default();
        let mut contract = Group::from(&vm);
        let creditor = mock_address(1);
        let debtor = mock_address(2);

        contract.add_participant(creditor).unwrap();
        contract.add_participant(debtor).unwrap();

        let mut inner_map = contract.transactions.setter(creditor);
        inner_map.insert(debtor, I128::from_dec_str("42").unwrap());

        assert_eq!(contract.get_transaction(creditor, debtor), I128::from_dec_str("42").unwrap());
        assert_eq!(contract.get_transaction(debtor, creditor), I128::ZERO); // Nenhum valor na direção oposta
    }
}

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
