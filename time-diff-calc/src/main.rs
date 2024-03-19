/* Made by Sam Stockstrom */
/* a simple time difference calculator for personal use */
/* 60 / min | 60 min / hr | 24 hr / day */

use text_io::read; // for user input
use regex::Regex;
fn main() 
{
    println!("start time (hh:mm::ss am/pm) 12 hour clock: ");
    let start_time_input: String = read!("{}\n");
    analyze(start_time_input);
    println!("end time (hh:mm:ss am/pm) 12 hour clock: ");
    let end_time_input: String = read!("{}\n");
    analyze(end_time_input);
}

fn analyze(user_input: String)
{
    /* verify user input */
    let re = Regex::new(r"[0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]{1,3})?\s[A-Za-z]+").unwrap();
    if !re.is_match(&user_input)
    {
        println!("incorrect format!");
        return;
    }

    let times: Vec<&str> = user_input.split(':').collect();
    let hours: u8 = times[0].parse().unwrap();
    let minutes: u8 = times[1].parse().unwrap();
    let seconds: u8;
    let end = times[2];
    let meridiem: &str;

    if end.contains(" pm")
    {
        let formatted_end = end.replace(" pm", "");
        seconds = formatted_end.parse().unwrap();
        meridiem = "pm";
    }
    else if end.contains(" am")
    {
        let formatted_end = end.replace(" am", "");
        seconds = formatted_end.parse().unwrap();
        meridiem = "am";
    }
    else
    {
        println!("incorrect format!");
        return;
    }

    if hours > 12 || minutes > 59 || seconds > 59
    {
        println!("incorrect time!");
        return;
    }

    /* convert 12 hr to 24 hr */
}