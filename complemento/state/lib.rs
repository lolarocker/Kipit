use ink_lang as ink;
use ink_storage::collections::HashMap as StorageHashMap;

#[ink::contract]
mod document_contract {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_prelude::vec::Vec;
    use ink_storage::traits::{PackedLayout, SpreadLayout};

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    pub struct Document {
        pub content: Vec<u8>,
        pub is_valid: bool,
    }

    #[ink(storage)]
    pub struct DocumentContract {
        documents: StorageHashMap<AccountId, Document>,
        owner: AccountId,
    }

    impl DocumentContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                documents: StorageHashMap::new(),
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn upload_document(&mut self, content: Vec<u8>) {
            let sender = self.env().caller();
            let document = Document {
                content: content.clone(),
                is_valid: self.validate_document(&content),
            };
            self.documents.insert(sender, document);
        }

        #[ink(message)]
        pub fn get_document(&self, account_id: AccountId) -> Option<&Document> {
            self.documents.get(&account_id)
        }

        #[ink(message)]
        pub fn notify_payment_methods(&self, other_contract: AccountId) {
            // Implement logic to notify another contract about uploaded documents
            // For example, trigger a function on the other contract.
            // Note: This is a placeholder, and the actual implementation depends on your specific use case.
            ink_env::debug_println("Notifying payment methods contract");
            self.env().emit_event(NotifyPaymentMethods {
                sender: self.env().caller(),
                other_contract,
            });
        }

        fn validate_document(&self, content: &[u8]) -> bool {
            // Implement document validation logic
            // For simplicity, always consider the document as valid in this example.
            true
        }
    }

    #[ink(event)]
    pub struct NotifyPaymentMethods {
        #[ink(topic)]
        sender: AccountId,
        #[ink(topic)]
        other_contract: AccountId,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn upload_and_notify() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut contract = DocumentContract::new();

            // Mock document content for testing
            let document_content = Vec::from("Mock document content");

            // Upload a document to the contract
            contract.upload_document(document_content.clone());

            // Notify payment methods contract about uploaded documents
            contract.notify_payment_methods(accounts.alice);

            // Verify that the document is valid
            let document = contract.get_document(accounts.alice);
            assert_eq!(document.unwrap().is_valid, true);
        }
    }
}
