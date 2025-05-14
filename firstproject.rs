fn main() {
    let number = 5;
    let result = factorial(number);
    println!("The factorial of {} is {}", number, result);
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    while n > 1 {
        let mut new_n = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
    }
}

fn print_array(arr: &[i32]) {
    for &val in arr {
        print!("{} ", val);
    }
    println!();
}

fn main() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array:");
    print_array(&numbers);

    bubble_sort(&mut numbers);
    println!("Sorted array:");
    print_array(&numbers);
}