// fn main() {
//     let mut _x = 5;
//
//     let  _r = &mut _x;
//
//     *_r += 1;
//     *_r -= 3;
//
//     println!("Value of _x = {}", _x);
//     // println!("Value of _r = {}", _r);
// }

fn main() {
    let mut account = BankAccount{
        owner: "Ahmed Habib".to_string(),
        balance: 555.95,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(45.65);

    // Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance is {}", self.owner, self.balance);
    }
}