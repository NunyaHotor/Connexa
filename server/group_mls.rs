use openmls::prelude::*;
use uuid::Uuid;

pub struct GroupSession {
    pub group_id: Uuid,
    pub mls_group: MlsGroup,
}

impl GroupSession {
    pub fn new(group_id: Uuid, creator_credential: CredentialWithKey, backend: &impl OpenMlsCryptoProvider) -> Self {
        let group_id_bytes = group_id.as_bytes();
        let mls_group = MlsGroup::builder()
            .with_group_id(GroupId::from_slice(group_id_bytes))
            .with_credential(creator_credential)
            .build(backend)
            .expect("Failed to create MLS group");
        Self { group_id, mls_group }
    }

    /// Add a new member to the MLS group.
    pub fn add_member(
        &mut self,
        new_member_credential: CredentialWithKey,
        backend: &impl OpenMlsCryptoProvider,
    ) -> Result<(), MlsGroupError> {
        let add_proposal = self.mls_group.create_add_proposal(
            backend,
            &new_member_credential,
        )?;
        let commit = self.mls_group.create_commit(
            backend,
            &[add_proposal],
            &[],
            false,
        )?;
        self.mls_group.apply_commit(commit.staged_commit, &[add_proposal], backend)?;
        Ok(())
    }

    /// Remove a member from the MLS group by their leaf index.
    pub fn remove_member(
        &mut self,
        leaf_index: LeafNodeIndex,
        backend: &impl OpenMlsCryptoProvider,
    ) -> Result<(), MlsGroupError> {
        let remove_proposal = self.mls_group.create_remove_proposal(
            backend,
            leaf_index,
        )?;
        let commit = self.mls_group.create_commit(
            backend,
            &[remove_proposal],
            &[],
            false,
        )?;
        self.mls_group.apply_commit(commit.staged_commit, &[remove_proposal], backend)?;
        Ok(())
    }

    /// Encrypt a group message.
    pub fn encrypt_message(
        &self,
        backend: &impl OpenMlsCryptoProvider,
        plaintext: &[u8],
    ) -> Result<MlsCiphertext, MlsGroupError> {
        self.mls_group.create_application_message(
            backend,
            plaintext,
            &[],
        )
    }

    /// Decrypt a group message.
    pub fn decrypt_message(
        &mut self,
        backend: &impl OpenMlsCryptoProvider,
        ciphertext: &MlsCiphertext,
    ) -> Result<Vec<u8>, MlsGroupError> {
        let message = self.mls_group.decrypt(backend, ciphertext, &[])?;
        Ok(message.into_bytes())
    }
}