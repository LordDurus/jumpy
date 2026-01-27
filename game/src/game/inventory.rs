use crate::game::book::BookId;
pub type KeyId = u16;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Book {
	pub book_id: BookId,
	pub current_page: u16,
	pub total_pages: u16,
}

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

	pub fn advance_book_page(&mut self, book_id: u16) {
		if let Some(book) = self.books.iter_mut().find(|b| b.book_id == book_id) {
			if book.current_page < book.total_pages {
				book.current_page += 1;
			}
		}
	}

	pub fn get_book(&self, book_id: u16) -> Option<&Book> {
		return self.books.iter().find(|b| b.book_id == book_id);
	}
}
