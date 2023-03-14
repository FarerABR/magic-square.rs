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
        square = doubly_even_square(n);
    } else {
        square = singly_even_square(n);
    }

    check_magic(&square);
    show_square(&square, number_of_dec(&(n * n)));
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

fn doubly_even_square(n: i32) -> Vec<Vec<i32>> {
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
}

fn singly_even_square(n: i32) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

    // Top left
    fill_quarter(&mut out, 0, n / 2, 0, n / 2, 1, (n / 2) * (n / 2));

    // Bottom right
    fill_quarter(
        &mut out,
        n / 2,
        n,
        n / 2,
        n,
        (n / 2) * (n / 2) + 1,
        n * n / 2,
    );

    // Top right
    fill_quarter(
        &mut out,
        0,
        n / 2,
        n / 2,
        n,
        n * n / 2 + 1,
        3 * (n / 2) * (n / 2),
    );

    // Bottom left
    fill_quarter(
        &mut out,
        n / 2,
        n,
        0,
        n / 2,
        3 * (n / 2) * (n / 2) + 1,
        n * n,
    );

    let shift = (n / 2 - 1) / 2;
    for i in 0..n / 2 {
        for j in 0..shift {
            if i == n / 4 {
                exchange_cell(i, j + shift, &mut out);
            } else {
                exchange_cell(i, j, &mut out);
            }
        }
    }
    for i in 0..n / 2 {
        for j in (n - shift)..(n - 1) {
            exchange_cell(i, j, &mut out);
        }
    }
    out
}

fn fill_quarter(
    sq: &mut Vec<Vec<i32>>,
    f_row: i32,
    l_row: i32,
    f_col: i32,
    l_col: i32,
    f_num: i32,
    l_num: i32,
) {
    let mut num = f_num;
    let mut i = f_row;
    let mut j = (l_col + f_col) / 2;

    while num <= l_num {
        if i < f_row && j >= l_col {
            i += 2;
            j -= 1;
        };
        if j >= l_col {
            j = f_col;
        }
        if i < f_row {
            i = l_row - 1;
        }
        if sq[i as usize][j as usize] != 0 {
            i += 2;
            j -= 1;
            continue;
        } else {
            sq[i as usize][j as usize] = num;
            num += 1;
        }
        i -= 1;
        j += 1;
    }
}
fn exchange_cell(i: i32, j: i32, sq: &mut Vec<Vec<i32>>) {
    let k = sq.len() / 2 + i as usize;
    let tmp = sq[i as usize][j as usize];
    sq[i as usize][j as usize] = sq[k][j as usize];
    sq[k][j as usize] = tmp;
}

fn show_square(sq: &Vec<Vec<i32>>, dec: i32) {
    (0..sq.len()).for_each(|i| {
        (0..sq.len()).for_each(|j| {
            print!("{}  ", sq[i][j]);
            (0..=(dec as i32 - number_of_dec(&sq[i][j])).abs()).for_each(|_| {
                print!(" ");
            })
        });
        println!("\n");
    })
}

fn check_magic(sq: &Vec<Vec<i32>>) {
    if sq.iter().fold(0, |acc, i| acc + i[0]) != sq.iter().fold(0, |acc, i| acc + i[1])
        || sq.iter().fold(0, |acc, i| acc + i[sq.len() - 1])
            != sq.iter().fold(0, |acc, i| acc + i[sq.len() - 2])
    {
        eprintln!("THE SQUARE IS NOT MAGIC");
    }
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
