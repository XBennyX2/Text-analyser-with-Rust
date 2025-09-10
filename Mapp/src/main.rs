fn my_map<T, U, F>(list: &[T], func: &F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    if list.is_empty() {
        vec![]
    } else {
        let (first, rest) = list.split_first().unwrap();
        let mut result = vec![func(first)];
        result.extend(my_map(rest, func));
        result
    }
}

fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(&A) -> C
where
    F: Fn(B) -> C,
    G: Fn(&A) -> B,
{
    move |x| f(g(x))
}

fn main() {
    let numbers = vec![1, 2, 3, 4];

    // pure functions
    let double = |x: &i32| x * 2;       // takes a reference
    let add_three = |x: i32| x + 3;     // works on a value

    // compose into one function
    let transform = compose(add_three, double);

    // apply with our recursive map
    let result = my_map(&numbers, &transform);

    println!("Result: {:?}", result); // [5, 7, 9, 11]
}
