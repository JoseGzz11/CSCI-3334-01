#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount{ balance: initial_balance }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount < 0.0 {
            self.balance;
        }
        else {
            self.balance += amount;
        }
        
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount < 0.0 || amount > self.balance {
            self.balance;
        }
        else {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        return self.balance;
    }

    pub fn apply_interest(&mut self, mut percent: f64) {
        if percent > 0.0 {
            percent = percent * 0.01;

            self.balance = (self.balance * percent) + self.balance; 
        }
    }
}

fn main() {
    let mut account = BankAccount::new(1000.0);
    
    account.deposit(1000.0);
    println!("{:?}", account);

    account.deposit(800.0);
    println!("{:?}", account);

    account.withdraw(400.0);
    println!("{:?}", account);

    account.apply_interest(4.6);
    println!("{:?}", account);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(1500.0);
        assert_eq!(account.balance, 1500.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(1500.0);
        account.deposit(500.0);
        assert_eq!(account.balance, 2000.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(1500.0);
        account.withdraw(500.0);
        assert_eq!(account.balance, 1000.0);
    }

    // // Add more tests here
    #[test]
    fn test_check_balance() {
        // Write a test for checking the account balance
        let account = BankAccount::new(1500.0);

        assert_eq!(account.balance, 1500.0);
    }

    #[test]
    fn test_negative_withdraw() {
        // Write a test for withdrawing negative money
        let mut account = BankAccount::new(1500.0);
        account.withdraw(-500.0);
        assert_eq!(account.balance, 1500.0);
    }
    
    #[test]
    fn test_over_withdraw() {
        // Write a test for withdrawing too much money
        let mut account = BankAccount::new(1500.0);
        account.withdraw(2000.0);
        assert_eq!(account.balance, 1500.0);
    }
    
    #[test]
    fn test_negative_deposit() {
        // Write a test for depositing negative money
        let mut account = BankAccount::new(1500.0);
        account.deposit(-500.0);
        assert_eq!(account.balance, 1500.0);
    }

    #[test]
    fn test_apply_interest() {
        // Write a test for applying an interest rate percentage
        let mut account = BankAccount::new(1500.0);
        account.apply_interest(2.6);
        assert_eq!(account.balance, 1539.0);
    }

    #[test]
    fn test_apply_negative_interest() {
        // Write a test for applying a negative interest rate percentage
        let mut account = BankAccount::new(1500.0);
        account.apply_interest(-2.6);
        assert_eq!(account.balance, 1500.0);
    }

    
}