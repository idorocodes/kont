use litesvm::LiteSVM;
use solana_program::program_pack::Pack;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::state::{Account as SplTokenAccount, AccountState};

// --- HELPER FUNCTION TO SETUP ENVIRONMENT ---
fn setup_test_env() -> (LiteSVM, Pubkey, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = Pubkey::new_unique();

    let program_bytes = std::fs::read("../target/deploy/kont_bridge_program.so")
        .expect("Program binary not found. Did you run `cargo build-sbf`?");
    
    svm.add_program(program_id, &program_bytes).unwrap();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 2_000_000_000).unwrap();

    (svm, program_id, payer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::transaction::TransactionError;
    use solana_sdk::instruction::InstructionError;

    // --- YOUR ORIGINAL HAPPY-PATH TESTS ---

    #[test]
    fn test_active_account_with_prints() {
        let (mut svm, program_id, payer) = setup_test_env();
        let token_account_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let amount = 99_999_u64;

        let mock_token_data = SplTokenAccount {
            mint: Pubkey::new_unique(),
            owner: owner_pubkey,
            amount,
            delegate: solana_program::program_option::COption::None,
            state: AccountState::Initialized,
            is_native: solana_program::program_option::COption::None,
            delegated_amount: 0,
            close_authority: solana_program::program_option::COption::None,
        };

        let mut serialized_data = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(mock_token_data, &mut serialized_data).unwrap();

        let token_account = Account {
            lamports: 1_000_000_000,
            data: serialized_data,
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        };
        svm.set_account(token_account_pubkey, token_account).unwrap();

        let ix = Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(token_account_pubkey, false)],
            data: vec![],
        };

        let tx = Transaction::new(
            &[&payer],
            solana_sdk::message::Message::new(&[ix], Some(&payer.pubkey())),
            svm.latest_blockhash(),
        );

        let result = svm.send_transaction(tx).unwrap();

        println!("\n==================================================");
        println!("🔥 TEST: ACTIVE ACCOUNT PROCESSING");
        println!("==================================================");
        println!("Token Account Pubkey: {}", token_account_pubkey);
        println!("⚡ Compute Units Consumed: {}", result.compute_units_consumed);
        println!("--- TRANSACTION LOGS ---");
        for log in &result.logs {
            println!("  {}", log);
        }
        println!("==================================================\n");

        let logs_str = result.logs.join("\n");
        assert!(logs_str.contains(&format!("kont:amount={}", amount)));
    }

    #[test]
    fn test_maximum_possible_balance() {
        let (mut svm, program_id, payer) = setup_test_env();
        let token_account_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let max_amount = u64::MAX;

        let mock_token_data = SplTokenAccount {
            mint: Pubkey::new_unique(),
            owner: owner_pubkey,
            amount: max_amount,
            delegate: solana_program::program_option::COption::None,
            state: AccountState::Initialized,
            is_native: solana_program::program_option::COption::None,
            delegated_amount: 0,
            close_authority: solana_program::program_option::COption::None,
        };

        let mut serialized_data = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(mock_token_data, &mut serialized_data).unwrap();

        let token_account = Account {
            lamports: 1_000_000_000,
            data: serialized_data,
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        };
        svm.set_account(token_account_pubkey, token_account).unwrap();

        let ix = Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(token_account_pubkey, false)],
            data: vec![],
        };

        let tx = Transaction::new(
            &[&payer],
            solana_sdk::message::Message::new(&[ix], Some(&payer.pubkey())),
            svm.latest_blockhash(),
        );

        let result = svm.send_transaction(tx).unwrap();

        println!("\n==================================================");
        println!("💎 TEST: MAXIMUM BALANCE BOUNDARY LIMIT (u64::MAX)");
        println!("==================================================");
        println!("⚡ Compute Units Consumed: {}", result.compute_units_consumed);
        println!("--- TRANSACTION LOGS ---");
        for log in &result.logs {
            println!("  {}", log);
        }
        println!("==================================================\n");

        let logs_str = result.logs.join("\n");
        assert!(logs_str.contains(&format!("kont:amount={}", max_amount)));
    }

    #[test]
    fn test_zero_balance_account() {
        let (mut svm, program_id, payer) = setup_test_env();
        let token_account_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();

        let mock_token_data = SplTokenAccount {
            mint: Pubkey::new_unique(),
            owner: owner_pubkey,
            amount: 0,
            delegate: solana_program::program_option::COption::None,
            state: AccountState::Initialized,
            is_native: solana_program::program_option::COption::None,
            delegated_amount: 0,
            close_authority: solana_program::program_option::COption::None,
        };

        let mut serialized_data = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(mock_token_data, &mut serialized_data).unwrap();

        let token_account = Account {
            lamports: 1_000_000_000,
            data: serialized_data,
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        };
        svm.set_account(token_account_pubkey, token_account).unwrap();

        let ix = Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(token_account_pubkey, false)],
            data: vec![],
        };

        let tx = Transaction::new(
            &[&payer],
            solana_sdk::message::Message::new(&[ix], Some(&payer.pubkey())),
            svm.latest_blockhash(),
        );

        let result = svm.send_transaction(tx).unwrap();

        println!("\n==================================================");
        println!("⚪ TEST: ZERO BALANCE");
        println!("==================================================");
        println!("⚡ Compute Units Consumed: {}", result.compute_units_consumed);
        println!("--- TRANSACTION LOGS ---");
        for log in &result.logs {
            println!("  {}", log);
        }
        println!("==================================================\n");

        let logs_str = result.logs.join("\n");
        assert!(logs_str.contains("kont:amount=0"));
    }

 

 
    #[test]
    fn test_frozen_account_state() {
        let (mut svm, program_id, payer) = setup_test_env();
        let token_account_pubkey = Pubkey::new_unique();

        let mock_token_data = SplTokenAccount {
            mint: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            amount: 500,
            delegate: solana_program::program_option::COption::None,
            state: AccountState::Frozen, // Set state byte to 2
            is_native: solana_program::program_option::COption::None,
            delegated_amount: 0,
            close_authority: solana_program::program_option::COption::None,
        };

        let mut serialized_data = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(mock_token_data, &mut serialized_data).unwrap();

        let token_account = Account {
            lamports: 1_000_000_000,
            data: serialized_data,
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        };
        svm.set_account(token_account_pubkey, token_account).unwrap();

        let ix = Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(token_account_pubkey, false)],
            data: vec![],
        };

        let tx = Transaction::new(&[&payer], solana_sdk::message::Message::new(&[ix], Some(&payer.pubkey())), svm.latest_blockhash());
        let result = svm.send_transaction(tx).unwrap();

        println!("\n==================================================");
        println!("❄️  TEST: FROZEN STATE VERIFICATION");
        println!("==================================================");
        println!("⚡ Compute Units Consumed: {}", result.compute_units_consumed);
        println!("==================================================\n");

        let logs_str = result.logs.join("\n");
        assert!(logs_str.contains("kont:frozen=true"));
    }

    
    #[test]
    fn test_account_owner_spoof_vulnerability() {
        let (mut svm, program_id, payer) = setup_test_env();
        let malicious_account_pubkey = Pubkey::new_unique();
        let fake_program_owner = Pubkey::new_unique(); // NOT spl_token::id()

        let mock_token_data = SplTokenAccount {
            mint: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            amount: 888_888,
            delegate: solana_program::program_option::COption::None,
            state: AccountState::Initialized,
            is_native: solana_program::program_option::COption::None,
            delegated_amount: 0,
            close_authority: solana_program::program_option::COption::None,
        };

        let mut serialized_data = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(mock_token_data, &mut serialized_data).unwrap();

        // Inject the spoofed account with an incorrect owner ID
        let spoofed_account = Account {
            lamports: 1_000_000_000,
            data: serialized_data,
            owner: fake_program_owner, 
            executable: false,
            rent_epoch: 0,
        };
        svm.set_account(malicious_account_pubkey, spoofed_account).unwrap();

        let ix = Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(malicious_account_pubkey, false)],
            data: vec![],
        };

       let tx = Transaction::new(&[&payer], solana_sdk::message::Message::new(&[ix], Some(&payer.pubkey())), svm.latest_blockhash());
        let err = svm.send_transaction(tx).unwrap_err();

        // Match on the underlying TransactionError inside the metadata
        match err.err {
            TransactionError::InstructionError(0, InstructionError::IllegalOwner) => {} 
            TransactionError::InstructionError(0, InstructionError::Custom(_)) => {} 
            TransactionError::InstructionError(0, InstructionError::InvalidAccountData) => {}
            _ => panic!("Security vulnerability! The program processed data from an unverified owner account: {:?}", err.err),
        }
    }
    #[test]
    fn test_malformed_short_account_data() {
        let (mut svm, program_id, payer) = setup_test_env();
        let broken_account_pubkey = Pubkey::new_unique();

        let short_data = vec![0u8; 45]; // Missing 120 bytes of required structure data

        let broken_account = Account {
            lamports: 1_000_000_000,
            data: short_data,
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        };
        svm.set_account(broken_account_pubkey, broken_account).unwrap();

        let ix = Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(broken_account_pubkey, false)],
            data: vec![],
        };

