pub struct Book {
   title: String,
   number_of_pages: i32,
}

impl Book {
   pub fn new(title: String, number_of_pages: i32) -> Self {
      Self {
         title,
         number_of_pages,
      }
   }

   pub fn get_title(&self) -> &String {
      return &self.title;
   }

   pub fn get_number_of_pages(&self) -> i32 {
      return self.number_of_pages;
   }
}
