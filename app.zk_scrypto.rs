// File: app.zk_scrypto.rs
// zk_soundness_vault: simple note-based XRD vault with zk/FHE-style commitments.

use scrypto::prelude::*;

/// A minimal zk-style soundness vault that locks XRD into "notes"
/// with opaque commitments, inspired by Aztec/Zama-style off-chain systems.
#[blueprint]
mod zk_soundness_vault {
    use scrypto::prelude::*;

      /// A single locked note in the vault.
    #[derive(ScryptoSbor, Debug, Clone)]
    pub struct Note {
        /// Off-chain zk/FHE commitment (hash, ciphertext, etc).
        pub commitment: String,
        /// Amount of XRD locked in this note.
        pub amount: Decimal,
        /// Whether this note has been spent.
        pub spent: bool,
    }

     #[derive(ScryptoSbor, Debug, Clone, Copy)]
    pub struct DepositEvent {
        pub note_id: u64,
        pub amount: Decimal,
        pub opaque_commitment: String,
    }

    #[derive(ScryptoSbor, Debug, Clone, Copy)]
    pub struct WithdrawalEvent {
        pub note_id: u64,
        pub amount: Decimal,
        pub recipient: ComponentAddress,
    }

    pub struct ZkSoundnessVault {
        /// Vault holding all locked XRD.
        vault: Vault,
        /// Mapping from note_id -> note data.
        notes: KeyValueStore<u64, Note>,
        /// Next note id to assign (also equal to number of notes ever created).
        next_note_id: u64,
        /// Total XRD currently locked across all unspent notes.
        total_locked: Decimal,
    }

    impl ZkSoundnessVault {
              /// Instantiate a new zk soundness vault component with an empty XRD vault
        /// and no owner (OwnerRole::None).
        pub fn instantiate() -> Global<ZkSoundnessVault> {
            let component = Self {
                              vault: Vault::new(VAULT_RESOURCE),
                notes: KeyValueStore::new(),
                next_note_id: 0,
                total_locked: Decimal::ZERO,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();

            component
        }

        pub fn deposit_with_commitment(&mut self, mut payment: Bucket, commitment: String) -> u64 {
                       assert!(
                payment.resource_address() == VAULT_RESOURCE,
                "Only XRD deposits are supported in this minimal example"
            );

            let amount = payment.amount();
                      assert!(
                amount.is_positive(),
                "Deposit amount must be strictly positive"
            );

            self.vault.put(payment);

            let note_id = self.next_note_id;
            self.next_note_id += 1;
            self.total_locked += amount;

            let note = Note {
                commitment,
                amount,
                spent: false,
            };

            self.notes.insert(note_id, note);

            // This event is intentionally minimal: off-chain systems inspired by
            // Aztec, Zama, or other soundness-focused zk frameworks are expected
            // to index these events and correlate them with encrypted state.
            emit_event(DepositEvent {
                note_id,
                amount,
                opaque_commitment: String::from("opaque:stored-off-chain (Aztec/Zama style)"),
            });

            note_id
        }

        pub fn withdraw_note(&mut self, note_id: u64, recipient: ComponentAddress) -> Bucket {
            let mut note = self
                .notes
                .get(&note_id)
                .expect("Unknown note id in zk_soundness_vault");

            assert!(!note.spent, "Note already spent");
            assert!(
                note.amount > Decimal::ZERO,
                "Note amount must be positive to withdraw"
            );
            assert!(
                self.total_locked >= note.amount,
                "Vault invariant broken: insufficient locked amount; soundness violation"
            );

            // In a real zk system, you would only allow this call after
            // verifying an off-chain proof of correctness. Here we assume
            // that verification is done off-chain and the call is authorized.
            note.spent = true;
            let amount = note.amount;
            note.amount = Decimal::ZERO;
            self.notes.insert(note_id, note);

            self.total_locked -= amount;
            let withdrawn = self.vault.take(amount);

            emit_event(WithdrawalEvent {
                note_id,
                amount,
                recipient,
            });

            withdrawn
        }
        /// Return the resource address managed by this vault (XRD in this example).
        pub fn get_vault_resource_address(&self) -> ResourceAddress {
            VAULT_RESOURCE
        }

        pub fn get_total_locked(&self) -> Decimal {
            self.total_locked
        }

            /// Return the number of notes ever created.
        /// This is equal to the next note id and includes spent notes.
        pub fn get_note_count(&self) -> u64 {
            self.next_note_id
        }
        /// Return the total number of notes ever created (including spent ones).
        pub fn get_total_notes_created(&self) -> u64 {
            self.next_note_id
        }

        pub fn get_note_metadata(&self, note_id: u64) -> Option<Note> {
            self.notes.get(&note_id)
        }
    }
            /// Approximate count of unspent notes by scanning the store.
        /// This is O(n) over the number of notes and intended for light use.
        pub fn get_unspent_note_count(&self) -> u64 {
            let mut count: u64 = 0;
            for (id, note) in self.notes.iter() {
                let _ = id; // silence unused warning if needed
                if !note.spent {
                    count += 1;
                }
            }
            count
        }

}
