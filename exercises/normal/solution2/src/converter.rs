pub fn convert_base(num_str: &str, to_base: u32) -> String {
   let mut r = String::new();
   let num_v: Vec<&str> = num_str.split('(').collect();
   
   let from_base = num_v.get(1).unwrap();
   let mut from_base_i = 0;
   
   for i in from_base.chars().enumerate() {
        if i.1 == ')' {
            continue;
        }
        from_base_i = from_base_i * 10 + (i.1 as i32 - '0' as i32) ;
   } 

   let mut num_base = 0;
   for i in num_v.get(0).unwrap().chars().rev().enumerate() {
        num_base += (i.1 as i32 - '0' as i32) * from_base_i.pow(i.0 as u32);
   }

   if num_base == 0 {
        r.push_str("0");
        return r;
   }
   let mut s= std::collections::VecDeque::new();
   while num_base != 0 {
        let c = num_base % to_base as i32;
        if to_base == 16 && c > 9 {
            let c = match c {
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                15 => 'f',
                _ => panic!("unreachable"),
            };
            s.push_back(c);
        }else {
            let c = (c + '0' as i32) as u8 as char;
            s.push_back(c);
        }
        num_base = num_base / to_base as i32
   }
   
   while !s.is_empty() {
        r.push(s.pop_back().unwrap());
   }
   
   r
}
