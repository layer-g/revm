#[cfg(test)]
mod test {
    use crate::{EVM, InMemoryDB};

    #[test]
    fn test_default_two_categories_send_eth_tx_is_category_1() {
        // Init EVM & DB
        let mut evm: EVM<InMemoryDB> = crate::new();
        let mut database = InMemoryDB::default();

        // Set block env (categories & base fee) & tx env (category)
        // evm.env.block.categories = categories
        // evm.env.tx.category = category

        // call evm inspect commit

        // verify expected output
    }

    #[test]
    fn test_three_categories_tx_in_a_whitelist() {

    }

    #[test]
    fn test_three_categories_tx_not_in_any_whitelist() {

    }
}
