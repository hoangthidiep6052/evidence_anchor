# evidence_anchor

## Project Title
evidence_anchor — Court Evidence Timestamp Registry on Stellar Soroban

## Project Description
`evidence_anchor` is a Soroban smart-contract registry that lets court-approved
custodians (clerks, accredited attorneys, forensic labs) anchor the SHA-256
hash of a piece of digital evidence to a specific case number on the Stellar
blockchain. The actual document never leaves the custodian's secure storage —
only its hash, a short description, the anchoring party, the ledger sequence
and the Unix timestamp are committed on-chain. Once anchored, the record is
write-once and publicly verifiable, giving courts and opposing counsel a
tamper-proof proof-of-existence and a clean chain-of-custody trail.

## Project Vision
Today, contesting whether a PDF, email or photograph was altered after the
fact is slow, expensive and dependent on a small number of trusted notaries.
Our vision is to make "this exact file existed in this exact form at this
exact time" a one-second, near-free check that any judge, lawyer or citizen
can perform from a block explorer. By piggybacking on Stellar's ~5-second
finality and sub-cent fees, `evidence_anchor` aims to become the default
timestamp layer for court systems and law firms across emerging jurisdictions,
removing a major friction point in digital litigation while preserving the
confidentiality of the underlying documents.

## Key Features
- **Admin-controlled custodian registry** — only the court authority (the
  `admin` set in `init`) can approve or replace custodians via
  `register_custodian`, each tagged with a jurisdiction `Symbol`
  (e.g. `HANOI`, `NYSUP`).
- **Write-once anchoring** — `anchor(custodian, evidence_hash, case_number,
  description)` records the hash, ledger sequence and timestamp; the same
  `(hash, case_number)` pair cannot be overwritten.
- **Off-chain documents, on-chain proof** — only a `BytesN<32>` hash and a
  short `String` description are stored, so confidential filings never leave
  the custodian's secure storage.
- **Public, read-only verification** — `verify`, `get_anchor`, `get_ledger`,
  `get_custodian` and `is_custodian` let courts, opposing counsel and the
  public confirm chain-of-custody without any authorization.
- **Native Soroban auth** — every state-changing call uses `require_auth()`,
  so impersonating a custodian or the admin requires their actual signing key.
- **Auditable events** — `custodian` and `anchor` events are emitted so
  indexers and court dashboards can stream the registry in real time.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** legal dApp — see `contracts/evidence_anchor/src/lib.rs` for the full evidence_anchor business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CDN33Y24MTT2IPJ7HZC5TFPCN5M6KGSGJC6PRYMHR6KV2RQVQQGUAOGP`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/0c426261bcf9057a7f93e4aca67514611f292c999cc843209db224e6b1584c9c`

## Future Scope
- **Multi-signature custodianship** — require N-of-M custodians (e.g. clerk +
  forensic lab) to co-sign an anchor for high-stakes evidence.
- **Custodian revocation & rotation** — admin functions to revoke compromised
  custodians without invalidating their past anchors.
- **Per-jurisdiction sub-admins** — delegate custodian approval to regional
  court authorities while keeping a global root admin.
- **Encrypted off-chain metadata pointers** — optional IPFS / Storj CIDs and
  KMS-encrypted access policies so authorized parties can retrieve the
  underlying document with a single click from the on-chain record.
- **Cross-chain attestations** — bridge anchor records to Ethereum / Polygon
  rollups via Stellar's interop story so multi-jurisdiction cases can rely on
  a single source of truth.
- **Court-facing dApp** — a Freighter-powered web UI with role-based views
  (judge, custodian, attorney, public) and one-click verification reports
  suitable for filing as exhibits.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `evidence_anchor` (legal)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
