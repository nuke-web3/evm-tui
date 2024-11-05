use alloy::{
    network::{Ethereum, EthereumWallet, NetworkWallet, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{
        ledger::{HDPath, LedgerSigner},
        local::PrivateKeySigner,
        Signer,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // https://github.com/alloy-rs/examples/blob/main/examples/transactions/examples/transfer_eth.rs
    // https://alloy.rs/examples/providers/builder.html

    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;
    // Set up the HTTP provider with the `reqwest` crate.
    let rpc_url = anvil.endpoint_url();

    // Set up signer from the first default Anvil account (Alice).
    let allice_signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let alice = allice_signer.address();
    let allice_wallet = EthereumWallet::from(allice_signer);

    let allice_provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(allice_wallet)
        .on_http(rpc_url.to_owned());

    // NOTE: The Ledger must  have the elthereum application open
    let ledger_signer = LedgerSigner::new(HDPath::LedgerLive(0), Some(31337)).await?;
    let ledger = ledger_signer.address();
    let ledger_wallet = EthereumWallet::from(ledger_signer);

    // Build a transaction to send from Ledger to Bob.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let allice_tx = TransactionRequest::default()
        .with_from(alice)
        .with_to(ledger)
        .with_gas_price(20_000_000_000)
        .with_gas_limit(21_000)
        .with_value(U256::from(420_000_000_000_000i64));

    dbg!(&allice_tx);

    // Send the transaction and listen for the transaction to be included.
    allice_provider
        .send_transaction(allice_tx.to_owned())
        .await?
        .watch()
        .await?;

    // println!("✨✨✨✨✨✨\nSent transaction:\n{allice_tx:?}\n✨✨✨✨✨");

    let ledger_provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(ledger_wallet)
        .on_http(rpc_url);

    // Build a transaction to send from Ledger to Vitalik.
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let ledger_tx = TransactionRequest::default()
        .with_from(ledger)
        .with_to(vitalik)
        .with_gas_price(20_000_000_000)
        .with_gas_limit(21_000)
        .with_value(U256::from(69_000_000_000_000i64));

    dbg!(&ledger_tx);

    ledger_provider
        .send_transaction(ledger_tx.to_owned())
        .await?
        .watch()
        .await?;

    // println!("✨✨✨✨✨✨\nSent transaction:\n{ledger_tx:?}\n✨✨✨✨✨");

    Ok(())
}