let tx = Transaction::new(&[&payer], solana_sdk::message::Message::new(&[ix], Some(&payer.pubkey())), svm.latest_blockhash());
        let err = svm.send_transaction(tx).unwrap_err();

        // Match on the underlying TransactionError inside the metadata
        match err.err {
            TransactionError::InstructionError(0, InstructionError::InvalidAccountData) => {}
            TransactionError::InstructionError(0, InstructionError::Custom(_)) => {}
            _ => panic!("Expected a data check error, got unexpected transaction result: {:?}", err.err),
        }
    }

    /// Test 7: Completely Empty Account Data Slice (0 Bytes)
    /// Tests the radical zero-boundary constraint where account data is totally missing.
    #[test]
    fn test_empty_account_data() {
        let (mut svm, program_id, payer) = setup_test_env();
        let empty_account_pubkey = Pubkey::new_unique();

        let empty_account = Account {
            lamports: 1_000_000_000,
            data: vec![], // Completely empty
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        };
        svm.set_account(empty_account_pubkey, empty_account).unwrap();

        let ix = Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(empty_account_pubkey, false)],
            data: vec![],
        };

        let tx = Transaction::new(&[&payer], solana_sdk::message::Message::new(&[ix], Some(&payer.pubkey())), svm.latest_blockhash());
        let err = svm.send_transaction(tx).unwrap_err();

        // Check the underlying TransactionError inside the metadata
        assert!(matches!(err.err, TransactionError::InstructionError(0, _)));
    }
 
    #[test]
    fn test_oversized_token_extensions_data() {
        let (mut svm, program_id, payer) = setup_test_env();
        let oversized_account_pubkey = Pubkey::new_unique();
        let amount = 1_234_567_u64;

        // Simulate a 300-byte raw account structure (Legacy layout + extensions)
        let mut complex_data = vec![0u8; 300];
        
        let mock_token_data = SplTokenAccount {
            mint: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            amount,
            delegate: solana_program::program_option::COption::None,
            state: AccountState::Initialized,
            is_native: solana_program::program_option::COption::None,
            delegated_amount: 0,
            close_authority: solana_program::program_option::COption::None,
        };

        // Pack the first 165 bytes exactly as a traditional layout
        let mut base_serialized = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(mock_token_data, &mut base_serialized).unwrap();
        complex_data[..SplTokenAccount::LEN].copy_from_slice(&base_serialized);

        let oversized_account = Account {
            lamports: 2_000_000_000,
            data: complex_data,
            owner: spl_token::id(), // Or Token-2022 ID depending on program targeting scope
            executable: false,
            rent_epoch: 0,
        };
        svm.set_account(oversized_account_pubkey, oversized_account).unwrap();

        let ix = Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(oversized_account_pubkey, false)],
            data: vec![],
        };

        let tx = Transaction::new(&[&payer], solana_sdk::message::Message::new(&[ix], Some(&payer.pubkey())), svm.latest_blockhash());
        let result = svm.send_transaction(tx);

        println!("\n==================================================");
        println!("🚀 TEST: OVERSIZED TOKEN-2022 EXTENSION PARSE");
        println!("==================================================");
        match &result {
            Ok(meta) => {
                println!("Result: ✅ Success (Program slices dynamically)");
                println!("⚡ Compute Units Consumed: {}", meta.compute_units_consumed);
                let logs_str = meta.logs.join("\n");
                assert!(logs_str.contains(&format!("kont:amount={}", amount)));
            }
            Err(e) => {
                println!("Result: ❌ Rejected (Program enforces exact 165-byte verification)");
                println!("Error Details: {:?}", e);
            }
        }
        println!("==================================================\n");
    }
}