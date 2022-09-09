pub mod core;
pub mod error;
pub mod refiner;

use crate::core::UserId;
use crate::error::Error;
use std::collections::HashMap;

use crate::core::client::Client;
use crate::core::AccountOwner;

fn process(intput_file: String) -> Result<HashMap<UserId, Client>, error::Error> {
    let querys = refiner::get_data(intput_file)?;
    let mut map: HashMap<UserId, Client> = HashMap::new();

    for q in querys {
        let user_id = q.user_id();
        let client = match map.get_mut(&user_id) {
            Some(client) => client,
            None => {
                let client = Client::new(user_id);
                map.insert(user_id, client);
                map.get_mut(&user_id).unwrap()
            }
        };

        if let Err(e) = client.dispatch(q) {
            eprintln!("Error while dispatching query : {:?}", e);
        }
    }

    Ok(map)
}

fn get_filename_from_args() -> Result<String, Error> {
    let mut args = std::env::args().skip(1);

    args.next().ok_or(Error::InputArgIsNotProvided)
}

fn main() {
    let input_file = get_filename_from_args();

    let clients = match input_file {
        Ok(f) => process(f).unwrap(),
        Err(e) => panic!("Error : {:?}\nUsage : cargo run -- [input_csv]", e),
    };

    println!("client,available,held,total,locked");
    for (_, client) in clients {
        println!("{}", client);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let clients = process("test_data/simple_transaction.csv".to_string()).unwrap();

        assert_eq!(
            format!("{}", clients.get(&1).unwrap()),
            "1,2.12,0,2.12,false"
        );
        assert_eq!(
            format!("{}", clients.get(&2).unwrap()),
            "2,18.9999,0,18.9999,false"
        );
    }

    #[test]
    fn complex_data() {
        let clients = process("test_data/complex_transaction.csv".to_string()).unwrap();

        assert_eq!(
            format!("{}", clients.get(&1).unwrap()),
            "1,139,20.5,159.5,false"
        );
        assert_eq!(
            format!("{}", clients.get(&2).unwrap()),
            "2,1970,0,1970,false"
        );
        assert_eq!(
            format!("{}", clients.get(&3).unwrap()),
            "3,1000,0,1000,true"
        );
    }

    #[test]
    fn float_data() {
        let clients = process("test_data/float_transaction.csv".to_string()).unwrap();

        assert_eq!(
            format!("{}", clients.get(&1).unwrap()),
            "1,4.8631,2.0962,6.9593,false"
        );
    }
}
