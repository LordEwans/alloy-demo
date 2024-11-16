use alloy::{ primitives::TxHash, providers::{ Provider, ProviderBuilder } };
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://base.llamarpc.com".parse()?;
    let transaction_hash =
        "0x3e7aaff96f6dcf9b970ca11272c64bb022dc29e806574ad099add847917eadca".parse::<TxHash>()?;

    let provider = ProviderBuilder::new().on_http(rpc_url);

    let tx = provider.get_transaction_by_hash(transaction_hash).await?.unwrap();
    let sender = tx.from;
    let sender_balance =
        provider.get_balance(sender).await.unwrap().to_string().parse::<f64>().unwrap() /
        (10f64).powi(18);
    let chain = provider.get_chain_id().await?;

    println!("sender: {sender:?}, with a balance: {sender_balance:?} eth, on chain: {chain:?}.");

    Ok(())
}
