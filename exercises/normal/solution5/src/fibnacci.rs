pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut v = vec![0;10];
    let mut r = 0;
    
    let mut index  = 0;
    
    loop {
        if r >= threshold {
            break;
        }
        let mut current_fib = 0;
        index = index + 1;
        if index == 1 {
            *v.get_mut(index).unwrap() = 1;
            r = 1;
            current_fib = 1;
        } else {
            current_fib = *v.get(index - 1).unwrap()
                          +
                          *v.get(index - 2).unwrap();
            if current_fib % 2 == 1 {
                r += current_fib;
            }
            match v.get_mut(index) {
                Some(index_ref) => {
                    *index_ref = current_fib;
                },
                None => {
                    v.push(current_fib);
                }
            }
        } 
        
    }
    r
}
