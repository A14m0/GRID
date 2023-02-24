// This file implements some misc algebra type maths

num main() {
    // the goal of this program is to find the sum of all 
    // multiples of 3 or 5 below 1000

    num sum = 0;

    // im biased and like Rust's format for defining ranges (which would be iterables)
    for(num i in 0..1000) {
        if(i % 3 == 0 || i % 5 == 0) {
            sum += i;
        }
    }

    print("Sum of all multiples of 3 or 5 below 1000: " + sum.to_string());

    return 0;
}