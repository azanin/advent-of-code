fn main() {
    let report = vec![1721, 979, 366, 299, 675, 1456];


    let duplicated: Vec<i32> = report.iter().flat_map(|&a| {
        report.iter().flat_map(move |&b| if a + b == 2020 { vec![a * b] } else { Vec::new() })
    }).collect();


    println!("{:?}", duplicated);
}
