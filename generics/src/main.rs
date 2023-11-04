fn main() {
    //let numbers = vec![1, 2, 3, 100, 4, 5, 6, 7, 8];
    let numbers: Vec<i32> = vec![];

    match largest(&numbers) {
        Ok(n) => println!("The largest number is {}", n),
        Err(e) => println!("{}", e),
    }
}

fn largest<T: std::cmp::PartialOrd>(numbers: &[T]) -> Result<&T, &'static str> {
    if numbers.len() == 0 {
        return Result::Err("There are no items in the list");
    }

    let mut largest = &numbers[0];

    for n in numbers {
        if n > largest {
            largest = n;
        }
    }

    return Result::Ok(largest);
}

