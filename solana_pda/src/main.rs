use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

fn main() -> Result<()> {
    // 1) Connect to Devnet
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());

    // 2) Create a fresh payer and airdrop 2 SOL
    let payer = Keypair::new();
    let sig = rpc.request_airdrop(&payer.pubkey(), 2 * LAMPORTS_PER_SOL)?;
    println!("💧 Requested airdrop: {}", sig);
    // Wait until the airdrop has been processed
    loop {
        let bal = rpc.get_balance(&payer.pubkey())?;
        if bal >= 2 * LAMPORTS_PER_SOL {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("🌞 Airdropped 2 SOL to {}", payer.pubkey());("🌞 Airdropped 2 SOL to {}", payer.pubkey());

    // 3) Derive and create a seeded account
    let seed_str = "my_unique_seed";
    let base = payer.pubkey();
    // Define the program or owner for the data account
    let program_id = Pubkey::new_unique();

    // Derive a deterministic address using base + seed + owner
    let seeded_addr = Pubkey::create_with_seed(&base, seed_str, &program_id)?;
    println!("🔑 Derived seeded account: {}", seeded_addr);

    // Build instruction to create the account
    let lamports = LAMPORTS_PER_SOL; // fund with 1 SOL
    let space = 0;                   // no data space
    let owner = program_id;          // owner of this account

    let create_ix = system_instruction::create_account_with_seed(
        &base,         // funding & signer
        &seeded_addr,  // new account address
        &base,         // base keypair signs as new account signer
        seed_str,
        lamports,
        space,
        &owner,
    );

    let blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&base),
        &[&payer],
        blockhash,
    );
    rpc.send_and_confirm_transaction(&tx)?;
    println!("✅ Seeded account created and funded");

    // 4) Fetch & display the new account's info
    let account = rpc.get_account(&seeded_addr)?;
    println!("🔍 Account info at {}:", seeded_addr);
    println!("   • Lamports:   {}", account.lamports);
    println!("   • Owner:      {}", account.owner);
    println!("   • Data length:{}", account.data.len());
    println!("   • Executable: {}", account.executable);

    Ok(())
}
