fn is_prime(i: u32) -> bool {
    if i <= 1 {
        return false;
    }
    if i <= 3{
        return true;
    }
    let sqrt_root = (i as f32).sqrt() as u32;
    for divder in 2..=sqrt_root {
        if i % divder == 0 {
            return false;
        }
    }
    true
}

pub fn goldbach_conjecture() -> String {
    let mut r = String::new();
    let mut primes: Vec<u32> = vec![2,3,5,7];
    let epsilon = 1e-6;
    let mut prime_set = std::collections::HashSet::new();
    prime_set.insert(2_u32);
    prime_set.insert(3_u32);
    prime_set.insert(5_u32);
    prime_set.insert(7_u32);
    
    // start testing from 4
    let mut num = 9_u32;
    let mut find_nums = 0;
    while find_nums != 2 {
        
        if is_prime(num) {
            if !prime_set.contains(&num) {
                prime_set.insert(num);
                primes.push(num);
            }
            num += 2;
            continue;
        }

        // 奇合数
        let mut find = true;
        for i in primes.iter().rev() {
            let remain = num - *i;
            if remain % 2 != 0 {
                continue
            }
            // 继续判断
            let remain_sqrt = ((remain / 2) as f64).sqrt();
            if remain_sqrt.fract().abs() < epsilon {
                find = false;
                break;
            }
        }
        if find {
            r.push_str(num.to_string().as_str());
            if find_nums == 0 {  
                r.push(',');
            }
            find_nums += 1;
        }
        num += 2;
    }
    r    
}
