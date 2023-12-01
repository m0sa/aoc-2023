pub fn somefn() {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn some_test() {
        let input = utils::resource("src/day01.txt");
        somefn();
    }
}
