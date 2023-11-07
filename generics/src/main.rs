fn main() {
    let numbers1 = vec![1, 2, 3, 100, 4, 5, 6, 7, 8];
    let result1 = largest(&numbers1);
    print_result(result1);

    let numbers2: Vec<i32> = vec![];
    let result2 = largest(&numbers2);
    print_result(result2);
}

fn largest<T: std::cmp::PartialOrd>(stuff: &[T]) -> Result<&T, &'static str> {
    if stuff.len() == 0 {
        return Result::Err("There are no items in the list");
    }

    let mut largest = &stuff[0];

    for n in stuff {
        if n > largest {
            largest = n;
        }
    }

    return Result::Ok(largest);
}

fn print_result<T: std::fmt::Debug>(result: Result<T, &'static str>) {
    match result {
        Ok(n) => println!("The largest number is {n:?}"),
        Err(e) => println!("{e:?}"),
    }
}


