use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::StreamExt;
use loom_types_blockchain::MempoolTx;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the provider.
    let rpc_url = "wss://mainnet.infura.io/ws/v3/<YOUR_API_KEY>";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    // Subscribe to pending transactions.
    // Alteratively use `subscribe_full_pending_transactions` to get the full transaction details
    // directly if supported by the RPC provider.
    let sub = provider.subscribe_full_pending_transactions().await?;
    // Wait and take the next 3 transactions.
    let mut stream = sub.into_stream().take(3);
    println!("Awaiting pending transactions...");
    // Take the stream and print the pending transaction.
    let handle = tokio::spawn(async move {
        while let Some(tx_hash) = stream.next().await {
            // Get the transaction details.
            if let Ok(tx) = provider.get_transaction_by_hash(tx_hash).await {
                println!("Transaction details: {tx:#?}");
            }
        }
    });
    handle.await?;
    Ok(())
}
