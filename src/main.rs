use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashSet};

fn main() {

    let mut prods = BTreeMap::<BTreeSet<i32>, BTreeSet<i32>>::new();

//  A convenient way to permutate all values of ABCD from the ten digits; this will result in duplicates but they are eliminated
//  quickly by collecting totals into one set per ABCD
    for s in (0..10).into_iter().permutations(4) {
        let s_arr = s.clone().into_iter().collect::<BTreeSet<i32>>();
        let p = (10 * s[0] + s[1]) * (10 * s[2] + s[3]);
        prods.entry(s_arr).or_insert(BTreeSet::<i32>::new()).insert(p);
    }

//  For each ABCD we then calculate the number of distinct totals (n) and the digits implied by the requirement that n = AB we have found
            if num_sets != 0 && num_sets == b {
//  ... calculate c as the count of remaining 4-digit products
                let four_digs = s.1.iter().filter(|&&p| p > 999).collect::<Vec<&i32>>();
                let c = four_digs.len() - num_sets;
//  whence D                
                let mut used = s.0.clone();
                used.remove(&(a as i32));
                used.remove(&(b as i32));
                used.remove(&(c as i32));
                println!("A:{} B:{} C:{} D:{:?}", a, b, c, used.first().unwrap());
            }
        }
    }
}
