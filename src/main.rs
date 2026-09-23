mod report;
mod supplies;

fn main() {
    println!("Oregon Trail Check");

    let party: u32 = 4;
    let days: u32 = 30;
    let rations_per_day: u32 = 3;

    let food = supplies::food_needed(party, days, rations_per_day);

    report::print_report(party, days, food);
}
