fn rgb(r: i32, g: i32, b: i32) -> String {
    let vec = vec![r,g,b];
    vec.iter()
    .map(|mut n| {
        if n < &0 {
            n = &0
        };
        if n > &255 {
            n = &255
        }
        format!("{:02X}", n).to_string()
    })
    .collect()
}
