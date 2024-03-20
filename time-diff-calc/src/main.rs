/* Made by Sam Stockstrom */
/* a simple time difference calculator for personal use */
/* 60 / min | 60 min / hr | 24 hr / day */

use text_io::read; // for user input
use regex::Regex;
fn main() 
{
    test();
    // user input
    //println!("start time (hh:mm::ss am/pm) 12 hour clock: ");
    //let start_time_input: String = read!("{}\n");
    //let start_time = analyze(start_time_input.trim().to_string(), 0);
    
    //println!("end time (hh:mm:ss am/pm) 12 hour clock: ");
    // let end_time_input: String = read!("{}\n");
    //let difference: i32 = analyze(end_time_input.trim().to_string(), start_time);
    //println!("Time Difference: {} hours", difference)
}

fn test()
{
    let test1: Option<Vec<i32>> = analyze("01:37:00 pm".to_string(), "02:37:00 pm".to_string());
    if test1.is_some() && test1.unwrap()[0] == 1
    {
        println!("[0] Test 1 passed!");
    }
    else
    {
        println!("[!] Test 1 failed!");
    }

    let test2: Option<Vec<i32>> = analyze("08:23:32 am".to_string(), "02:37:00 pm".to_string());
    if test2.is_some() && test2.unwrap()[0] == 6
    {
        println!("[0] Test 2 passed!");
    }
    else
    {
        println!("[!] Test 2 failed!");
    }

    let test3: Option<Vec<i32>> = analyze("11:37:00 am".to_string(), "03:37:00 pm".to_string());
    if test3.is_some() && test3.unwrap()[0] == 4
    {
        println!("[0] Test 3 passed!");
    }
    else
    {
        println!("[!] Test 3 failed!");
    }
}

fn analyze(start_user_input: String, end_user_input: String) -> Option<Vec<i32>>
{
    /* verify user input */
    let re = Regex::new(r"[0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]{1,3})?\s[A-Za-z]+").unwrap();
    if !re.is_match(&start_user_input) || !re.is_match(&end_user_input)
    {
        println!("[error::regex] incorrect format!");
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
    let end_e: &str = times[0][2];

    if start_e.contains(" pm")
    {
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
        println!("[error::logic] incorrect format!");
        return None;
    }

    if end_e.contains(" pm")
    {
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
        println!("[error::logic] incorrect format!");
        return None;
    }

    if start_h > 24 || start_m > 59 || start_s > 59
    {
        println!("[error::time::start] incorrect time!");
        return None;
    }

    if end_h > 24 || end_m > 59 || end_s > 59
    {
        println!("[error::time::end] incorrect time!");
        return None;
    }
    

    let correct: Vec<i32> = vec![(start_h - end_h).abs(),0,0];
    return Some(correct);

}