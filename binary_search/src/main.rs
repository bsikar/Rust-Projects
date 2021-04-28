fn main() {
    let ar = [-51, -11, 3, 13, 17, 19, 22, 29, 44, 57, 62, 78];

    binary_search(&ar, -441);
    binary_search(&ar, -1);
    binary_search(&ar, 15);
    binary_search(&ar, 34);
    binary_search(&ar, 45);
    binary_search(&ar, 62);
    binary_search(&ar, 75);
    println!(
        "{}",
        std::iter::successors(Some(5480u16), |&x| if x > 1 {
            x.checked_div(2)
        } else {
            None
        })
        .count()
    );
}

fn binary_search(ar: &[isize], target: isize) {
    let mut min = 0;
    let mut max = ar.len() - 1;
    let mut x = false;
    let mut i = 0;

    //  println!("low mid high");
    while min <= max {
        i += 1;
        let mid = min + (max - min) / 2;
        //        println!("{}   {}   {}", min, mid, max);
        if ar[mid] == target {
            x = true;
            break;
        } else if ar[mid] < target {
            min = mid + 1;
        } else if mid == 0 {
            break;
        } else {
            max = mid - 1;
        }
    }
    println!("{}, {}", i, if x { "found" } else { "not found" });
}
