use scrypto::prelude::*;

#[blueprint]
mod zk_soundness_vault_scripts {
    use crate::zk_soundness_vault::ZkSoundnessVault;

    pub struct ZkSoundnessVaultScripts {
        /// Live component of the underlying vault.
        vault: Global<ZkSoundnessVault>,
    }

    impl ZkSoundnessVaultScripts {
        /// Instantiate the scripts wrapper around an **already deployed** vault component.
        ///
        /// Typical flow in a transaction manifest:
        /// 1) Call the vault’s own instantiate function first (whatever it’s called in `app.zk_scrypto.rs`),
        ///    capture the returned `Global<ZkSoundnessVault>`.
        /// 2) Call this `instantiate` and pass that vault as an argument.
        pub fn instantiate(
            vault: Global<ZkSoundnessVault>,
        ) -> Global<ZkSoundnessVaultScripts> {
            Self { vault }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize()
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

        /// Withdraw using a note id.
        ///
        /// - `note_id`: id previously returned from a deposit.
        ///
        /// Returns: `Bucket` of XRD withdrawn from the vault.
        pub fn withdraw_note_script(
            &mut self,
            note_id: u64,
        ) -> Bucket {
            // Assumes underlying blueprint method:
            //   pub fn withdraw_note(&mut self, note_id: u64) -> Bucket
            self.vault.withdraw_note(note_id)
        }

        /// Read-only helper: total XRD locked in the vault (according to its accounting).
        pub fn get_total_locked_via_script(&self) -> Decimal {
            self.vault.get_total_locked()
        }
        /// Return true if the vault currently has zero XRD locked.
        pub fn is_vault_empty_via_script(&self) -> bool {
            self.vault.get_total_locked().is_zero()
        }

        /// Read-only helper: how many notes have been created so far.
        pub fn get_note_count_via_script(&self) -> u64 {
            self.vault.get_note_count()
        }
    }
}
