use crate::{
    AssetId, ContractId, CreateSupplyRequest, LedgerId, RunWarpRequest, Service, SupplyId,
    UniqueAssetId, User, U256,
};
use candid::{Nat, Principal};
use chrono::prelude::*;
use num_bigint::BigUint;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    io::{self, Write},
};

pub struct Scenarios<'a> {
    service: &'a Service,
    clmp_contract_id: ContractId,
}

const CLMP_LEDGER_ID: LedgerId = 1;

impl<'a> Scenarios<'a> {
    pub async fn init(service: &'a Service) -> Self {
        let clmp_contract_id = service
            .int_get_ledger_contract_id(&CLMP_LEDGER_ID)
            .await
            .unwrap()
            .unwrap();

        Self {
            service,
            clmp_contract_id,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        self.dfinity_demo().await
    }

    pub async fn dfinity_demo(&self) -> Result<(), Box<dyn Error>> {
        println!();
        let mut stdin_buffer = String::new();

        // get users
        let alice = User::alice();
        let bob = User::bob();
        let charlie = User::charlie();

        let exchange = User::exchange();

        // create 'Real Estate Token' asset
        self.service.set_identity(alice.identity.clone());

        let req_asset_re = CreateAssetRequest {
            asset_hash: Self::calculate_sha_256("Real Estate Token")?,
            ..CreateAssetRequest::default()
        };
        let unique_asset_id_re = self.create_asset(&req_asset_re).await?;

        // get asset
        let asset_re = self
            .service
            .led_base_get_asset(&self.clmp_contract_id, &req_asset_re.asset_id)
            .await?;
        println!(
            "0x{}: {}",
            hex::encode_upper(&unique_asset_id_re.0.to_bytes_be()),
            serde_json::to_string_pretty(&asset_re).unwrap()
        );

        // issue 1000000 tokens
        self.service
            .led_base_issue_tokens(
                &self.clmp_contract_id,
                &req_asset_re.asset_id,
                &Nat::from(1_000_000_u32),
            )
            .await?;

        print!("Creating exchange assets (press ENTER...)");
        let _ = std::io::stdout().flush();
        io::stdin().read_line(&mut stdin_buffer).unwrap();

        // create USD asset
        self.service.set_identity(exchange.identity.clone());

        let req_asset_usd = CreateAssetRequest {
            asset_hash: Self::calculate_sha_256("USD")?,
            ..CreateAssetRequest::default()
        };
        let unique_asset_id_usd = self.create_asset(&req_asset_usd).await?;
        println!("USD: {}", unique_asset_id_usd);

        self.service
            .led_base_issue_tokens(
                &self.clmp_contract_id,
                &req_asset_usd.asset_id,
                &Nat::from(20e9 as u128),
            )
            .await?;

        // create BTC asset
        self.service.set_identity(exchange.identity.clone());

        let req_asset_btc = CreateAssetRequest {
            asset_hash: Self::calculate_sha_256("BTC")?,
            ..CreateAssetRequest::default()
        };
        let unique_asset_id_btc = self.create_asset(&req_asset_btc).await?;
        println!("BTC: {}", unique_asset_id_btc);

        self.service
            .led_base_issue_tokens(
                &self.clmp_contract_id,
                &req_asset_btc.asset_id,
                &Nat::from(181_000_u32),
            )
            .await?;

        print!("Show balances (press ENTER...)");
        let _ = std::io::stdout().flush();
        io::stdin().read_line(&mut stdin_buffer).unwrap();

        println!("\nInitial holdings:");
        let print_balances_request = PrintBalancesRequest {
            unique_asset_id_re: &unique_asset_id_re,
            unique_asset_id_usd: &unique_asset_id_usd,
            unique_asset_id_btc: &unique_asset_id_btc,
            alice: &alice,
            bob: &bob,
            charlie: &charlie,
            exchange: &exchange,
        };
        self.print_balances(&print_balances_request).await?;
        println!();

        print!("Transfer 100 RE from Alice to Bob (press ENTER...)");
        let _ = std::io::stdout().flush();
        io::stdin().read_line(&mut stdin_buffer).unwrap();

        self.service.set_identity(alice.identity.clone());
        self.service
            .int_transfer_tokens(&unique_asset_id_re, &bob.principal, &Nat::from(100_u8))
            .await?;

        print!("Transfer 11 BTC from Exchange to Charlie (press ENTER...)");
        let _ = std::io::stdout().flush();
        io::stdin().read_line(&mut stdin_buffer).unwrap();

        self.service.set_identity(exchange.identity.clone());
        self.service
            .int_transfer_tokens(&unique_asset_id_btc, &charlie.principal, &Nat::from(11u8))
            .await?;

        println!("\nHoldings after transfer:");
        self.print_balances(&print_balances_request).await?;
        println!();

        print!("Create supplies (press ENTER...)");
        let _ = std::io::stdout().flush();
        io::stdin().read_line(&mut stdin_buffer).unwrap();

        let valid_until = Local::now()
            .checked_add_days(chrono::Days::new(10))
            .unwrap()
            .timestamp_nanos_opt()
            .unwrap() as u64;

        // create supply (1 RE = 200 USD)
        println!("RE => USD");
        self.service.set_identity(alice.identity.clone());
        let req_supply_re_usd = CreateSupplyRequest {
            offered: unique_asset_id_re.clone(),
            max_amount: Nat::from(1e5 as u128),
            desired: unique_asset_id_usd.clone(),
            exchange_rate: U256::from(5e6 as u128),
            valid_until,

            controller: None,
            receiver_address: None,
            ext_ref: 0x123_u32,
            take_all: false,
        };
        let supply_id_re_usd = self
            .service
            .int_create_supply(&req_supply_re_usd)
            .await?
            .data;
        println!("RE => USD supply id: {}", supply_id_re_usd);

        // create supply (1 BTC = 60000 USD)
        println!("USD => BTC");
        self.service.set_identity(exchange.identity.clone());
        let req_supply_usd_btc = CreateSupplyRequest {
            offered: unique_asset_id_usd.clone(),
            max_amount: Nat::from(10e5 as u128),
            desired: unique_asset_id_btc.clone(),
            exchange_rate: U256::from(6e13 as u128),
            valid_until,

            controller: None,
            receiver_address: None,
            ext_ref: 0x456_u32,
            take_all: false,
        };
        let supply_id_usd_btc = self
            .service
            .int_create_supply(&req_supply_usd_btc)
            .await?
            .data;
        println!("USD => BTC supply id: {}", supply_id_usd_btc);

        print!("\nRun warp (press ENTER...)");
        let _ = std::io::stdout().flush();
        io::stdin().read_line(&mut stdin_buffer).unwrap();

        self.service.set_identity(charlie.identity.clone());

        let warp_amount = U256::from(1 as u128);
        let warp_supplies = Vec::from([supply_id_usd_btc, supply_id_re_usd]);
        self.run_warp(warp_supplies, warp_amount).await?;

        print!("Show balances (press ENTER...)");
        let _ = std::io::stdout().flush();
        io::stdin().read_line(&mut stdin_buffer).unwrap();

        println!("\nHoldings after trade:");
        self.print_balances(&print_balances_request).await?;
        println!();

        Ok(())
    }

    async fn print_balances(&self, req: &PrintBalancesRequest<'a>) -> Result<(), Box<dyn Error>> {
        use prettytable::{Cell, Row, Table};

        Service::TRACE.set(false);

        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new(&format!("{:<15}", "")),
            Cell::new(&format!("Alice{:<15}", "")),
            Cell::new(&format!("Bob{:<15}", "")),
            Cell::new(&format!("Charlie{:<15}", "")),
            Cell::new(&format!("exchange{:<15}", "")),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Real estate"),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_re, &req.alice.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_re, &req.bob.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_re, &req.charlie.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_re, &req.exchange.principal)
                    .await?
                    .to_string(),
            ),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("USD"),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_usd, &req.alice.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_usd, &req.bob.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_usd, &req.charlie.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_usd, &req.exchange.principal)
                    .await?
                    .to_string(),
            ),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("BTC"),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_btc, &req.alice.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_btc, &req.bob.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_btc, &req.charlie.principal)
                    .await?
                    .to_string(),
            ),
            Cell::new(
                &self
                    .service
                    .int_get_balance(req.unique_asset_id_btc, &req.exchange.principal)
                    .await?
                    .to_string(),
            ),
        ]));

        table.printstd();

        Service::TRACE.set(true);

        Ok(())
    }

    fn calculate_sha_256(input: &str) -> Result<Nat, Box<dyn Error>> {
        let mut sha256 = Sha256::new();
        sha256.update(input);
        let hash_bytes: [u8; 32] = sha256.finalize().into();
        let hash_nat = Nat(BigUint::from_bytes_be(&hash_bytes));
        Ok(hash_nat)
    }

    async fn create_asset(
        &self,
        req: &CreateAssetRequest,
    ) -> Result<UniqueAssetId, Box<dyn Error>> {
        self.service
            .led_base_activate_asset(
                &self.clmp_contract_id,
                &req.asset_id,
                &req.asset_hash,
                &req.asset_bitwise,
                &req.asset_controller,
            )
            .await?;

        let mut unique_asset_id_bytes: Vec<u8> = Vec::new();
        let asset_id_bytes = req.asset_id.0.to_bytes_be();
        for i in 0..asset_id_bytes.len() {
            unique_asset_id_bytes.push(asset_id_bytes[i]);
        }
        unique_asset_id_bytes.extend_from_slice(&CLMP_LEDGER_ID.to_be_bytes());

        let unique_asset_id = Nat(BigUint::from_bytes_be(&unique_asset_id_bytes));

        Ok(unique_asset_id)
    }

    async fn run_warp(
        &self,
        supplies: Vec<SupplyId>,
        warp_amount: U256,
    ) -> Result<(), Box<dyn Error>> {
        let warp_target_address: Option<Principal> = None;

        let warp_req = RunWarpRequest {
            input_amount: warp_amount,
            target_address: warp_target_address,
            supplies,
        };
        self.service.int_run_warp(&warp_req).await?;

        Ok(())
    }
}

struct CreateAssetRequest {
    asset_id: AssetId,
    asset_hash: Nat,
    asset_bitwise: bool,
    asset_controller: Option<ContractId>,
}

impl CreateAssetRequest {
    pub fn default() -> Self {
        let mut rng = rand::thread_rng();
        let asset_id_bytes: [u8; 10] = rng.gen();
        let asset_id = Nat(BigUint::from_bytes_be(&asset_id_bytes));

        Self {
            asset_id,
            asset_hash: Nat::from(0x42 as u128),
            asset_bitwise: false,
            asset_controller: None,
        }
    }
}

struct PrintBalancesRequest<'a> {
    unique_asset_id_re: &'a AssetId,
    unique_asset_id_usd: &'a AssetId,
    unique_asset_id_btc: &'a AssetId,
    alice: &'a User,
    bob: &'a User,
    charlie: &'a User,
    exchange: &'a User,
}
