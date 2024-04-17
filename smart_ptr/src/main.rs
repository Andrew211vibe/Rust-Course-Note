// fn main() {
//     let arr = vec![Box::new(1), Box::new(2)];
//     let (first, second) = (&arr[0], &arr[1]);
//     let sum = **first + **second;
//     println!("sum: {:?}", sum);
// }

use std::cell::Cell;

struct Bank {
    balance: Cell<i32>,
}

impl Bank {
    fn new() -> Bank {
        Bank { balance: Cell::new(0) }
    }

    fn deposit(&self, amount: i32) {
        self.balance.set(self.balance.get() + amount);
    }

    fn withdraw(&self, amount: i32) -> bool {
        if self.balance.get() >= amount {
            self.balance.set(self.balance.get() - amount);
            true
        } else {
            false
        }
    }
}

fn main() {
    let bank = Bank::new();
    bank.deposit(100);
    assert!(bank.withdraw(50));
    assert_eq!(bank.balance.get(), 50);
}