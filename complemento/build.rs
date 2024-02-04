use ink_lang as ink;
use ink_storage::collections::HashMap as StorageHashMap;
use sr25519::{Pair, Public, Signature};

#[ink::contract]
mod document_contract {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::traits::{PackedLayout, SpreadLayout};

    /// Type definition for document data.
    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    pub struct DocumentData {
        pub file_content: Vec<u8>,
        pub is_valid: bool,
        pub signature: Signature,
        // Add more fields as needed
    }

    /// The smart contract.
    #[ink(storage)]
    pub struct DocumentContract {
        documents: StorageHashMap<AccountId, DocumentData>,
        owner: AccountId,
    }

    impl DocumentContract {
        /// Constructor to initialize the contract.
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                documents: StorageHashMap::new(),
                owner,
            }
        }

        /// Upload a document to the contract.
        #[ink(message)]
        pub fn upload_document(&mut self, file_content: Vec<u8>) {
            let sender = self.env().caller();
            let document_data = DocumentData {
                file_content: file_content.clone(),
                is_valid: self.validate_ine(&file_content),
                signature: self.sign_message(&file_content),
            };
            self.documents.insert(sender, document_data);
        }

        /// Get a document's data by account ID.
        #[ink(message)]
        pub fn get_document_data(&self, account_id: AccountId) -> Option<&DocumentData> {
            self.documents.get(&account_id)
        }

        /// Function to simulate INE validation.
        fn validate_ine(&self, _file_content: &[u8]) -> bool {
            // Implement INE validation logic here
            // For simplicity, always consider the document as valid in this example.
            true
        }

        /// Sign a message using the owner's key pair.
        fn sign_message(&self, message: &[u8]) -> Signature {
            let key_pair = Pair::from_string(
                &self.owner.to_string(),
                None,
            )
            .expect("Failed to create key pair");
            key_pair.sign(message)
        }

        /// Verify the signature of a document.
        #[ink(message)]
        pub fn verify_signature(&self, account_id: AccountId) -> bool {
            if let Some(document_data) = self.documents.get(&account_id) {
                let key = Public::from_string(&account_id.to_string()).expect("Failed to create public key");
                document_data.signature.verify(&key, &document_data.file_content)
            } else {
                false
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn upload_and_verify_signature() {
            // Initialize a contract instance with a mock owner account ID
            let mut contract = DocumentContract::new(AccountId::new([0x42; 32]));

            // Mock account ID for testing
            let sender = AccountId::new([0x43; 32]);

            // Mock document content for testing
            let document_content = Vec::from("Mock document content");

            // Upload a document to the contract
            contract.upload_document(document_content.clone());

            // Verify the signature of the uploaded document
            let signature_valid = contract.verify_signature(sender);

            // Assert that the signature is valid
            assert!(signature_valid);
        }
    }
}

//

