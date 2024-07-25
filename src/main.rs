mod agent;
mod scenarios;

use crate::agent::*;
use ic_agent::{export::Principal, identity::Secp256k1Identity, Agent, Identity};
use scenarios::Scenarios;
use std::{cell::RefCell, error::Error, io::{self, Write}, path::Path, rc::Rc};

#[tokio::main]
async fn main() {
    println!("Welcome to the demo!");
    print!("Press ENTER to start...");
    let _ = std::io::stdout().flush();
    io::stdin().read_line(&mut String::new()).unwrap();

    let service = match init_service().await {
        Err(e) => panic!("Error while init: {e}"),
        Ok(s) => s,
    };

    if let Err(e) = run(&service).await {
        panic!("Error during run: {e}")
    };

    print!("Press ENTER to exit...");
    let _ = std::io::stdout().flush();
    io::stdin().read_line(&mut String::new()).unwrap();

    println!("\n\n");
}

async fn init_service() -> Result<Service, Box<dyn Error>> {
    // localhost replica
    // let url = "http://localhost:4943";
    // let canister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai";

    // mainnet
    //let url = "https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io";
    let url = "https://a4gq6-oaaaa-aaaab-qaa4q-cai.ic0.app";
    let canister_id = "vqhr2-kqaaa-aaaag-alfea-cai";

    println!("Replica address: {}", url);
    println!("CoreLedger canister id: {}", canister_id);
    println!();

    let agent = Agent::builder().with_url(url).build()?;
    agent.fetch_root_key().await?;

    let agent = Rc::new(RefCell::new(agent));
    let canister_id = Principal::from_text(canister_id)?;

    let service = Service::new(Rc::clone(&agent), canister_id);
    Ok(service)
}

async fn run(service: &Service) -> Result<(), Box<dyn Error>> {
    let scenarios = Scenarios::init(&service).await;
    scenarios.run().await
}

struct User {
    identity: Secp256k1Identity,
    principal: Principal,
}

impl User {
    pub fn from_pem<P: AsRef<Path>>(file_path: P) -> Self {
        let identity = Secp256k1Identity::from_pem_file(file_path).unwrap();
        let principal = identity.sender().unwrap();

        User {
            identity,
            principal,
        }
    }

    pub fn alice() -> Self {
        Self::from_pem("./identities/alice.pem")
    }

    pub fn bob() -> Self {
        Self::from_pem("./identities/bob.pem")
    }

    pub fn charlie() -> Self {
        Self::from_pem("./identities/charlie.pem")
    }

    pub fn exchange() -> Self {
        Self::from_pem("./identities/exchange.pem")
    }
}
