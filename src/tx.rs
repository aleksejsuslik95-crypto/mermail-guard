use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    system_transaction,
};
use std::str::FromStr;

pub struct WalletExecutor {
    rpc_client: RpcClient,
    payer: Keypair,
}

impl WalletExecutor {
    pub fn new() -> Self {
        let rpc_url = "https://api.devnet.solana.com".to_string();
        let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        let payer = Keypair::new();

        Self { rpc_client, payer }
    }

    /// Проверяет, является ли строка валидным base58 адресом Solana
    pub fn validate_recipient(address: &str) -> Result<Pubkey> {
        Pubkey::from_str(address)
            .map_err(|_| anyhow!("Invalid Solana public key format"))
    }

    /// Выполняет перевод в Devnet
    pub fn transfer_testnet(&self, to_pubkey: &Pubkey, amount_tokens: f64) -> Result<String> {
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let lamports = sol_to_lamports(amount_tokens);

        let tx = system_transaction::transfer(
            &self.payer,
            to_pubkey,
            lamports,
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&tx)?;
        Ok(signature.to_string())
    }
}