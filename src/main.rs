use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("number of args must be 2")
    }
    println!("{}", lev(&args[1], &args[2]));
}

static FAILURE: u32 = u32::MAX;

fn lev(left: &str, right: &str) -> u32 {
    let mut cache: HashMap<(usize, usize), u32> = HashMap::new();
    return lev_unicode_scalars(&left, &right, &mut cache);
}

fn lev_unicode_scalars(left: &str, right: &str, cache: &mut HashMap<(usize, usize), u32>) -> u32 {
    let l_len = left.len();
    let r_len = right.len();

    if let Some(res) = cache.get(&(l_len, r_len)) {
        return *res;
    }

    if left == right {
        return 0;
    }

    let mut l_tail: String = String::new();
    let mut r_tail: String = String::new();

    let (mut del, mut ins, mut rpl) = (FAILURE, FAILURE, FAILURE);

    if l_len > 0 {
        l_tail = left.chars().skip(1).collect::<String>();
        del = 1 + lev_unicode_scalars(&l_tail, right, cache)
    }

    if r_len > 0 {
        r_tail = right.chars().skip(1).collect::<String>();
        ins = 1 + lev_unicode_scalars(&left, &r_tail, cache)
    }

    if let (Some(l_head), Some(r_head)) = (left.chars().nth(0), right.chars().nth(0)) {
        rpl = (if l_head == r_head { 0 } else { 1 }) + lev_unicode_scalars(&l_tail, &r_tail, cache);
    }

    let min = *[del, ins, rpl].iter().min().unwrap();
    cache.insert((l_len, r_len), min);

    return min;
}
