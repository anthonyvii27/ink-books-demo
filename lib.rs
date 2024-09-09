#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod books {
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;

    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Library {
        books: Vec<String>,
        owners: Vec<(AccountId, u32)>,
    }

    impl Library {
        #[ink(constructor)]
        pub fn new(book_list: Vec<String>, owner_list: Vec<(AccountId, u32)>) -> Self {
            Self {
                books: book_list,
                owners: owner_list,
            }
        }

        #[ink(message)]
        pub fn create_book(
            &mut self,
            name: String,
            category: String,
            author: String,
            owner_id: AccountId,
        ) {
            let mut new_book = String::new();
            new_book.push_str(&name);
            new_book.push_str(";");
            new_book.push_str(&category);
            new_book.push_str(";");
            new_book.push_str(&author);
            self.books.push(new_book);

            let index = self.books.len().checked_sub(1).unwrap_or(0) as u32;

            self.owners.push((owner_id, index));
        }

        #[ink(message)]
        pub fn update_book(
            &mut self,
            owner_id: AccountId,
            book_index: u32,
            new_name: Option<String>,
            new_category: Option<String>,
            new_author: Option<String>,
        ) -> String {
            if book_index as usize >= self.books.len() {
                return "Error: Book index out of range".to_string();
            }

            let (current_owner_id, _) = match self.owners.get(book_index as usize) {
                Some(owner) => owner,
                None => return "Error: Book index out of range".to_string(),
            };

            if *current_owner_id != owner_id {
                return "Error: You are not the owner of this book".to_string();
            }

            let mut book_parts: Vec<String> = self.books[book_index as usize]
                .split(';')
                .map(|s| s.to_string())
                .collect();

            if let Some(name) = new_name {
                book_parts[0] = name;
            }
            if let Some(category) = new_category {
                book_parts[1] = category;
            }
            if let Some(author) = new_author {
                book_parts[2] = author;
            }

            self.books[book_index as usize] = book_parts.join(";");
            "Success: Book updated".to_string()
        }

        #[ink(message)]
        pub fn get_books_by_owner_id(&self, target_owner_id: AccountId) -> Vec<String> {
            self.owners
                .iter()
                .filter_map(|(owner_id, index)| {
                    if *owner_id == target_owner_id {
                        self.books.get(*index as usize).cloned()
                    } else {
                        None
                    }
                })
                .collect()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::default_accounts;
        use ink::prelude::string::String;
        use ink::prelude::vec::Vec;

        #[ink::test]
        fn create_and_update_book_works() {
            let accounts = default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = Library::new(
                vec![
                    "Book1;Category1;Author1".into(),
                    "Book2;Category2;Author2".into()
                ],
                vec![(accounts.alice, 0), (accounts.bob, 1)]
            );

            contract.create_book(
                "Book3".into(),
                "Category3".into(),
                "Author3".into(),
                accounts.alice
            );

            let result = contract.update_book(
                accounts.alice,
                0,
                Some("UpdatedBook1".into()),
                None,
                None
            );
            assert_eq!(result, "Success: Book updated".to_string());

            let books = contract.get_books_by_owner_id(accounts.alice);
            assert_eq!(books.len(), 2);
            assert_eq!(books[0], "UpdatedBook1;Category1;Author1");
            assert_eq!(books[1], "Book3;Category3;Author3");
        }

        #[ink::test]
        fn update_book_out_of_range_fails() {
            let accounts = default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = Library::new(
                vec![
                    "Book1;Category1;Author1".into(),
                    "Book2;Category2;Author2".into()
                ],
                vec![(accounts.alice, 0), (accounts.bob, 1)]
            );

            let result = contract.update_book(
                accounts.alice,
                10, // Out-of-range index
                Some("NewName".into()),
                None,
                None
            );
            assert_eq!(result, "Error: Book index out of range".to_string());
        }

        #[ink::test]
        fn get_books_by_owner_id_works() {
            let accounts = default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = Library::new(
                vec![
                    "Book1;Category1;Author1".into(),
                    "Book2;Category2;Author2".into(),
                    "Book3;Category3;Author3".into()
                ],
                vec![
                    (accounts.alice, 0),
                    (accounts.alice, 1),
                    (accounts.bob, 2)
                ]
            );

            contract.create_book(
                "Book4".into(),
                "Category4".into(),
                "Author4".into(),
                accounts.alice
            );

            let alice_books = contract.get_books_by_owner_id(accounts.alice);
            assert_eq!(alice_books.len(), 3);
            assert_eq!(alice_books[0], "Book1;Category1;Author1");
            assert_eq!(alice_books[1], "Book2;Category2;Author2");
            assert_eq!(alice_books[2], "Book4;Category4;Author4");

            let bob_books = contract.get_books_by_owner_id(accounts.bob);
            assert_eq!(bob_books.len(), 1);
            assert_eq!(bob_books[0], "Book3;Category3;Author3");
        }
    }
}
