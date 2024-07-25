#![allow(dead_code)]

use candid::{self, CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_agent::{Agent, Identity};
use serde::{Serialize, Serializer};
use serde_bytes::ByteBuf;
use std::{cell::RefCell, error::Error, io::Write, rc::Rc};

pub type ContractId = Nat;
pub type SupplyId = Nat;
pub type U256 = Nat;
pub type UniqueAssetId = Nat;
pub type AccountId = Nat;
pub type LedgerId = u16;
pub type AmendmentId = Nat;
pub type AssetId = Nat;
pub type TxId = Nat;
pub type Hash = Nat;

#[derive(CandidType, Deserialize)]
pub struct Response {
    pub tx_id: TxId,
}

#[derive(CandidType, Deserialize)]
pub struct AccountUpdate {
    pub account_id: AccountId,
    pub previous_amount: U256,
    pub current_amount: U256,
}

#[derive(CandidType, Deserialize)]
pub struct AdministratorChanged {
    pub is_admin_status: bool,
    pub affected_address: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct AmendmentUpdate {
    pub amendment_id: AmendmentId,
}

#[derive(CandidType, Deserialize)]
pub enum AssetUpdateCode {
    DestroyTokensSuccess,
    SetAssetControllerSuccess,
    SetAssetIssuerSuccess,
    AssetCreationSuccess,
    IssueTokensSucesss,
    AmendmentCreationSuccess,
    AssetActivationSuccess,
    AssetCreationActivationSuccess,
}

#[derive(CandidType, Deserialize)]
pub struct AssetUpdate {
    pub event_id: AssetUpdateCode,
    pub asset_id: AssetId,
}

#[derive(CandidType, Deserialize)]
pub struct BlacklistChanged {
    pub code: u8,
    pub affected_address: Principal,
    pub controller_id: ContractId,
}

#[derive(CandidType, Deserialize)]
pub struct ControllerCreated {
    pub id: ContractId,
}

#[derive(CandidType, Deserialize)]
pub struct LedgerAdded {
    pub contract_id: ContractId,
}

#[derive(CandidType, Deserialize)]
pub struct LimitChanged {
    pub affected_address: Principal,
    pub controller_id: ContractId,
    pub new_limit: U256,
}

#[derive(CandidType, Deserialize)]
pub struct LimitConsumed {
    pub consumed_amount: U256,
    pub remaining_limit: U256,
    pub affected_address: Principal,
    pub controller_id: ContractId,
}

#[derive(CandidType, Deserialize)]
pub struct OwnershipTransferred {
    pub new_owner: Option<Principal>,
    pub previous_owner: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct PauseChanged {
    pub paused: bool,
}

#[derive(CandidType, Deserialize)]
pub struct PricingChanged {
    pub unique_asset_id: UniqueAssetId,
    pub new_fee_amount: U256,
    pub event_id: u8,
}

#[derive(CandidType, Deserialize)]
pub enum SupplyUpdateCode {
    NewSupplyCreated,
    SupplyTerminated,
    UpdateSupplyControllerWithNonEmptyController,
    UpdateSupplyRemoveController,
    SupplyConsumed,
    UpdateSupplyExpiryDate,
    RemainingAmountChangedToANewValueUpdateSupplyAmount,
    SupplyTerminatedByUpdateSupplyExchangeRateThisIsTheOldSupply,
    NewSupplyCreatedByUpdateSupplyExchangeRateThisIsTheNewSupply,
}

#[derive(CandidType, Deserialize)]
pub struct SupplyUpdate {
    pub supply_id: SupplyId,
    pub current_amount: U256,
    pub event_id: SupplyUpdateCode,
}

#[derive(CandidType, Deserialize)]
pub struct TokensCreated {
    pub unique_asset_id: UniqueAssetId,
    pub previous_amount: U256,
    pub current_amount: U256,
}

#[derive(CandidType, Deserialize)]
pub struct TokensDestroyed {
    pub unique_asset_id: UniqueAssetId,
    pub previous_amount: U256,
    pub current_amount: U256,
}

#[derive(CandidType, Deserialize)]
pub enum EventType {
    AssetUpdate,
    AdministratorChanged,
    AmendmentUpdate,
    SupplyUpdate,
    PauseChanged,
    LedgerAdded,
    LimitChanged,
    TokensCreated,
    ControllerCreated,
    LimitConsumed,
    TokensDestroyed,
    PricingChanged,
    AccountUpdate,
    OwnershipTransferred,
    BlacklistChanged,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionEvent {
    pub contract_id: Option<ContractId>,
    pub ledger_id: Option<LedgerId>,
    pub event_ix: u64,
    pub event_type: EventType,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    pub occured_on: u64,
    pub tx_id: TxId,
    pub events: Vec<TransactionEvent>,
}

#[derive(CandidType, Deserialize)]
pub struct CreateSupplyRequest {
    pub controller: Option<ContractId>,
    pub desired: UniqueAssetId,
    pub receiver_address: Option<Principal>,
    pub ext_ref: u32,
    pub valid_until: u64,
    pub offered: UniqueAssetId,
    pub take_all: bool,
    pub max_amount: U256,
    pub exchange_rate: U256,
}

#[derive(CandidType, Deserialize)]
pub struct ResponseSupplyId {
    pub tx_id: TxId,
    pub data: SupplyId,
}

#[derive(CandidType, Deserialize)]
pub struct SupplyParameters {
    pub controller: Option<ContractId>,
    pub desired_address: Option<Principal>,
    pub take_all: bool,
}

#[derive(CandidType, Deserialize)]
pub struct Supply {
    pub open_amount: U256,
    pub owner: Principal,
    pub parameters: Option<SupplyParameters>,
    pub valid_until: u64,
    pub exchange_rate: U256,
}

#[derive(CandidType, Deserialize)]
pub struct RunWarpRequest {
    pub input_amount: U256,
    pub target_address: Option<Principal>,
    pub supplies: Vec<SupplyId>,
}

#[derive(CandidType, Deserialize)]
pub struct ResponseAmendmentId {
    pub tx_id: TxId,
    pub data: AmendmentId,
}

#[derive(CandidType, Deserialize)]
pub struct Amendment {
    pub hash: ByteBuf,
    pub created_on: u64,
}

fn serialize_hash<S>(value: &Option<[u8; 32]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ser_d = match value {
        None => "null".to_string(),
        Some(h) => format!("0x{}", hex::encode_upper(h)),
    };
    serializer.serialize_str(&ser_d)
}

#[derive(CandidType, Deserialize, Debug, Serialize)]
pub struct Asset {
    pub bitwise: bool,
    #[serde(serialize_with = "serialize_hash")]
    pub hash: Option<[u8; 32]>,
    pub created_on: Option<u64>,
    pub issuer: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct ResponseContractId {
    pub tx_id: TxId,
    pub data: ContractId,
}

pub struct Service {
    agent: Rc<RefCell<Agent>>,
    canister_id: Principal,
}

impl Service {
    thread_local! {
        pub static TRACE: RefCell<bool> = RefCell::new(true);
    }

    pub fn new(agent: Rc<RefCell<Agent>>, canister_id: Principal) -> Self {
        Self { agent, canister_id }
    }

    pub fn set_identity<I>(&self, identity: I)
    where
        I: 'static + Identity,
    {
        self.agent.borrow_mut().set_identity(identity);
    }

    async fn query<T>(&self, method_name: &str, args: Vec<u8>) -> Result<T, Box<dyn Error>>
    where
        T: for<'de> Deserialize<'de> + CandidType,
    {
        let trace = Self::TRACE.with(|t| t.borrow().clone());
        if trace {
            print!("[query] {}...", method_name);
            let _ = std::io::stdout().flush();
        }

        let response = &self
            .agent
            .borrow()
            .query(&self.canister_id, method_name)
            .with_arg(args)
            .await?;

        let result = Decode!(response.as_slice(), T)?;

        if trace {
            println!(" ok");
            let _ = std::io::stdout().flush();
        }

        Ok(result)
    }

    async fn update<T>(&self, method_name: &str, args: Vec<u8>) -> Result<T, Box<dyn Error>>
    where
        T: for<'de> Deserialize<'de> + CandidType,
    {
        let trace = Self::TRACE.with(|t| t.borrow().clone());
        if trace {
            print!("[update] {}...", method_name);
            let _ = std::io::stdout().flush();
        }

        let response = &self
            .agent
            .borrow()
            .update(&self.canister_id, method_name)
            .with_arg(args)
            .await?;

        let result = Decode!(response.as_slice(), T)?;

        if trace {
            println!(" ok");
            let _ = std::io::stdout().flush();
        }

        Ok(result)
    }

    pub async fn ctr_get_consume_supply(
        &self,
        controller_id: &ContractId,
        receiver: &Principal,
        supply_id: &SupplyId,
        amount: &U256,
    ) -> Result<u8, Box<dyn Error>> {
        let method_name = "ctr_get_consume_supply";
        let args = Encode!(controller_id, receiver, supply_id, amount)?;
        self.query(method_name, args).await
    }

    pub async fn ctr_get_make_supply(
        &self,
        controller_id: &ContractId,
        owner: &Principal,
        offered_unique_asset_id: &UniqueAssetId,
        desired_unique_asset_id: &UniqueAssetId,
        amount: &U256,
    ) -> Result<u8, Box<dyn Error>> {
        let method_name = "ctr_get_make_supply";
        let args = Encode!(
            controller_id,
            owner,
            offered_unique_asset_id,
            desired_unique_asset_id,
            amount
        )?;
        self.query(method_name, args).await
    }

    pub async fn ctr_get_send(
        &self,
        controller_id: &ContractId,
        sender: &Option<Principal>,
        receiver: &Option<Principal>,
        amount: &U256,
    ) -> Result<u8, Box<dyn Error>> {
        let method_name = "ctr_get_send";
        let args = Encode!(controller_id, sender, receiver, amount)?;
        self.query(method_name, args).await
    }

    pub async fn ctr_remove_address(
        &self,
        contract_id: &ContractId,
        address: &Principal,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "ctr_remove_address";
        let args = Encode!(contract_id, address)?;
        self.update(method_name, args).await
    }

    pub async fn ctr_remove_address_array(
        &self,
        contract_id: &ContractId,
        addresses: &Vec<Principal>,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "ctr_remove_address";
        let args = Encode!(contract_id, addresses)?;
        self.update(method_name, args).await
    }

    pub async fn ctr_remove_blacklist(
        &self,
        contract_id: &ContractId,
        address: &Principal,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "ctr_remove_blacklist";
        let args = Encode!(contract_id, address)?;
        self.update(method_name, args).await
    }

    pub async fn ctr_remove_blacklist_array(
        &self,
        contract_id: &ContractId,
        addresses: &Vec<Principal>,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "ctr_remove_blacklist_array";
        let args = Encode!(contract_id, addresses)?;
        self.update(method_name, args).await
    }

    pub async fn ctr_set_blacklist(
        &self,
        contract_id: &ContractId,
        address: &Principal,
        code: &u8,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "ctr_set_blacklist";
        let args = Encode!(contract_id, address, code)?;
        self.update(method_name, args).await
    }

    pub async fn ctr_set_blacklist_array(
        &self,
        contract_id: &ContractId,
        addresses: &Vec<Principal>,
        codes: &Vec<u8>,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "ctr_set_blacklist_array";
        let args = Encode!(contract_id, addresses, codes)?;
        self.update(method_name, args).await
    }

    pub async fn ctr_set_limit(
        &self,
        contract_id: &ContractId,
        address: &Principal,
        limit: &U256,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "ctr_set_limit";
        let args = Encode!(contract_id, address, limit)?;
        self.update(method_name, args).await
    }

    pub async fn ctr_set_limit_array(
        &self,
        contract_id: &ContractId,
        addresses: &Vec<Principal>,
        limits: &Vec<U256>,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "ctr_set_limit_array";
        let args = Encode!(contract_id, addresses, limits)?;
        self.update(method_name, args).await
    }

    pub async fn ctr_validate_usage_controller(
        &self,
        controller: &Option<ContractId>,
    ) -> Result<bool, Box<dyn Error>> {
        let method_name = "ctr_validate_usage_controller";
        let args = Encode!(controller)?;
        self.query(method_name, args).await
    }

    pub async fn event_account_update_count(&self) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_account_update_count";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn event_account_update_get(
        &self,
        event_ix: &u64,
    ) -> Result<Option<AccountUpdate>, Box<dyn Error>> {
        let method_name = "event_account_update_get";
        let args = Encode!(event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_administrator_changed_count(
        &self,
        contract_id: &ContractId,
    ) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_administrator_changed_count";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn event_administrator_changed_get(
        &self,
        contract_id: &ContractId,
        event_ix: &u64,
    ) -> Result<Option<AdministratorChanged>, Box<dyn Error>> {
        let method_name = "event_administrator_changed_get";
        let args = Encode!(contract_id, event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_amendment_update_count(
        &self,
        ledger_id: &LedgerId,
    ) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_amendment_update_count";
        let args = Encode!(ledger_id)?;
        self.query(method_name, args).await
    }

    pub async fn event_amendment_update_get(
        &self,
        ledger_id: &LedgerId,
        event_ix: &u64,
    ) -> Result<Option<AmendmentUpdate>, Box<dyn Error>> {
        let method_name = "event_amendment_update_get";
        let args = Encode!(ledger_id, event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_asset_update_count(
        &self,
        ledger_id: &LedgerId,
    ) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_asset_update_count";
        let args = Encode!(ledger_id)?;
        self.query(method_name, args).await
    }

    pub async fn event_asset_update_get(
        &self,
        ledger_id: &LedgerId,
        event_ix: &u64,
    ) -> Result<Option<AssetUpdate>, Box<dyn Error>> {
        let method_name = "event_asset_update_get";
        let args = Encode!(ledger_id, event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_blacklist_changed_count(
        &self,
        contract_id: &ContractId,
    ) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_blacklist_changed_count";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn event_blacklist_changed_get(
        &self,
        contract_id: &ContractId,
        event_ix: &u64,
    ) -> Result<Option<BlacklistChanged>, Box<dyn Error>> {
        let method_name = "event_blacklist_changed_get";
        let args = Encode!(contract_id, event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_controller_created_count(&self) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_controller_created_count";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn event_controller_created_get(
        &self,
        event_ix: &u64,
    ) -> Result<Option<ControllerCreated>, Box<dyn Error>> {
        let method_name = "event_controller_created_get";
        let args = Encode!(event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_ledger_added_count(&self) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_ledger_added_count";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn event_ledger_added_get(
        &self,
        event_ix: &u64,
    ) -> Result<Option<LedgerAdded>, Box<dyn Error>> {
        let method_name = "event_ledger_added_get";
        let args = Encode!(event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_limit_changed_count(
        &self,
        contract_id: &ContractId,
    ) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_limit_changed_count";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn event_limit_changed_get(
        &self,
        contract_id: &ContractId,
        event_ix: &u64,
    ) -> Result<Option<LimitChanged>, Box<dyn Error>> {
        let method_name = "event_limit_changed_get";
        let args = Encode!(contract_id, event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_limit_consumed_count(
        &self,
        contract_id: &ContractId,
    ) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_limit_consumed_count";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn event_limit_consumed_get(
        &self,
        contract_id: &ContractId,
        event_ix: &u64,
    ) -> Result<Option<LimitConsumed>, Box<dyn Error>> {
        let method_name = "event_limit_consumed_get";
        let args = Encode!(contract_id, event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_ownership_transferred_count(
        &self,
        contract_id: &ContractId,
    ) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_ownership_transferred_count";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn event_ownership_transferred_get(
        &self,
        contract_id: &ContractId,
        event_ix: &u64,
    ) -> Result<Option<OwnershipTransferred>, Box<dyn Error>> {
        let method_name = "event_ownership_transferred_get";
        let args = Encode!(contract_id, event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_pause_changed_count(
        &self,
        contract_id: &ContractId,
    ) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_pause_changed_count";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn event_pause_changed_get(
        &self,
        contract_id: &ContractId,
        event_ix: &u64,
    ) -> Result<Option<PauseChanged>, Box<dyn Error>> {
        let method_name = "event_pause_changed_get";
        let args = Encode!(contract_id, event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_pricing_changed_count(&self) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_pricing_changed_count";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn event_pricing_changed_get(
        &self,
        event_ix: &u64,
    ) -> Result<Option<PricingChanged>, Box<dyn Error>> {
        let method_name = "event_pricing_changed_get";
        let args = Encode!(event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_supply_update_count(&self) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_supply_update_count";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn event_supply_update_get(
        &self,
        event_ix: &u64,
    ) -> Result<Option<SupplyUpdate>, Box<dyn Error>> {
        let method_name = "event_supply_update_get";
        let args = Encode!(event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_tokens_created_count(&self) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_tokens_created_count";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn event_tokens_created_get(
        &self,
        event_ix: &u64,
    ) -> Result<Option<TokensCreated>, Box<dyn Error>> {
        let method_name = "event_tokens_created_get";
        let args = Encode!(event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn event_tokens_destroyed_count(&self) -> Result<u64, Box<dyn Error>> {
        let method_name = "event_tokens_destroyed_count";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn event_tokens_destroyed_get(
        &self,
        event_ix: &u64,
    ) -> Result<Option<TokensDestroyed>, Box<dyn Error>> {
        let method_name = "event_tokens_destroyed_get";
        let args = Encode!(event_ix)?;
        self.query(method_name, args).await
    }

    pub async fn get_tx(&self, tx_id: &TxId) -> Result<Option<Transaction>, Box<dyn Error>> {
        let method_name = "get_tx";
        let args = Encode!(tx_id)?;
        self.query(method_name, args).await
    }

    pub async fn int_create_supply(
        &self,
        request: &CreateSupplyRequest,
    ) -> Result<ResponseSupplyId, Box<dyn Error>> {
        let method_name = "int_create_supply";
        let args = Encode!(request)?;
        self.update(method_name, args).await
    }

    pub async fn int_get_balance(
        &self,
        unique_asset_id: &UniqueAssetId,
        holder: &Principal,
    ) -> Result<U256, Box<dyn Error>> {
        let method_name = "int_get_balance";
        let args = Encode!(unique_asset_id, holder)?;
        self.query(method_name, args).await
    }

    pub async fn int_get_decimal_ptr(&self) -> Result<U256, Box<dyn Error>> {
        let method_name = "int_get_decimal_ptr";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn int_get_ledger_id(
        &self,
        contract_id: &ContractId,
    ) -> Result<LedgerId, Box<dyn Error>> {
        let method_name = "int_get_ledger_id";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn int_get_ledger_contract_id(
        &self,
        ledger_id: &LedgerId,
    ) -> Result<Option<ContractId>, Box<dyn Error>> {
        let method_name = "int_get_ledger_contract_id";
        let args = Encode!(ledger_id)?;
        self.query(method_name, args).await
    }

    pub async fn int_get_supply(
        &self,
        supply_id: &SupplyId,
    ) -> Result<Option<Supply>, Box<dyn Error>> {
        let method_name = "int_get_supply";
        let args = Encode!(supply_id)?;
        self.query(method_name, args).await
    }

    pub async fn int_get_tokens(
        &self,
        contract_id: &ContractId,
        asset_id: &AssetId,
    ) -> Result<U256, Box<dyn Error>> {
        let method_name = "int_get_tokens";
        let args = Encode!(contract_id, asset_id)?;
        self.query(method_name, args).await
    }

    pub async fn int_run_warp(&self, request: &RunWarpRequest) -> Result<Response, Box<dyn Error>> {
        let method_name = "int_run_warp";
        let args = Encode!(request)?;
        self.update(method_name, args).await
    }

    pub async fn int_set_contract(
        &self,
        ledger_contract_id: &ContractId,
        ledger_id: &LedgerId,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "int_set_contract";
        let args = Encode!(ledger_contract_id, ledger_id)?;
        self.update(method_name, args).await
    }

    pub async fn int_set_price(
        &self,
        fee_type: &u8,
        unique_asset_id: &UniqueAssetId,
        fee_amount: &U256,
        wallet: &Principal,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "int_set_price";
        let args = Encode!(fee_type, unique_asset_id, fee_amount, wallet)?;
        self.update(method_name, args).await
    }

    pub async fn int_set_supply_controller(
        &self,
        supply_id: &SupplyId,
        controller_id: &Option<ContractId>,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "int_set_supply_controller";
        let args = Encode!(supply_id, controller_id)?;
        self.update(method_name, args).await
    }

    pub async fn int_terminate_supply(
        &self,
        supply_id: &SupplyId,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "int_terminate_supply";
        let args = Encode!(supply_id)?;
        self.update(method_name, args).await
    }

    pub async fn int_transfer_tokens(
        &self,
        unique_asset_id: &UniqueAssetId,
        receiver: &Principal,
        amount: &U256,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "int_transfer_tokens";
        let args = Encode!(unique_asset_id, receiver, amount)?;
        self.update(method_name, args).await
    }

    pub async fn int_update_supply_amount(
        &self,
        supply_id: &SupplyId,
        new_total_amount: &U256,
        additional_amount: &U256,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "int_update_supply_amount";
        let args = Encode!(supply_id, new_total_amount, additional_amount)?;
        self.update(method_name, args).await
    }

    pub async fn int_update_supply_exchange_rate(
        &self,
        supply_id: &SupplyId,
        exchange_rate: &U256,
    ) -> Result<ResponseSupplyId, Box<dyn Error>> {
        let method_name = "int_update_supply_exchange_rate";
        let args = Encode!(supply_id, exchange_rate)?;
        self.update(method_name, args).await
    }

    pub async fn int_update_supply_expiry_date(
        &self,
        supply_id: &SupplyId,
        valid_until: &u64,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "int_update_supply_expiry_date";
        let args = Encode!(supply_id, valid_until)?;
        self.update(method_name, args).await
    }

    pub async fn led_amen_change_issuer(
        &self,
        contract_id: &ContractId,
        asset_id: &AssetId,
        hash: &Hash,
        new_issuer: &Principal,
    ) -> Result<ResponseAmendmentId, Box<dyn Error>> {
        let method_name = "led_amen_change_issuer";
        let args = Encode!(contract_id, asset_id, hash, new_issuer)?;
        self.update(method_name, args).await
    }

    pub async fn led_amen_create_amendment(
        &self,
        contract_id: &ContractId,
        asset_id: &AssetId,
        hash: &Hash,
    ) -> Result<ResponseAmendmentId, Box<dyn Error>> {
        let method_name = "led_amen_create_amendment";
        let args = Encode!(contract_id, asset_id, hash)?;
        self.update(method_name, args).await
    }

    pub async fn led_amen_get_amendment(
        &self,
        amendment_id: &AmendmentId,
    ) -> Result<Option<Amendment>, Box<dyn Error>> {
        let method_name = "led_amen_get_amendment";
        let args = Encode!(amendment_id)?;
        self.query(method_name, args).await
    }

    pub async fn led_base_activate_asset(
        &self,
        contract_id: &ContractId,
        asset_id: &AssetId,
        hash: &Hash,
        bitwise: &bool,
        controller: &Option<ContractId>,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "led_base_activate_asset";
        let args = Encode!(contract_id, asset_id, hash, bitwise, controller)?;
        self.update(method_name, args).await
    }

    pub async fn led_base_create_asset(
        &self,
        contract_id: &ContractId,
        asset_id: &AssetId,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "led_base_create_asset";
        let args = Encode!(contract_id, asset_id)?;
        self.update(method_name, args).await
    }

    pub async fn led_base_destroy_tokens(
        &self,
        contract_id: &ContractId,
        asset_id: &AssetId,
        amount: &U256,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "led_base_destroy_tokens";
        let args = Encode!(contract_id, asset_id, amount)?;
        self.update(method_name, args).await
    }

    pub async fn led_base_get_asset(
        &self,
        contract_id: &ContractId,
        asset_id: &AssetId,
    ) -> Result<Option<Asset>, Box<dyn Error>> {
        let method_name = "led_base_get_asset";
        let args = Encode!(contract_id, asset_id)?;
        self.query(method_name, args).await
    }

    pub async fn led_base_issue_tokens(
        &self,
        contract_id: &ContractId,
        asset_id: &AssetId,
        amount: &U256,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "led_base_issue_tokens";
        let args = Encode!(contract_id, asset_id, amount)?;
        self.update(method_name, args).await
    }

    pub async fn led_kyc_remove_usage_controller(
        &self,
        ledger_contract_id: &ContractId,
        asset_id: &AssetId,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "led_kyc_remove_usage_controller";
        let args = Encode!(ledger_contract_id, asset_id)?;
        self.update(method_name, args).await
    }

    pub async fn led_kyc_set_usage_controller(
        &self,
        ledger_contract_id: &ContractId,
        asset_id: &AssetId,
        controller_contract_id: &ContractId,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "led_kyc_set_usage_controller";
        let args = Encode!(ledger_contract_id, asset_id, controller_contract_id)?;
        self.update(method_name, args).await
    }

    pub async fn mng_contract_deployment_code(
        &self,
        contract_id: &ContractId,
    ) -> Result<String, Box<dyn Error>> {
        let method_name = "mng_contract_deployment_code";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn mng_contract_name(
        &self,
        contract_id: &ContractId,
    ) -> Result<String, Box<dyn Error>> {
        let method_name = "mng_contract_name";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn mng_contract_version(
        &self,
        contract_id: &ContractId,
    ) -> Result<String, Box<dyn Error>> {
        let method_name = "mng_contract_version";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn mng_create_clmp(
        &self,
        deployment_code: &String,
    ) -> Result<ResponseContractId, Box<dyn Error>> {
        let method_name = "mng_create_clmp";
        let args = Encode!(deployment_code)?;
        self.update(method_name, args).await
    }

    pub async fn mng_create_controller(
        &self,
        deployment_code: &String,
        owner: &Principal,
    ) -> Result<ResponseContractId, Box<dyn Error>> {
        let method_name = "mng_create_controller";
        let args = Encode!(deployment_code, owner)?;
        self.update(method_name, args).await
    }

    pub async fn mng_create_integration(
        &self,
        deployment_code: &String,
        decimal_pointer: &U256,
    ) -> Result<ResponseContractId, Box<dyn Error>> {
        let method_name = "mng_create_integration";
        let args = Encode!(deployment_code, decimal_pointer)?;
        self.update(method_name, args).await
    }

    pub async fn mng_grant_admin(
        &self,
        contract_id: &ContractId,
        user: &Principal,
    ) -> Result<bool, Box<dyn Error>> {
        let method_name = "mng_grant_admin";
        let args = Encode!(contract_id, user)?;
        self.query(method_name, args).await
    }

    pub async fn mng_get_integration(&self) -> Result<Option<ContractId>, Box<dyn Error>> {
        let method_name = "mng_get_integration";
        let args = Encode!()?;
        self.query(method_name, args).await
    }

    pub async fn mng_is_admin(
        &self,
        contract_id: &ContractId,
        user: &Principal,
    ) -> Result<bool, Box<dyn Error>> {
        let method_name = "mng_is_admin";
        let args = Encode!(contract_id, user)?;
        self.query(method_name, args).await
    }

    pub async fn mng_is_owner(
        &self,
        contract_id: &ContractId,
        user: &Principal,
    ) -> Result<bool, Box<dyn Error>> {
        let method_name = "mng_is_owner";
        let args = Encode!(contract_id, user)?;
        self.query(method_name, args).await
    }

    pub async fn mng_owner(
        &self,
        contract_id: &ContractId,
    ) -> Result<Option<Principal>, Box<dyn Error>> {
        let method_name = "mng_owner";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn mng_pause(&self, contract_id: &ContractId) -> Result<Response, Box<dyn Error>> {
        let method_name = "mng_pause";
        let args = Encode!(contract_id)?;
        self.update(method_name, args).await
    }

    pub async fn mng_paused(&self, contract_id: &ContractId) -> Result<bool, Box<dyn Error>> {
        let method_name = "mng_paused";
        let args = Encode!(contract_id)?;
        self.query(method_name, args).await
    }

    pub async fn mng_renounce_ownership(
        &self,
        contract_id: &ContractId,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "mng_renounce_ownership";
        let args = Encode!(contract_id)?;
        self.update(method_name, args).await
    }

    pub async fn mng_revoke_admin(
        &self,
        contract_id: &ContractId,
        user: &Principal,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "mng_revoke_admin";
        let args = Encode!(contract_id, user)?;
        self.update(method_name, args).await
    }

    pub async fn mng_transfer_ownership(
        &self,
        contract_id: &ContractId,
        user: &Principal,
    ) -> Result<Response, Box<dyn Error>> {
        let method_name = "mng_transfer_ownership";
        let args = Encode!(contract_id, user)?;
        self.update(method_name, args).await
    }

    pub async fn mng_unpause(&self, contract_id: &ContractId) -> Result<Response, Box<dyn Error>> {
        let method_name = "mng_unpause";
        let args = Encode!(contract_id)?;
        self.update(method_name, args).await
    }
}
