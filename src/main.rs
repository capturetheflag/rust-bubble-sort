mod book;
mod sort;

fn main() {
   let books = get_books();
   let sorted_books = sort::bubble_sort(books, &compare_books);

   for i in 0..sorted_books.len() {
      println!("{title} {pages}", title = sorted_books[i].get_title(), pages = sorted_books[i].get_number_of_pages());
   }
}

fn compare_books(a: &book::Book, b: &book::Book) -> i32 {
   return a.get_number_of_pages() - b.get_number_of_pages();
}

fn get_books() -> Vec<book::Book> {
   let mut books: Vec<book::Book> = Vec::with_capacity(7);

   books.push(book::Book::new("Harry Potter and the Philosopher's Stone".to_owned(), 223));
   books.push(book::Book::new("Harry Potter and the Chamber of Secrets".to_owned(), 251));
   books.push(book::Book::new("Harry Potter and the Prisoner of Azkaban".to_owned(), 317));
   books.push(book::Book::new("Harry Potter and the Goblet of Fire".to_owned(), 636));
   books.push(book::Book::new("Harry Potter and the Order of the Phoenix".to_owned(), 766));
   books.push(book::Book::new("Harry Potter and the Half-Blood Prince".to_owned(), 607));
   books.push(book::Book::new("Harry Potter and the Deathly Hallows".to_owned(), 607));

   return books;
}
