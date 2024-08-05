#[derive(Debug)]
//create Bank instance
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

//add inherent implementation (class) Account

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    //mutable method as argument, an i32 argument and return type
    fn deposit(&mut self, amount: i32) -> i32 {
        //increment balance with amount, then implicit return i32
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        //decrement balance with amount, return implicitly
        self.balance -= amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

//add inherent implementation of Bank

impl Bank {
    //reference whatever type Bank is with Self
    fn new() -> Self {
        //implicit return of Bank
        Bank { accounts: vec![] }
    }

    //make into a mutable reference and method
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}


// ampersands & , create a 'read-oonly' reference to a value
// fn print_account(account: &Account) {
//     println!("{:#?}", account);
// }

// fn print_holder(holder: String) {
//     println!("{holder}");
// }

// //passs a mutable reference to the argument wth &mut
// fn change_account(account: &mut Account) {
//     account.balance = 10;

// /*     println!("{:#?}", account.holder); */
// }

// fn make_and_print_account() -> &Account {
//     let account = Account::new(1, String::from("me"));

//     println!("{:#?}", account);

//     &account
// }

fn main() {
    // let mut account = Account::new(1, String::from("me"));

    // // allowed the account value to be  changed here
    // change_account(&mut account);


    // // here we created a 'read-only' immutable refernce to the acount variable above
    // let account_ref = &account;
    // //this will work because there are now two source of account, the original and the reference
    // print_account(account_ref);
    // //or
    // print_account(&account);

    // println!("{:#?}", account_ref.holder);
    //

  /*   make_and_print_account(); */
    // let account_ref = make_and_print_account();

    // println!("{}", account_ref.balance)
    //
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

 
    account.deposit(500);
    account.withdraw(250);

    bank.add_account(account);

    println!("{:#?}", bank)

}
