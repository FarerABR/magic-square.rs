use std::vec;

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

    #[allow(unused_assignments)]
    let mut square = Vec::new();
    if n % 2 == 1 {
        square = odd_square(n);
    } else if n % 4 == 0 {
        square = d_even_square(n);
    } else {
        square = s_even_square(n);
    }
    let dec = number_of_dec(&(n * n));
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
    let mut out: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
    (0..n).for_each(|i| {
        (0..n).for_each(|j| {
            out[i as usize][j as usize] = (n * i) + j + 1;
        })
    });

    (0..n / 4).for_each(|i| {
        (0..n / 4).for_each(|j| {
            out[i as usize][j as usize] = n.pow(2) + 1 - out[i as usize][j as usize];
        })
    });

    (0..n / 4).for_each(|i| {
        (3 * n / 4..n).for_each(|j| {
            out[i as usize][j as usize] = n.pow(2) + 1 - out[i as usize][j as usize];
        })
    });

    (3 * n / 4..n).for_each(|i| {
        (0..n / 4).for_each(|j| {
            out[i as usize][j as usize] = n.pow(2) + 1 - out[i as usize][j as usize];
        })
    });

    (3 * n / 4..n).for_each(|i| {
        (3 * n / 4..n).for_each(|j| {
            out[i as usize][j as usize] = n.pow(2) + 1 - out[i as usize][j as usize];
        })
    });

    (n / 4..3 * n / 4).for_each(|i| {
        (n / 4..3 * n / 4).for_each(|j| {
            out[i as usize][j as usize] = n.pow(2) + 1 - out[i as usize][j as usize];
        })
    });
    out
    // todo!()
}

fn s_even_square(n: i32) -> Vec<Vec<i32>> {
    todo!()
}

fn show_square(sq: &Vec<Vec<i32>>, dec: i32) {
    (0..sq.len()).for_each(|i| {
        (0..sq.len()).for_each(|j| {
            print!("{}", sq[i][j]);
            (0..=(dec as i32 - number_of_dec(&sq[i][j])).abs()).for_each(|_| {
                print!(" ");
            })
        });
        println!("\n");
    })
}

fn number_of_dec(input: &i32) -> i32 {
    let mut num = input.clone();
    let mut out = 0;
    while num != 0 {
        num /= 10;
        out += 1;
    }
    out
}
