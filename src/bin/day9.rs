use advent2020::input;

fn main() {
    let input = input::read_all(9);
    let nums: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();

    println!("Part 1");
    let mut invalid = 0;
    for i in 25..nums.len() {
        let mut ok = false;
        'sum: for j in 0..25 {
            for k in 0..25 {
                if j == k { continue; }
                
                if nums[i-j-1] + nums[i-k-1] == nums[i] {
                    ok = true;
                    break 'sum;
                }
            }
        }

        if !ok {
            println!("not a sum {}", nums[i]);
            invalid = nums[i];
        }
    }

    println!("Part 2");
    let mut begin = 0;
    let mut sum = 0;
    for end in 0..nums.len() {

        sum += nums[end];

        while sum > invalid {
            sum -= nums[begin];
            begin += 1;
        }

        if sum == invalid && end > begin+1 {
            let range = &nums[begin..(end+1)];
            assert_eq!(range.iter().sum::<isize>(), invalid);
            let mn = range.iter().min().unwrap();
            let mx = range.iter().max().unwrap();
            println!("{},{} {}", begin, end, mn + mx);
            break;
        } 
    }
}

