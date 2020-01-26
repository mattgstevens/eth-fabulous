mod account;
use crate::account::Account;

use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use colored::*;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Instant;

pub fn try_generate_wallet(
    regex: &Regex,
    trying: Arc<Mutex<bool>>,
    verbosity: u64,
    counter: Arc<Mutex<usize>>,
) -> Option<Account> {
    let mut result = None;
    loop {
        let mut trying = trying.lock().unwrap();
        if *trying {
            let mut num = counter.lock().unwrap();
            *num += 1;

            let account = Account::rand_new();
            let address = account.address_as_hex();

            if verbosity >= 2 {
                eprint!("{}\r", account.address_as_hex());
            }

            if regex.is_match(&address) {
                result = Some(account);
                *trying = false;
                break;
            }
        } else {
            break;
        }
    }

    result
}

pub fn run(search: &str, cpus: usize, verbosity: u64) -> Result<Account, String> {
    let now = Instant::now();
    let regex = Arc::new(Regex::new(search).unwrap());
    let trying = Arc::new(Mutex::new(true));
    let account = Arc::new(Mutex::new(None));
    let counter = Arc::new(Mutex::new(0));

    if verbosity >= 2 {
        println!("searching for address containing: {}.", search);
        println!("using {} logical processors.", cpus);
        println!("using level {} verbosity.", verbosity);
        println!("\n");
        println!("searching...")
    }

    let mut workers = Vec::with_capacity(cpus);
    for _ in 0..cpus {
        let regex = Arc::clone(&regex);
        let trying = Arc::clone(&trying);
        let account = Arc::clone(&account);
        let counter = Arc::clone(&counter);

        let worker = thread::spawn(move || {
            if let Some(result) = try_generate_wallet(&regex, trying, verbosity, counter) {
                *account.lock().unwrap() = Some(result);
            }
        });
        workers.push(worker);
    }

    for worker in workers {
        worker.join().unwrap();
    }

    let account = account.lock().unwrap().take().unwrap();

    if verbosity >= 1 {
        print_found_account(&account, now, *counter.lock().unwrap());
    }

    Ok(account)
}


//
// --- iter style
//

struct AccountIter {}

impl AccountIter {
    pub fn new() -> AccountIter {
        AccountIter {}
    }
}

impl Iterator for AccountIter {
    type Item = Account;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Account::rand_new())
    }
}

pub fn run_iter(search: &str) -> Account {
    let now = Instant::now();
    let regex = Regex::new(search).unwrap();

    // let counter = Mutex::new(0);
    let counter = AtomicUsize::new(0);

    let account = AccountIter::new()
        .into_iter()
        .par_bridge()
        .find_any(|account| {
            let address = account.address_as_hex();

            // let mut num = counter.lock().unwrap();
            // *num += 1;
            // since we are not reading out the value across threads, we can use relaxed ordering
            counter.fetch_add(1, Ordering::Relaxed);

            regex.is_match(&address)
        })
        .unwrap();

    // print_found_account(&account, now, *counter.lock().unwrap());
    print_found_account(&account, now, counter.load(Ordering::Relaxed));

    account
}

pub fn print_found_account(account: &Account, now: Instant, counter: usize) {
    println!(
        "\n\n{}",
        format!(
            "found matching account in {} seconds.",
            now.elapsed().as_secs()
        )
        .magenta()
    );
    println!(
        "\n\n{}",
        format!(
            "searched through {} addresses.",
            counter
        )
        .magenta()
    );
    println!(
        "{}{}",
        "private key: ".yellow(),
        String::from(account.priv_key_as_hex()).cyan()
    );
    println!(
        "{}{}",
        "public key: ".yellow(),
        String::from(account.pub_key_as_hex()).cyan()
    );
    println!(
        "{}{}",
        "address: ".yellow(),
        String::from(account.address_as_hex()).cyan()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_iter() {
        run_iter("0000");
    }

    #[test]
    fn test_run() {
        run("0000", 1, 2).unwrap();
    }

}
