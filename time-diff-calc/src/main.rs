/* Made by Sam Stockstrom */
/* a simple time difference calculator for personal use */

use text_io::read; // for user input
use regex::Regex;
fn main() 
{
    // test();
    println!();
    println!("[accepted format(s)]");
    println!("12 HOUR CLOCK [hh:mm::ss am/pm]");
    println!("12 HOUR CLOCK [h:mm:ss am/pm]\n");
    println!("start time: ");
    let start_time_input: String = read!("{}\n");
    println!();
    println!("end time: ");
    let end_time_input: String = read!("{}\n");
    println!();
    let result: Option<Vec<i32>> = analyze(start_time_input.trim().to_string(), end_time_input.trim().to_string());

    if result.is_some()
    {
        let unwrapped = result.unwrap();
        println!("\ntime difference:");
        println!("{} hours {} minutes {} and seconds", unwrapped[0], unwrapped[1], unwrapped[2]);
    }
    else
    {
        println!("[!] [error::main] returned empty result!");
    }

    let _input: String = read!("{}\n"); // wait for user input so it doesn't close
}

/* fn test()
{
    let test1: Option<Vec<i32>> = analyze("1:37:00 pm".to_string(), "2:37:00 pm".to_string());
    if test1.is_some()
    {
        let test1_unwrapped = test1.unwrap();
        if test1_unwrapped[0] == 1 && test1_unwrapped[1] == 0 && test1_unwrapped[2] == 0
        {
            println!("[0] Test 1 passed!");
        }
        else
        {
            println!("[!] Test 1 failed!");
        }
    }
    else
    {
        println!("[!] Test 1 failed!");
    }

    let test2: Option<Vec<i32>> = analyze("8:23:32 am".to_string(), "2:37:00 pm".to_string());
    if test2.is_some()
    {
        let test2_unwrapped = test2.unwrap();
        if test2_unwrapped[0] == 6 && test2_unwrapped[1] == 13 && test2_unwrapped[2] == 28
        {
            println!("[0] Test 2 passed!");
        }
        else
        {
            println!("[!] Test 2 failed!");
        }
    }
    else
    {
        println!("[!] Test 2 failed!");
    }

    let test3: Option<Vec<i32>> = analyze("11:37:00 am".to_string(), "3:37:00 pm".to_string());
    if test3.is_some()
    {
        let test3_unwrapped = test3.unwrap();
        if test3_unwrapped[0] == 4 && test3_unwrapped[1] == 0 && test3_unwrapped[2] == 0
        {
            println!("[0] Test 3 passed!");
        }
        else
        {
            println!("[!] Test 3 failed!");
        }
    }
    else
    {
        println!("[!] Test 3 failed!");
    }
} */

fn analyze(start_user_input: String, end_user_input: String) -> Option<Vec<i32>>
{
    /* verify user input */
    let re = Regex::new(r"[0-9]?[0-9]:[0-9]{2}:[0-9]{2}(\.[0-9]{1,3})?\s[A-Za-z]+").unwrap();
    if !re.is_match(&start_user_input) || !re.is_match(&end_user_input)
    {
        println!("[!] [error::regex] incorrect format!");
        return None;
    }

    let times: Vec<Vec<&str>> = vec![start_user_input.split(':').collect(), end_user_input.split(':').collect()];
    
    let mut start_h: i32 = times[0][0].parse().unwrap();
    let start_m: i32 = times[0][1].parse().unwrap();
    let start_s: i32;
    let start_e: &str = times[0][2];

    let mut end_h: i32 = times[1][0].parse().unwrap();
    let end_m: i32 = times[1][1].parse().unwrap();
    let end_s: i32;
    let end_e: &str = times[1][2];

    // println!("[0] [debug::start] given 12 hr = {}", start_h);
    // println!("[0] [debug::end] given 12 hr = {}", end_h);

    if start_e.contains(" pm")
    {
        // println!("[0] [debug::start] PM");
        let mut formatted_end = start_e.replace(" pm", "");
        if formatted_end.starts_with("0") { formatted_end.remove(0); }
        start_s = formatted_end.parse().unwrap();

        if start_h >= 1 && start_h <= 11 // 1pm to 11pm
        {
            start_h += 12;
        }
    }
    else if start_e.contains(" am")
    {
        // println!("[0] [debug::start] AM");
        let mut formatted_end = start_e.replace(" am", "");
        if formatted_end.starts_with("0") { formatted_end.remove(0); }
        start_s = formatted_end.parse().unwrap();
        
        if start_h == 12
        {
            start_h = 0;
        }

    }
    else
    {
        println!("[!] [error::logic] incorrect format!");
        return None;
    }

    if end_e.contains(" pm")
    {
        // println!("[0] [debug::end] PM");
        let mut formatted_end = end_e.replace(" pm", "");
        if formatted_end.starts_with("0") { formatted_end.remove(0); }
        end_s = formatted_end.parse().unwrap();

        if end_h >= 1 && end_h <= 11 // 1pm to 11pm
        {
            end_h += 12;
        }
    }
    else if end_e.contains(" am")
    {
        // println!("[0] [debug::end] AM");
        let mut formatted_end = end_e.replace(" am", "");
        if formatted_end.starts_with("0") { formatted_end.remove(0); }
        end_s = formatted_end.parse().unwrap();
        
        if end_h == 12
        {
            end_h = 0;
        }

    }
    else
    {
        println!("[!] [error::logic] incorrect format!");
        return None;
    }

    if start_h > 24 || start_m > 59 || start_s > 59
    {
        println!("[!] [error::time::start] incorrect time!");
        return None;
    }

    if end_h > 24 || end_m > 59 || end_s > 59
    {
        println!("[!] [error::time::end] incorrect time!");
        return None;
    }
    
    let mut calc_h: i32 = end_h - start_h;
    let mut calc_m: i32 = end_m - start_m;
    let mut calc_s: i32 = end_s - start_s;

    if calc_s < 0 { calc_m -= 1; calc_s = 60 - calc_s.abs(); }
    if calc_m < 0 { calc_h -= 1; calc_m = 60 - calc_m.abs(); }

    let correct: Vec<i32> = vec![calc_h,calc_m,calc_s];
    // println!("[0] [debug::logic] diff = {:?}", correct);
    return Some(correct);

}