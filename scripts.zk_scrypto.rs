// Scripts wrapper for zk_soundness_vault: convenient entrypoints for UIs/manifests.

use scrypto::prelude::*;

/// Blueprint providing script-style helpers around an existing
/// `ZkSoundnessVault` component.
#[blueprint]
mod zk_soundness_vault_scripts {
       use crate::zk_soundness_vault::ZkSoundnessVault as VaultBlueprint;

            pub struct ZkSoundnessVaultScripts {
        /// Live component of the underlying vault.
        vault: Global<VaultBlueprint>,
    }


    impl ZkSoundnessVaultScripts {
        /// Instantiate the scripts wrapper around an **already deployed** vault component.
        ///
        /// Typical flow in a transaction manifest:
        /// 1) Call the vault’s own instantiate function first (whatever it’s called in `app.zk_scrypto.rs`),
        ///    capture the returned `Global<ZkSoundnessVault>`.
        /// 2) Call this `instantiate` and pass that vault as an argument.
              pub fn instantiate(
            vault: Global<VaultBlueprint>,
        ) -> Global<ZkSoundnessVaultScripts> {
            Self { vault }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize()
        }
        /// Convenience: instantiate a new vault and a scripts wrapper around it
        /// in a single call.
        pub fn instantiate_with_new_vault() -> (Global<VaultBlueprint>, Global<ZkSoundnessVaultScripts>) {
            let vault: Global<VaultBlueprint> = VaultBlueprint::instantiate();
            let wrapper = Self { vault }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            (vault, wrapper)
        }

        /// Deposit XRD into the vault with a commitment.
        ///
        /// - `xrd`: bucket of XRD (taken from the caller’s account in the manifest)
        /// - `commitment`: opaque string created off-chain (hash, encrypted note, etc.)
        ///
        /// Returns: `note_id` created by the underlying vault.
        pub fn deposit_with_commitment_script(
            &mut self,
            xrd: Bucket,
            commitment: String,
        ) -> u64 {
            // Assumes underlying blueprint method:
            //   pub fn deposit_with_commitment(&mut self, xrd: Bucket, commitment: String) -> u64
            self.vault.deposit_with_commitment(xrd, commitment)
        }
        /// Convenience: deposit XRD with an empty commitment string.
        pub fn deposit_with_empty_commitment_script(
            &mut self,
            xrd: Bucket,
        ) -> u64 {
            let commitment = String::new();
            self.vault.deposit_with_commitment(xrd, commitment)
        }

              /// Withdraw using a note id, sending the XRD to the given recipient.
        ///
        /// - `note_id`: id previously returned from a deposit.
        /// - `recipient`: component or account address that should receive the XRD.
        ///
        /// Returns: `Bucket` of XRD withdrawn from the vault.
        pub fn withdraw_note_script(
            &mut self,
            note_id: u64,
            recipient: ComponentAddress,
        ) -> Bucket {
            // Assumes underlying blueprint method:
            //   pub fn withdraw_note(&mut self, note_id: u64, recipient: ComponentAddress) -> Bucket
            self.vault.withdraw_note(note_id, recipient)
        }
        /// Convenience: withdraw a note directly back to the caller.
        pub fn withdraw_note_to_caller_script(
            &mut self,
            note_id: u64,
        ) -> Bucket {
            let caller: ComponentAddress = Runtime::caller();
            self.vault.withdraw_note(note_id, caller)
        }
        /// Return the underlying vault component reference.
        pub fn get_underlying_vault(&self) -> Global<VaultBlueprint> {
            self.vault
        }

        /// Return the underlying vault's component address.
        pub fn get_underlying_vault_address(&self) -> ComponentAddress {
            self.vault.address()
        }

        /// Read-only helper: total XRD locked in the vault (according to its accounting).
        pub fn get_total_locked_via_script(&self) -> Decimal {
            self.vault.get_total_locked()
        }

        /// Read-only helper: how many notes have been created so far.
        pub fn get_note_count_via_script(&self) -> u64 {
            self.vault.get_note_count()
        }
    }
            /// Convenience view: get (total_locked, note_count) in one call.
        pub fn get_vault_stats_via_script(&self) -> (Decimal, u64) {
            let total = self.vault.get_total_locked();
            let count = self.vault.get_note_count();
            (total, count)
        }

}
