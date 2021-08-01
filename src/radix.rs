pub fn sort(arr : &mut [u64]) {
    let mut position : u64 = 1;
    let mut first : Vec<u64> = Vec::new();
    let mut second : Vec<u64> = Vec::new();
    for i in 0..63 {
        if i == 0 {
            let size = arr.len();
            for x in 0..size {
                let current = arr[x];
                let list = current & position;
                if list == 0 {
                    first.push(current);
                } else {
                    second.push(current);
                }
            }
        } else {
            let mut primary : Vec<u64> = Vec::new();
            let mut secondary : Vec<u64> = Vec::new();
            let first_size = first.len();
            let second_size = second.len();
            for x in 0..first_size {
                let current = first[x];
                let list = current & position;
                if list == 0 {
                    primary.push(current);
                } else {
                    secondary.push(current);
                }
            }
            for x in 0..second_size {
                let current = second[x];
                let list = current & position;
                if list == 0 {
                    primary.push(current);
                } else {
                    secondary.push(current);
                }
            }
            first = primary;
            second = secondary;
        }
        position = position << 1;
    }
    let first_size = first.len();
    let second_size = second.len();
    for x in 0..first_size {
        arr[x] = first[x];
    }
    for x in first_size..second_size {
        arr[x] = second[x];
    }
}