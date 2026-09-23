pub fn print_report(party: u32, days: u32, food: u32) {
    println!("===== SUPPLY REPORT =====");
    println!("Party size: {}", party);
    println!("Days:       {}", days);
    println!("Food:       {} lb", food);
    println!("=========================");
print_summary(party, days);
}

pub fn print_summary(party: u32, days: u32) {

    let food = crate::supplies::food_needed(party, days);

    println!("Summary: {} people need {} lb of food for {} days", party, food, days);

}
