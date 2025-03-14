#![feature(random)]

use scanf::scanf;

use std::{io::Write, mem::uninitialized, random::random};

const FU: [u32; 3] = [
    20, 30, 40
];

fn round_up_100(num: u32) -> u32 {
    if (num % 100) != 0 {
        num + 100 - (num % 100)
    } else {
        num
    }
}

fn is_hand_value_valid(han: u32, fu: u32, tsumo: bool) -> bool {
    if tsumo {
        if han == 1 && fu == 20 {
            false
        } else {
            true
        }
    } else {
        if fu == 20 {
            false
        } else {
            true
        }
    }
}


fn main() {

    let mut success: u64 = 0;
    let mut total: u64 = 0;

    print!("\x1B[2J");

    loop {
        let tsumo = random::<bool>();

        let han = (random::<u32>() % 4) + 1;
        let fu = FU[(random::<usize>() % 2) + if tsumo { 0 } else { 1 }];

        if !is_hand_value_valid(han, fu, tsumo) {
            continue;
        }

        println!("[{}] how much is worth {} han {} fu", if tsumo { "TSUMO" } else { "RON" }, han, fu);

        let mut number_oya: u32 = 0;
        let mut number_ko: u32 = 0;

        // base
        let mut base_score: u32 = fu * (1 << (2 + han));
        // cap to mangan
        base_score = base_score.min(2000);

        let payout_ron = round_up_100(base_score * 4);
        let payout_ko = round_up_100(base_score);
        let payout_oya = round_up_100(base_score * 2);

        if tsumo {
            print!("子: ");
            scanf!("{}", number_ko);
            print!("親: ");
            scanf!("{}", number_oya);
        } else {
            scanf!("{}", number_oya);
            // does not matter for ron
            number_ko = payout_ko;
        }

        print!("\x1B[2J");
        if number_oya == payout_oya && number_ko == payout_ko {
            success += 1;
            total += 1;
            println!("Correct! {}/{} {:.2}%", success, total, (success as f64 / total as f64) * 100.0);
        } else {
            total += 1;
            if tsumo {
                println!("Correct answer was: {}/{}. {}/{} {:.2}%", payout_ko, payout_oya, success, total, (success as f64 / total as f64) * 100.0);
            } else {
                println!("Correct answer was: {}. {}/{} {:.2}%", payout_oya, success, total, (success as f64 / total as f64) * 100.0);
            }
        }
    }

}
