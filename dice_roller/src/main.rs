extern crate rand;

use std::io;

fn main() {

    let mut dice_count = 0;
    let mut dice_size = 0;

    let mut count = true;
    let mut add = true;
    let mut total = 0;


    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("expected an input string");
    // let s = "3d8 + 1d6";

    let s = input.replace("\n", ""); // strip newline from end of input string

    for l in s.chars() {
        if l == 'd' {
            count = false;
        }
        else if l == '+' {
            count = true;
            if dice_count > 0 {
                total = sum_rolls(total, roll_dice(dice_size, dice_count), add);
                dice_count = 0;
                dice_size = 0;
            }
            add = true;
        }
        else if l == '-' {
            count = true;
            if dice_count > 0 {
                total = sum_rolls(total, roll_dice(dice_size, dice_count), add);
                dice_count = 0;
                dice_size = 0;
            }
            add = false;
        }
        else if l != ' ' {
            if count {
                dice_count = dice_count * 10 + (l as u32 - 48);
            }
            else {
                dice_size = dice_size * 10 + (l as u32 - 48);
            }
        }
    }

    total = sum_rolls(total, roll_dice(dice_size, dice_count), add);

    println!("Total from {:?}: {:?}", s, total);
}

fn sum_rolls(arg1: i32, arg2: i32, add: bool) -> i32 {
    if add {
        arg1 + arg2
    }
    else {
        arg1 - arg2
    }
}

fn roll_dice(sides: u32, dice: u32) -> i32 {
    let mut x = 0;
    let mut total = 0;
    while x < dice {
        x = x + 1;
        total = total + roll_die(sides);
    }
    total
}

fn roll_die(sides: u32) -> i32 {
    // println!("rolling a {:?}-sided dice", sides);
    (rand::random::<u32>() % sides + 1) as i32
}
