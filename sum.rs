fn sum<T: std::convert::From<T> + std::marker::Copy, O>(data: Vec<T>) -> O
where
    O: std::ops::Add<O, Output = O> + std::default::Default,
{
    let mut result = O::default();
    for b in data {
        result = result + O::from(b);
    }
    result
}
