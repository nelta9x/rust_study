use std::io;
use rand::Rng;

fn main() {
    println!("baseball game!");

    loop {
        let secret = gen_numbers();

        loop {
            println!("please input three digits like \"147\" (digit range: 0~9)");
    
            let guess = input_numbers();
    
            let mut strikes = 0;
            let mut balls = 0;
            let mut outs = 0;
    
            calculate_score(secret, guess, &mut strikes, &mut balls, &mut outs);
    
            println!("strikes: {strikes}, balls: {balls}, outs: {outs}");
    
            if strikes == 3 {
                println!("three strikes!");
                break;
            }
        }
    }
}

fn gen_numbers() -> [i32; 3] {
    // 중복되지 않기 위해 값을 생성하고 섞기만 한다.
    let mut cards = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in 0..cards.len() {
        let target_idx = rand::thread_rng().gen_range(0..cards.len());
        cards[i] = cards[target_idx];
    }

    [cards[0], cards[1], cards[2]]
}

fn input_numbers() -> [i32; 3] {
    let mut arr: [i32; 3] = [0, 0, 0];
    let mut str: String = String::new();

    io::stdin().read_line(&mut str).expect("failed to read line.");

    for (i, c) in str.chars().enumerate() {
        if arr.len() <= i {
            break;
        }
        arr[i] = convert_char_to_digit(c);
    }

    arr
}

fn calculate_score(secret: [i32; 3], guess: [i32; 3], strikes: &mut i32, balls: &mut i32, outs: &mut i32) {
    let mut num_strikes: i32 = 0;
    let mut num_balls: i32 = 0;

    for i in 0..secret.len() {
        if secret[i] == guess[i] {
            num_strikes += 1;
        }
    }

    for i in 0..secret.len() {
        for j in 0..guess.len() {
            if i != j {
                if secret[i] == guess[j] {
                    num_balls += 1;
                    break;
                }
            }
        }
    }

    *strikes = num_strikes;
    *balls = num_balls;
    *outs = secret.len() as i32 - num_strikes - num_balls;
}

fn convert_char_to_digit(c: char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0
    }
}