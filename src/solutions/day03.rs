fn parse_input(data: &str) -> Vec<Vec<usize>> {
    let mut banks = vec![];
    for line in data.lines() {
        // error here fix tomorrow
        let bank: Vec<usize> = line.chars().map(|s| s.to_digit(10).unwrap() as usize).collect();
        banks.push(bank);
    }

    banks
}
// idea is, from left, find largest number UNTIL last item. Keep track of the index of the largest number
// then, from right, find largest number UNTIL the aforementioned index
fn select_max_val(batteries: Vec<usize>) -> (usize, usize) {
    let mut first = 0;
    let mut l_index = 0;
    let mut second = 0;
    let mut idx = 0;

    // two loops, check for tenth position first
    while idx < batteries.len() - 1 {
        if batteries[idx] > first {
            first = batteries[idx];
            l_index = idx;
        }

        idx += 1;
    }

    while idx > l_index {
        if batteries[idx] > second {
            second = batteries[idx];
        }

        idx -= 1;
    }

    (first, second)
}

pub fn solve_part_one(data: &str) -> usize {
    // for tomorrow: find the highest number; pluck it out (teens). repeat for second highest
    let banks = parse_input(data);
    let mut count = 0;
    for bank in banks {
        let (first, second) = select_max_val(bank);

        let mut b = String::new();
        b.push_str(&first.to_string());
        b.push_str(&second.to_string());

        let new_number: usize = b.parse().unwrap();
        count += new_number;
    }
    count
}

fn select_max_val_p2(batteries: Vec<usize>) -> usize {
    let mut idx: usize = 0;
    let mut spares = batteries.len() - 12;
    let mut buff = String::new();

    while buff.len() < 12 {
        let range = idx..(idx + spares + 1);
        let window = &batteries[range];
        let max_val = window.iter().max().unwrap();
        let (i, n) = window
            .iter()
            .enumerate()
            .find(|n| n.1 == max_val).unwrap();
        buff.push_str(&n.to_string());

        if i > 0 {
            if spares != 0 {
                spares -= i;
            }
            idx += i + 1;
        } else {
            idx += 1;
        }
    }



    usize::from_str_radix(&buff, 10).unwrap()
}


pub fn solve_part_two(data: &str) -> usize {
    // going to be similar to part one, 
    let mut count = 0;
    let banks = parse_input(data);
    for bank in banks {
        let line: Vec<String> = bank.iter().map(|x| x.to_string()).collect();
        let b = line.join("");
        let max_val = select_max_val_p2(bank);
        count += max_val;
    }

    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve_part_one(data), 357);
    }
    // [818] 181911112111
    // 8 [181] 1911112111
    // 88 [18] 1911112111
    // 888911112111
    // 888911112111 
    // [8181]81911112111 -> 8
    // 8[1818]1911112111 -> 888
    // 888[191]1112111 -> 8889
    // 8889[11]12111 -> 88891
    // 88891[11]2111 -> 888911
    // 888911[12]111 -> 888912
    // 
    //  xx4[234]234234278 (1 space left)
    // 4334234
    #[test]
    fn test_solve_part_two() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve_part_two(data), 3121910778619);
    }
}