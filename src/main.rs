use std::error::Error;
use std::fs::File;

fn check_error(mut row: Vec<i32>) -> Vec<i32> {
    fn dot_product(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        v1.iter()
            .zip(v2.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    let h = vec![
        vec![1,1,0,0,1,1,0,0,0],
        vec![1,1,1,0,0,0,1,0,0],
        vec![1,0,1,1,0,0,0,1,0],
        vec![1,0,0,1,1,0,0,0,1],
    ];

    let err_check: Vec<i32> = h.iter()
        .map(|h_row| dot_product(&row, h_row) % 2)
        .collect();

    if err_check.iter().all(|&x| x == 0) { println!("{:?}", row); return row; }
    else {
        for col in 0..row.len() {
            let column: Vec<i32> = h.iter()
                .map(|r| r[col])
                .collect();

            if column == err_check {
                print!("{:?}, ERROR in bit {}", row, col);
                row[col] ^= 1;
                println!(" Corrected row: {:?}", row);
                return row;
            }
            
        }
    }

    println!("{:?}, ERROR, FAILED TO PARSE ERROR BIT", row);
    return row;
}

fn decode_msg(row: Vec<i32>) -> char {
    let mut value = 0;

    for &bit in &row[..5] {
        value = (value << 1) | bit;
    }
    
    let ch = (b'a' + value as u8 - 1) as char;

    if ch == '`' { return ' ' }
    else { return ch }
}


fn main() -> Result<(), Box<dyn Error>> {
    let message = File::open("src/messageFall2025.csv")?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(message);
    let mut message_string = String::new();

    print!("\n\n");
    for result in reader.records() {
        let record = result?;
        let row: Vec<i32> = record
            .iter()
            .map(|field| field.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;
        message_string.push(decode_msg(check_error(row)));
    }
    println!("\n{}", message_string);
    println!("\nMessage completed\n");
    Ok(())
}
