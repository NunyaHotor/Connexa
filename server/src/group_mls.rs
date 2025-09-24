use openmls::prelude::*;
use uuid::Uuid;
use anyhow::Error;

pub struct GroupSession {
    pub group_id: Uuid,
    pub mls_group: MlsGroup,
}

impl GroupSession {
    pub fn new(group_id: Uuid, creator_credential: CredentialWithKey, backend: &impl OpenMlsProvider) -> Self {
        let group_id_bytes = group_id.as_bytes();
        let mls_group = MlsGroup::new(
            backend,
            &group_id_bytes,
            creator_credential.credential(),
            creator_credential.private_key(),
        ).expect("Failed to create MLS group");
        Self { group_id, mls_group }
    }

    /// Add a new member to the MLS group.
    pub fn add_member(
        &mut self,
        new_member_credential: CredentialWithKey,
        backend: &impl OpenMlsProvider,
    ) -> Result<(), Error> {
        let add_proposal = self.mls_group.propose_add(
            backend,
            &new_member_credential,
        )?;
        let (commit, welcome) = self.mls_group.commit(
            backend,
            &[add_proposal],
        )?;
        self.mls_group.apply_commit(commit, backend)?;
        Ok(())
    }

    /// Remove a member from the MLS group by their leaf index.
    pub fn remove_member(
        &mut self,
        leaf_index: LeafNodeIndex,
        backend: &impl OpenMlsProvider,
    ) -> Result<(), Error> {
        let remove_proposal = self.mls_group.propose_remove(
            backend,
            leaf_index,
        )?;
        let (commit, welcome) = self.mls_group.commit(
            backend,
            &[remove_proposal],
        )?;
        self.mls_group.apply_commit(commit, backend)?;
        Ok(())
    }

    /// Encrypt a group message.
    pub fn encrypt_message(
        &self,
        backend: &impl OpenMlsProvider,
        plaintext: &[u8],
    ) -> Result<HpkeCiphertext, Error> {
        self.mls_group.create_message(
            backend,
            plaintext,
        )
    }

    /// Decrypt a group message.
    pub fn decrypt_message(
        &mut self,
        backend: &impl OpenMlsProvider,
        ciphertext: &HpkeCiphertext,
    ) -> Result<Vec<u8>, Error> {
        let message = self.mls_group.parse_message(ciphertext.as_slice(), backend)?;
        Ok(message.as_bytes().to_vec())
    }
}