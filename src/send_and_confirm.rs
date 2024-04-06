use std::{
    io::{stdout, Write},
    time::Duration,
    str::FromStr
};

use solana_client::{
    client_error::{ClientError, ClientErrorKind, Result as ClientResult},
    nonblocking::rpc_client::RpcClient
};
use solana_program::{
    instruction::Instruction,
    pubkey::Pubkey,
    system_instruction
};
use solana_sdk::{
    commitment_config::{CommitmentConfig,},
    signature::{Signature, Signer},
    transaction::Transaction,
};

use solana_transaction_status::{TransactionConfirmationStatus};

use crate::Miner;
use crate::send_transaction_jito;

const GATEWAY_RETRIES: usize = 4;
const CONFIRM_RETRIES: usize = 4;

impl Miner {
    pub async fn send_and_confirm(
        &self,
        ixs: &[Instruction],
        skip_confirm: bool,
    ) -> ClientResult<Signature> {
        let mut stdout = stdout();
        let signer = self.signer();
        let client =
            RpcClient::new_with_commitment(self.cluster.clone(), CommitmentConfig::confirmed());

        // Return error if balance is zero
        let balance = client
            .get_balance_with_commitment(&signer.pubkey(), CommitmentConfig::confirmed())
            .await
            .unwrap();
        if balance.value <= 0 {
            return Err(ClientError {
                request: None,
                kind: ClientErrorKind::Custom("Insufficient SOL balance".into()),
            });
        }

        // Build tx
        let (mut hash, _) = client
            .get_latest_blockhash_with_commitment(CommitmentConfig::finalized())
            .await
            .unwrap();
        // Create transfer instruction for the tip
        let tip_pubkey = Pubkey::from_str("DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL").unwrap();
        let tip_instruction = system_instruction::transfer(
            &signer.pubkey(),
            &tip_pubkey,
            600_000, // lamports to transfer as a tip
        );
        let mut all_ixs = ixs.to_vec();
        all_ixs.push(tip_instruction);

        let mut tx = Transaction::new_with_payer(&all_ixs, Some(&signer.pubkey()));
        tx.message.recent_blockhash = hash;
        println!("blockhash: {:?}", tx.message.recent_blockhash);
        tx.sign(&[&signer], hash);
        // Submit tx
        let mut sigs = vec![];
        let mut attempts = 0;
        loop {
            println!("Attempt: {:?}", attempts);
            match send_transaction_jito::send_transaction_jito(&tx).await {
                Ok(sig) => {
                    sigs.push(sig);
                    println!("{:?}", sig);

                    // Confirm tx
                    if skip_confirm {
                        return Ok(sig);
                    }
                    for _ in 0..CONFIRM_RETRIES {
                        std::thread::sleep(Duration::from_millis(10000));
                        match client.get_signature_statuses(&sigs).await {
                            Ok(signature_statuses) => {
                                println!("Confirms: {:?}", signature_statuses.value);
                                for signature_status in signature_statuses.value {
                                    if let Some(signature_status) = signature_status.as_ref() {
                                        if signature_status.confirmation_status.is_some() {
                                            let current_commitment = signature_status
                                                .confirmation_status
                                                .as_ref()
                                                .unwrap();
                                            match current_commitment {
                                                TransactionConfirmationStatus::Processed => {}
                                                TransactionConfirmationStatus::Confirmed
                                                | TransactionConfirmationStatus::Finalized => {
                                                    println!("Transaction landed!");
                                                    return Ok(sig);
                                                }
                                            }
                                        } else {
                                            println!("No status");
                                        }
                                    }
                                }
                            }

                            // Handle confirmation errors
                            Err(err) => {
                                println!("Error: {:?}", err);
                            }
                        }
                    }
                    println!("Transaction did not land");
                }

                // Handle submit errors
                Err(err) => {
                    println!("Error {:?}", err);
                }
            }
            stdout.flush().ok();

            // Retry
            std::thread::sleep(Duration::from_millis(1000));
            (hash, _) = client
                .get_latest_blockhash_with_commitment(CommitmentConfig::finalized())
                .await
                .unwrap();
            tx.message.recent_blockhash = hash;
            tx.sign(&[&signer], hash);
            attempts += 1;
            if attempts > GATEWAY_RETRIES {
                return Err(ClientError {
                    request: None,
                    kind: ClientErrorKind::Custom("Max retries".into()),
                });
            }
        }
    }
}
