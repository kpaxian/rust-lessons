pub fn init() {
    println!("generics");
    let number_list = vec![34, 50, 25, 100, 65];
    largest(&number_list);
    println!("Largest number is {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("Largest char is {}", largest(&char_list));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item     > largest {
            largest = item;
        }
    }
    largest
}