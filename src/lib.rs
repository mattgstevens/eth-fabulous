mod account;
use crate::account::Account;
use colored::*;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn try_generate_wallet(regex: &Regex, trying: Arc<Mutex<bool>>) -> Option<Account> {
    let mut result = None;
    loop {
        let account = Account::rand_new();
        let address = account.address_as_hex();
        let mut trying = trying.lock().unwrap();
        if !*trying {
            break;
        } else if regex.is_match(&address) {
            result = Some(account);
            *trying = false;
            break;
        } else {
            eprint!("{}\r", account.address_as_hex());
        }
    }

    result
}

pub fn run(search: &str, cpus: usize) -> Result<(), String> {
    let regex = Arc::new(Regex::new(search).unwrap());
    let trying = Arc::new(Mutex::new(true));
    let account = Arc::new(Mutex::new(None));

    let mut workers = Vec::with_capacity(cpus);
    for _ in 0..cpus {
        let regex = Arc::clone(&regex);
        let trying = Arc::clone(&trying);
        let account = Arc::clone(&account);

        let worker = thread::spawn(move || {
            if let Some(result) = try_generate_wallet(&regex, trying) {
                *account.lock().unwrap() = Some(result);
            }
        });
        workers.push(worker);
    }

    for worker in workers {
        worker.join().unwrap();
    }

    let result = account.lock().unwrap().take().unwrap();

    eprint!("{}\r", result.address_as_hex());
    println!("\n{}\n", "found matching address!".magenta());
    println!("{:x}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run("000", 1).unwrap();
    }
}
