fn main() {
    println!("Please enter the number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input
        .trim()
        .parse::<i32>()
        .expect("the number must be bigger than 2");
    if n < 3 {
        eprintln!("the number must be bigger than 2");
        return;
    }
    let mut square = Vec::new();
    if n % 2 == 1 {
        square = odd_square(n);
    } else if n % 4 == 0 {
        square = d_even_square(n);
    } else {
        square = s_even_square(n);
    }
    let dec = number_of_dec(&square[square.len() / 2 as usize][square.len() / 2 as usize]);
    show_square(&square, dec);
}

fn odd_square(n: i32) -> Vec<Vec<i32>> {
    let mut sq: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
    let mut i = 0;
    let mut j = n / 2;
    sq[i as usize][j as usize] = 1;

    (2..=n * n).for_each(|k| {
        let row = if i - 1 < 0 { n - 1 } else { i - 1 };
        let col = if j - 1 < 0 { n - 1 } else { j - 1 };
        if sq[row as usize][col as usize] == 0 {
            i = row;
            j = col;
        } else {
            i = (i + 1) % n;
        }
        sq[i as usize][j as usize] = k;
    });
    sq
}

fn d_even_square(n: i32) -> Vec<Vec<i32>> {
    todo!()
}

fn s_even_square(n: i32) -> Vec<Vec<i32>> {
    todo!()
}

fn show_square(sq: &Vec<Vec<i32>>, dec: usize) {
    (0..sq.len()).for_each(|i| {
        (0..sq.len()).for_each(|j| {
            print!("{}", sq[i][j]);
            (0..=(dec - number_of_dec(&sq[i][j]))).for_each(|_| {
                print!(" ");
            })
        });
        println!("\n");
    })
}

fn number_of_dec(input: &i32) -> usize {
    let mut num = input.clone();
    let mut out = 0;
    while num != 0 {
        num /= 10;
        out += 1;
    }
    out as usize
}
