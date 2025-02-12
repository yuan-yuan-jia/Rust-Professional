pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let currncies = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut r = 0;
    let mut remind = amount;
    
    while remind != 0 {
        let mut idx = (currncies.len() - 1) as i32;
        while idx >= 0  {
            if remind >= currncies[idx as usize] {
                let mut c = 1 as u32;
                let currency = currncies[idx as usize];
                while remind >= (c * currency) {
                    c += 1;
                }
                remind = remind - (c - 1) * currency;
                r = r + (c - 1);
                break;   
            }
            idx -= 1;
        }
    }
    r
}
