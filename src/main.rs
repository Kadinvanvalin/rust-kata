fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod binary_search_list {
    fn binary_fn(haystack: &Vec<u32>, needle: u32) -> bool {
        let mut lo: i32 = 0;
        let mut hi: i32 = haystack.len() as i32 - 1;
        while lo <= hi {
            println!("lo: {} hi: {}", lo, hi);
            let mid = ((lo + (hi - lo)/2) as f64).floor() as i32;
                if haystack.get(mid as usize).unwrap() == &needle {
                    return true
                } else if haystack.get(mid as usize).unwrap() > &needle {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
        }
        return false;
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn binary_search_array() {
        let foo = vec![1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(binary_fn(&foo, 69), true);
        assert_eq!(binary_fn(&foo, 1336), false);
        assert_eq!(binary_fn(&foo, 69420), true);
        assert_eq!(binary_fn(&foo, 69421), false);
        assert_eq!(binary_fn(&foo, 1), true);
        assert_eq!(binary_fn(&foo, 0), false);
    }
}
