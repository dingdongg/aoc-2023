use std::time::Instant;

mod services;

fn main() {
    println!("Hello, world!");
    // services::day_one::get_calibration_sum();
    // services::day_two::sum_min_set_powers();
    // services::day_three::sum_part_numbers();
    // services::day_three::sum_gear_ratios();
    // services::day_four::sum_card_points();
    // services::day_four::count_all_scratch_cards();
    // services::day_five::get_lowest_location();
    // services::day_five::get_lowest_location_from_ranges();
    // services::day_six::count_record_beating_strats();
    services::day_six::solve();
}
