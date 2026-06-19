#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, BytesN, Env, String, Symbol,
};

/// Storage keys used by the EvidenceAnchor registry.
///
/// * `Admin`                       — court authority that may approve custodians.
/// * `Custodian(addr)`             — jurisdiction (Symbol) of an approved custodian.
/// * `Anchor(hash, case_number)`   — the immutable anchor record for a piece of
///                                   evidence in a specific case.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Custodian(Address),
    Anchor(BytesN<32>, Symbol),
}

/// Immutable record persisted on-chain when a custodian anchors evidence.
/// The document itself stays off-chain — only the SHA-256 hash is stored.
#[contracttype]
#[derive(Clone)]
pub struct AnchorRecord {
    pub custodian: Address,
    pub jurisdiction: Symbol,
    pub description: String,
    pub timestamp: u64,
    pub ledger: u32,
}

#[contract]
pub struct EvidenceAnchor;

#[contractimpl]
impl EvidenceAnchor {
    /// Initialize the registry by recording the court-authority `admin`.
    /// Can only be called once. The admin is the sole address allowed to
    /// register custodians (e.g. court clerks, accredited attorneys).
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("registry already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Admin approves a new custodian for a given `jurisdiction`
    /// (e.g. `HANOI`, `NYSUP`, `EDTX`). Only approved custodians may
    /// later anchor evidence. Re-registering an address overwrites its
    /// jurisdiction.
    pub fn register_custodian(
        env: Env,
        admin: Address,
        custodian: Address,
        jurisdiction: Symbol,
    ) {
        admin.require_auth();
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("registry not initialized");
        if admin != stored_admin {
            panic!("only the registry admin may register a custodian");
        }
        env.storage()
            .persistent()
            .set(&DataKey::Custodian(custodian.clone()), &jurisdiction);

        env.events()
            .publish((symbol_short!("custodian"), custodian), jurisdiction);
    }

    /// A registered custodian anchors a document hash for a specific case
    /// number together with a short description. The current ledger
    /// timestamp and sequence are recorded on-chain to create a tamper-proof
    /// chain-of-custody timestamp. The document itself remains off-chain.
    ///
    /// Returns the Unix timestamp (seconds) recorded for the anchor.
    /// Panics if the same `(evidence_hash, case_number)` pair has already
    /// been anchored — evidence is write-once.
    pub fn anchor(
        env: Env,
        custodian: Address,
        evidence_hash: BytesN<32>,
        case_number: Symbol,
        description: String,
    ) -> u64 {
        custodian.require_auth();

        let jurisdiction: Symbol = env
            .storage()
            .persistent()
            .get(&DataKey::Custodian(custodian.clone()))
            .expect("address is not a registered custodian");

        let key = DataKey::Anchor(evidence_hash.clone(), case_number.clone());
        if env.storage().persistent().has(&key) {
            panic!("evidence already anchored for this case");
        }

        let timestamp = env.ledger().timestamp();
        let record = AnchorRecord {
            custodian: custodian.clone(),
            jurisdiction,
            description,
            timestamp,
            ledger: env.ledger().sequence(),
        };
        env.storage().persistent().set(&key, &record);

        env.events().publish(
            (symbol_short!("anchor"), custodian, case_number),
            evidence_hash,
        );

        timestamp
    }

    /// Quick existence check used by the court / opposing counsel:
    /// returns `1` if the `(evidence_hash, case_number)` pair has been
    /// anchored, `0` otherwise. Read-only, no authorization required.
    pub fn verify(env: Env, evidence_hash: BytesN<32>, case_number: Symbol) -> u32 {
        if env
            .storage()
            .persistent()
            .has(&DataKey::Anchor(evidence_hash, case_number))
        {
            1
        } else {
            0
        }
    }

    /// Returns the Unix timestamp (seconds) at which the evidence was
    /// anchored. Use this to prove the document existed in its current
    /// form at-or-before this moment. Panics if the evidence is missing.
    pub fn get_anchor(env: Env, evidence_hash: BytesN<32>, case_number: Symbol) -> u64 {
        let record: AnchorRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Anchor(evidence_hash, case_number))
            .expect("evidence not anchored");
        record.timestamp
    }

    /// Returns the custodian address that anchored the evidence.
    /// Useful to verify the chain-of-custody on dispute.
    /// Panics if the evidence is not in the registry.
    pub fn get_custodian(env: Env, evidence_hash: BytesN<32>, case_number: Symbol) -> Address {
        let record: AnchorRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Anchor(evidence_hash, case_number))
            .expect("evidence not anchored");
        record.custodian
    }

    /// Returns the ledger sequence at which the evidence was anchored.
    /// Combined with `get_anchor` (timestamp) it gives a second,
    /// independent proof of when the document was registered.
    pub fn get_ledger(env: Env, evidence_hash: BytesN<32>, case_number: Symbol) -> u32 {
        let record: AnchorRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Anchor(evidence_hash, case_number))
            .expect("evidence not anchored");
        record.ledger
    }

    /// Returns `true` if `addr` is currently an approved custodian.
    /// Read-only, no authorization required.
    pub fn is_custodian(env: Env, addr: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Custodian(addr))
    }

    /// Returns the jurisdiction Symbol assigned to a custodian.
    /// Panics if the address is not registered.
    pub fn custodian_jurisdiction(env: Env, addr: Address) -> Symbol {
        env.storage()
            .persistent()
            .get(&DataKey::Custodian(addr))
            .expect("address is not a registered custodian")
    }

    /// Returns the current registry admin (court authority).
    /// Panics if the registry has not been initialized.
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("registry not initialized")
    }
}
