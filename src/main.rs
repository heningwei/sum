fn main() {
    let arr: [u32;4] = [1,2,3,4];
    println!("sum is {:?}", sum(&arr));

    let arr: [u32;4] = [4000000000,4000000000,4000000000,4000000000];
    println!("sum is {:?}", sum(&arr));
}

fn sum(arr: &[u32]) -> Option<u32>{
    let mut result: u32 = 0;
    let mut tag: bool = false;
    for a in arr {
        (result, tag) = result.overflowing_add(*a);
        if tag == true {
            return None;
        }
    }
    Some(result)
}
