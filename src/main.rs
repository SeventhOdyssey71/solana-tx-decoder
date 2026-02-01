use anyhow::Result;
use clap::Parser;
use solana_client::rpc_client::RpcClient;

#[derive(Parser, Debug)]
#[command(name = "solana-tx-decoder")]
#[command(about = "Decode solana transactions", long_about = None)]

struct Args {
    #[arg(short, long)]
    signature: String,

    #[arg(short, long)]
    rpc_url: String,
}

#[tokio::main]

async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Decoding transaction with signature: {}", args.signature);
    println!("Using RPC URL: {}\n", args.rpc_url);

    //Create RPC Client
    let rpc_client = RpcClient::new_with_commitment(
        args.rpc_url,
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    );

    // Fetch transaction details
    let config = solana_client::rpc_config::RpcTransactionConfig {
        encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
        commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    let signature = args.signature.parse().expect("Invalid signature");
    let tx = rpc_client.get_transaction_with_config(&signature, config)?;

    // Print transaction details
    println!("Transaction found successfully!");
    println!("Slot: {}", tx.slot);
    println!("Block Time: {:?}", tx.block_time);

    if let Some(meta) = tx.transaction.meta {
        println!("Fee: {} Lamports", meta.fee);
        println!("Success: {}", meta.status.is_ok());
    }

    Ok(())
}
