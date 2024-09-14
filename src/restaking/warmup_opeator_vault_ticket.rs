use std::path::PathBuf;

use clap::Parser;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use super::RestakingHandler;

#[derive(Parser)]
#[command(about = "Warmup Operator Vault Ticket account")]
pub struct WarmupOperatorVaultTicket {
    /// RPC URL for the cluster
    #[arg(short, long, env, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    /// Path to keypair used to pay
    #[arg(long, env, default_value = "~/.config/solana/id.json")]
    keypair: PathBuf,

    /// Validator history program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "5b2dHDz9DLhXnwQDG612bgtBGJD62Riw9s9eYuDT3Zma"
    )]
    restaking_program_id: Pubkey,

    /// operator pubkey
    #[arg(long)]
    operator: Pubkey,

    /// Vault Pubkey
    #[arg(long)]
    vault: Pubkey,
}

pub async fn command_warmup_operator_vault_ticket(args: WarmupOperatorVaultTicket) {
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let handler = RestakingHandler::new(&args.rpc_url, &payer, args.restaking_program_id);
    handler
        .warmup_operator_vault_ticket(args.operator, args.vault)
        .await;
}
