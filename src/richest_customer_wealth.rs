pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    if accounts.len() == 0 {
        return 0;
    }

    if accounts.len() == 1 {
        return accounts[0].iter().sum::<i32>();
    }

    accounts
        .into_iter()
        .map(|n| n.into_iter().sum::<i32>())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::richest_customer_wealth::maximum_wealth;

    #[test]
    fn when_accounts_is_empty() {
        assert_eq!(0, maximum_wealth(vec![]))
    }

    #[test]
    fn when_accounts_has_one_account() {
        assert_eq!(6, maximum_wealth(vec![vec![1, 2, 3]]))
    }

    #[test]
    fn when_accounts_more_than_one_accounts() {
        assert_eq!(15, maximum_wealth(vec![vec![1, 2, 3], vec![4, 5, 6]]))
    }
}
