pub fn bubble_sort<T>(mut collection: Vec<T>, comparer: &dyn Fn(&T, &T) -> i32) -> Vec<T> {
   let mut is_sorted = false;
   let mut end_index = collection.len() - 1;

   while !is_sorted {
      is_sorted = true;

      for i in 0..end_index {
         if comparer(&collection[i], &collection[i + 1]) > 0 {
            collection.swap(i, i + 1);
            is_sorted = false;
         }
      }

      end_index -= 1;
   }

   return collection;
}
