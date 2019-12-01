// --- Day 1: The Tyranny of the Rocket Equation ---
//
// https://adventofcode.com/2019/day/1

fn calc_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn calc_fuel_integral(mass: i32) -> i32 {
    let fuel_mass = calc_fuel(mass);
    if fuel_mass < 1 {
        0
    } else {
        fuel_mass + calc_fuel_integral(fuel_mass)
    }
}

fn main() {
    let input = vec![
        82406, 83106, 120258, 142695, 50629, 117793, 81165, 83442, 70666, 94355, 64069, 72830,
        88813, 148762, 90723, 121206, 57713, 116892, 82470, 101686, 83768, 92160, 91532, 136997,
        142382, 120050, 81062, 106227, 112071, 102275, 54033, 109059, 91772, 63320, 81872, 52925,
        92225, 60053, 110402, 97125, 87404, 54970, 66662, 83979, 88474, 91361, 69272, 61559, 56603,
        96324, 66226, 95278, 105643, 139141, 116838, 130717, 97708, 108371, 73652, 100518, 98295,
        63127, 50486, 121157, 109721, 110874, 124791, 147116, 127335, 65889, 76769, 100596, 79740,
        125860, 120185, 73861, 97700, 147169, 106781, 71891, 64744, 107113, 59274, 77680, 101891,
        69848, 98922, 147825, 128315, 55221, 119892, 87492, 75814, 80350, 131504, 81095, 57344,
        63765, 143915, 126768,
    ];

    println!(
        "Part One: {}",
        input.iter().map(|m| calc_fuel(*m)).sum::<i32>()
    );
    println!(
        "Part Two: {}",
        input.iter().map(|m| calc_fuel_integral(*m)).sum::<i32>()
    );
}

#[test]
fn test_calc_fuel() {
    assert_eq!(calc_fuel(12), 2);
    assert_eq!(calc_fuel(14), 2);
    assert_eq!(calc_fuel(1969), 654);
    assert_eq!(calc_fuel(100756), 33583);
}

#[test]
fn test_calc_fuel_integral() {
    assert_eq!(calc_fuel_integral(12), 2);
    assert_eq!(calc_fuel_integral(14), 2);
    assert_eq!(calc_fuel_integral(1969), 966);
    assert_eq!(calc_fuel_integral(100756), 50346);
}
