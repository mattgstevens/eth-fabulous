mod account;
use crate::account::Account;
use colored::*;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn try_generate_wallet(
    regex: &Regex,
    trying: Arc<Mutex<bool>>,
    verbosity: u64,
) -> Option<Account> {
    let mut result = None;
    loop {
        let mut trying = trying.lock().unwrap();
        if *trying {
            let account = Account::rand_new();
            let address = account.address_as_hex();

            if regex.is_match(&address) {
                result = Some(account);
                *trying = false;
                break;
            }

            if verbosity >= 2 {
                eprint!("{}\r", account.address_as_hex());
            }
        } else {
            break;
        }
    }

    result
}

pub fn run(search: &str, cpus: usize, verbosity: u64) -> Result<Account, String> {
    let regex = Arc::new(Regex::new(search).unwrap());
    let trying = Arc::new(Mutex::new(true));
    let account = Arc::new(Mutex::new(None));

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

        let worker = thread::spawn(move || {
            if let Some(result) = try_generate_wallet(&regex, trying, verbosity) {
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
        println!("\n\n{}", "found account matching search!".magenta());
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

    Ok(account)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run("000", 1, 2).unwrap();
    }
}
