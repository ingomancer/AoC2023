pub fn run(input: String) -> (String, String) {
    let mut sum = 0;
    let mut sum2 = 0;
    for line in input.lines() {
        let nums = line.split(' ').map(|x| x.parse::<i32>().unwrap());
        let (first, last) = first_last_elem(nums);
        sum += last;
        sum2 += first;
    }
    (format!("{sum}"), format!("{sum2}"))
}

fn first_last_elem<'a, T>(mut nums: T) -> (i32, i32)
where
    T: Iterator<Item = i32>,
{
    let mut prev: i32 = nums.next().unwrap();
    let first = prev;
    let mut subseq = vec![];
    for num in nums {
        let num = num;
        subseq.push(num - prev);
        prev = num;
    }
    if subseq.iter().all(|x| x == &0) {
        return (first, prev);
    } else {
        let (next_first, next_prev) = first_last_elem(subseq.into_iter());
        return (first - next_first, prev + next_prev);
    }
}
