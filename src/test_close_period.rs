#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Events, Env};

#[test]
fn test_deferred_flush_lifecycle() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    // 1. Report revenue with defer_until_close = true
    client.report_revenue(&1, &5000, &true);

    // 2. Replace the deferred report before close
    client.replace_deferred(&1, &6000);

    // 3. Close the period (Atomic flush)
    client.close_period(&1);

    // 4. Claim succeeds because flush removed the deferred flag
    client.claim(&1);
}

#[test]
#[should_panic(expected = "Error(Contract, #456)")]
fn test_claim_on_deferred_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    // Report and defer
    client.report_revenue(&2, &5000, &true);

    // Attempting to claim before close_period must panic with DistributionDeferred
    client.claim(&2);
}
