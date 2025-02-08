pub fn new_count_distinct(input_str: &str) -> usize {
    let v: Vec<&str> = input_str.split(',').collect();
    let mut hash_set = std::collections::HashSet::new();

    for i in v.iter() {
        let s = String::from(*i);
        hash_set.insert(s);
    }
    hash_set.len()
}
