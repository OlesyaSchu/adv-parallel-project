use rayon::prelude::*;
use diam::prelude::*; 


pub fn lemonade_change(bills: Vec<i32>) -> bool {
  const COST: i32 = 5;
  let mut change_sum: i32 = 0;
  let mut fives: i32 = 0;
  let mut tens: i32 = 0;
  let mut result: bool = true;
    for bill in bills {
        if bill - COST <= change_sum {
          let mut change_to_give: i32 = bill - COST;
          if change_to_give != 0 {
              let tens_to_give = (change_to_give / 10).min(tens);
              change_to_give -= tens_to_give * 10;
              tens -= tens_to_give;

              let fives_to_give = (change_to_give / 5).min(fives);
              change_to_give -= fives_to_give * 5;
              fives -= fives_to_give;
            if change_to_give != 0 {
              result = false;
              return result;
            }
          }
          match bill {
            5 => {
              fives += 1; 
              change_sum += COST;
            },
            10 => {
              tens += 1; 
              change_sum += COST;
            },
            _ => {}
          }
        } else {
            result = false;
            return result;
        }
    }
    result
}


pub fn lemonade_change_parallel(bills: Vec<i32>) -> bool {
    const COST: i32 = 5;

    let calc_chunk = |chunk: &[i32]| -> Option<(i32, i32)> {
        let mut fives = 0;
        let mut tens = 0;

        for &bill in chunk {
            let mut change_to_give = bill - COST;

            if change_to_give > 0 {
                let tens_to_give = (change_to_give / 10).min(tens);
                change_to_give -= tens_to_give * 10;
                tens -= tens_to_give;

                let fives_to_give = (change_to_give / 5).min(fives);
                change_to_give -= fives_to_give * 5;
                fives -= fives_to_give;

                if change_to_give != 0 {
                    return None;
                }
            }

            match bill {
                5 => fives += 1,
                10 => tens += 1,
                _ => {}
            }
        }

        Some((fives, tens))
    };

    bills
        .par_chunks(bills.len() / rayon::current_num_threads())
        .log("outer")
        .map(calc_chunk)
        .reduce(
            || Some((0, 0)),
            |acc, result| {
                match (acc, result) {
                    (Some((fives_acc, tens_acc)), Some((fives, tens))) => {
                        Some((fives_acc + fives, tens_acc + tens))
                    }
                    _ => None,
                }
            },
        )
        .is_some()
}

fn main() {
    let pattern = vec![5, 5, 5, 10, 20];
    let mut bills = Vec::new();
    bills.extend(pattern.iter().cloned().cycle().take(100_000_000 * pattern.len()));
    println!("Num threads: {}", rayon::current_num_threads());

    let mut start = std::time::Instant::now();
    let mut result = lemonade_change(bills.clone());
    println!("Sequential execution");
    println!("Exec time: {:?}", start.elapsed());
    println!("Result: {}", result);


    start = std::time::Instant::now();
    println!("Parallel execution");
    result = lemonade_change_parallel(bills.clone());
    // diam::svg("outer.svg", || lemonade_change_parallel(bills.clone())).unwrap();
    println!("Exec time: {:?}", start.elapsed());
    println!("Result: {}", result);
}