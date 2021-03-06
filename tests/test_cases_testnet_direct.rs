//! Runs standard test cases against TestNet with several nodes running in separate threads.
//! The communication is performed through `ThreadUser` that performs direct communication with
//! internals of nodes.
#[cfg(feature = "expensive_tests")]
#[cfg(test)]
mod test {
    use testlib::standard_test_cases::*;

    use configs::chain_spec::DefaultIdType;
    use node_runtime::test_utils::alice_account;
    use std::sync::atomic::{AtomicU16, Ordering};
    use testlib::node::thread_node::ThreadNode;
    use testlib::node::{
        create_nodes_with_id_type, Node, NodeConfig, TEST_BLOCK_FETCH_LIMIT, TEST_BLOCK_MAX_SIZE,
    };
    use testlib::test_helpers::heavy_test;
    const NUM_TEST_NODE: usize = 4;
    static TEST_PORT: AtomicU16 = AtomicU16::new(6000);

    fn create_thread_nodes(test_prefix: &str, test_port: u16) -> Vec<ThreadNode> {
        let (_, account_names, nodes) = create_nodes_with_id_type(
            NUM_TEST_NODE,
            test_prefix,
            test_port,
            TEST_BLOCK_FETCH_LIMIT,
            TEST_BLOCK_MAX_SIZE,
            vec![],
            DefaultIdType::Named,
        );
        assert_eq!(account_names[0], alice_account());
        let mut nodes: Vec<_> = nodes
            .into_iter()
            .map(|cfg| match cfg {
                NodeConfig::Thread(config) => ThreadNode::new(config),
                _ => unreachable!(),
            })
            .collect();
        for i in 0..NUM_TEST_NODE {
            nodes[i].start();
        }
        nodes
    }

    /// Macro for running testnet test. Increment the atomic global counter for port,
    /// and get the test_prefix from the test name.
    macro_rules! run_testnet_test {
        ($f:expr) => {
            let port = TEST_PORT.fetch_add(NUM_TEST_NODE as u16, Ordering::SeqCst);
            let test_prefix = stringify!($f);
            let mut nodes = create_thread_nodes(test_prefix, port);
            let node = nodes.pop().unwrap();
            heavy_test(|| $f(node));
        };
    }

    #[test]
    fn test_smart_contract_simple_testnet() {
        run_testnet_test!(test_smart_contract_simple);
    }

    #[test]
    fn test_smart_contract_bad_method_name_testnet() {
        run_testnet_test!(test_smart_contract_bad_method_name);
    }

    #[test]
    fn test_smart_contract_empty_method_name_with_no_tokens_testnet() {
        run_testnet_test!(test_smart_contract_empty_method_name_with_no_tokens);
    }

    #[test]
    fn test_smart_contract_empty_method_name_with_tokens_testnet() {
        run_testnet_test!(test_smart_contract_empty_method_name_with_tokens);
    }

    #[test]
    fn test_smart_contract_with_args_testnet() {
        run_testnet_test!(test_smart_contract_with_args);
    }

    #[test]
    fn test_async_call_with_no_callback_testnet() {
        run_testnet_test!(test_async_call_with_no_callback);
    }

    #[test]
    fn test_async_call_with_callback_testnet() {
        run_testnet_test!(test_async_call_with_callback);
    }

    #[test]
    fn test_async_call_with_logs_testnet() {
        run_testnet_test!(test_async_call_with_logs);
    }

    #[test]
    fn test_deposit_with_callback_testnet() {
        run_testnet_test!(test_deposit_with_callback);
    }

    #[test]
    fn test_nonce_update_when_deploying_contract_testnet() {
        run_testnet_test!(test_nonce_update_when_deploying_contract);
    }

    #[test]
    fn test_nonce_updated_when_tx_failed_testnet() {
        run_testnet_test!(test_nonce_updated_when_tx_failed);
    }

    #[test]
    fn test_upload_contract_testnet() {
        run_testnet_test!(test_upload_contract);
    }

    #[test]
    fn test_redeploy_contract_testnet() {
        run_testnet_test!(test_redeploy_contract);
    }

    #[test]
    fn test_send_money_testnet() {
        run_testnet_test!(test_send_money);
    }

    #[test]
    fn test_send_money_over_balance_testnet() {
        run_testnet_test!(test_send_money_over_balance);
    }

    #[test]
    fn test_refund_on_send_money_to_non_existent_account_testnet() {
        run_testnet_test!(test_refund_on_send_money_to_non_existent_account);
    }

    #[test]
    fn test_create_account_testnet() {
        run_testnet_test!(test_create_account);
    }

    #[test]
    fn test_create_account_again_testnet() {
        run_testnet_test!(test_create_account_again);
    }

    #[test]
    fn test_create_account_failure_invalid_name_testnet() {
        run_testnet_test!(test_create_account_failure_invalid_name);
    }

    #[test]
    fn test_create_account_failure_already_exists_testnet() {
        run_testnet_test!(test_create_account_failure_already_exists);
    }

    #[test]
    fn test_swap_key_testnet() {
        run_testnet_test!(test_swap_key);
    }

    #[test]
    fn test_add_key_testnet() {
        run_testnet_test!(test_add_key);
    }

    #[test]
    fn test_add_existing_key_testnet() {
        run_testnet_test!(test_add_existing_key);
    }

    #[test]
    fn test_delete_key_testnet() {
        run_testnet_test!(test_delete_key);
    }

    #[test]
    fn test_delete_key_not_owned_testnet() {
        run_testnet_test!(test_delete_key_not_owned);
    }

    #[test]
    fn test_delete_key_no_key_left_testnet() {
        run_testnet_test!(test_delete_key_no_key_left);
    }
}
