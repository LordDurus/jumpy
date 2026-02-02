#[cfg(feature = "gba")]
extern crate alloc;

#[cfg(feature = "gba")]
use alloc::vec::Vec;

use crate::runtime::book::Book;
pub type KeyId = u16;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Key {
	pub key_id: KeyId,
	pub is_used: bool,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Inventory {
	pub coins: u16,
	pub books: Vec<Book>,
	pub keys: Vec<Key>,
}

impl Inventory {
	pub fn new() -> Inventory {
		return Inventory {
			coins: 0,
			books: Vec::new(),
			keys: Vec::new(),
		};
	}

	pub fn add_coins(&mut self, amount: u16) {
		self.coins = self.coins.saturating_add(amount);
		return;
	}

	pub fn add_key(&mut self, key_id: u16) {
		if self.keys.iter().any(|k| k.key_id == key_id) {
			return;
		}

		self.keys.push(Key { key_id, is_used: false });

		return;
	}

	pub fn add_book(&mut self, book_id: u16, total_pages: u16) {
		if self.books.iter().any(|b| b.book_id == book_id) {
			return;
		}

		self.books.push(Book {
			book_id,
			current_page: 0,
			total_pages,
		});
	}

	pub fn get_book(&self, book_id: u16) -> Option<&Book> {
		return self.books.iter().find(|b| b.book_id == book_id);
	}
}
