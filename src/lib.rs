pub mod bonk;

// use dotenvy::dotenv;
// use env_logger::{Builder, fmt::Formatter};
// use log::{error, info, warn};
// use sol_trade_sdk::swqos::flashblock::FlashBlockClient;
// use solana_sdk::{
//     instruction::Instruction,
//     pubkey,
//     pubkey::Pubkey,
//     signature::{Keypair, Signature},
//     signer::{EncodableKey, Signer},
// };
// use std::io::Write;
// use tokio::time::timeout;

// use sol_tx_send::platform_clients::{BuildTx, HashParam, TxSend};
// use solana_sdk::system_program;
// use spl_associated_token_account::instruction::create_associated_token_account;

// pub static VOLUME_BOOSTER: LazyLock<Arc<Keypair>> = LazyLock::new(|| {
//     let payer_path = env::var("BONK_VOLUME_BOOSTER_KEYPAIR_PATH")
//         .expect("PAYER_KEYPAIR_PATH environment variable must be set");
//     let keypair = Keypair::read_from_file(&payer_path)
//         .unwrap_or_else(|e| panic!("Failed to read keypair from file '{}': {}", payer_path, e));
//     info!("Using wallet : {}", keypair.pubkey());
//     Arc::new(keypair)
// });

// #[tokio::main]
// async fn main() {
//     rustls::crypto::CryptoProvider::install_default(rustls::crypto::ring::default_provider())
//         .unwrap();
//     dotenv().ok();
//     Builder::new()
//         .format(|buf: &mut Formatter, record: &log::Record| {
//             let ts = buf.timestamp_micros();
//             writeln!(
//                 buf,
//                 "[{} {} {}] {}",
//                 ts,
//                 record.level(),
//                 record.target(),
//                 record.args()
//             )
//         })
//         .filter_level(log::LevelFilter::Info)
//         .init();

//     let rpc = JSON_RPC_CLIENT.clone();
//     let mint = pubkey!("Fun6nFAxk1UNaXaFP7n1Q2qbmTWFD3jhoxPj1rC2bonk");
//     let creator = pubkey!("CreTUe3oDxMjejJiUPDe92iMmRsK2rVCcwbsNUxUu4DF");
//     let wsol = pubkey!("So11111111111111111111111111111111111111112");
//     let owner = VOLUME_BOOSTER.pubkey();
//     let platform_config = pubkey!("FfYek5vEz23cMkWsdJwG2oa6EphsvXSHrGpdALN4g6W1");

//     for i in 0..1 {
//         // 1. 幂等创建ATA指令
//         let create_ata_ix =
//             spl_associated_token_account::instruction::create_associated_token_account_idempotent(
//                 &owner,
//                 &owner,
//                 &mint,
//                 &spl_token::id(),
//             );

//         // 2. 买入参数
//         let amount_in = 10.to_lamport();
//         let max_amount_in = amount_in + (amount_in / 10);
//         let _min_amount_out = amount_in - amount_in / 10;

//         // 3. 用get_amount_out函数根据池子虚拟余额算价
//         let vbase = 1_073_025_605_596_382u128;
//         let vquote = 30_000_852_951u128;
//         let amount_out = crate::bonk::get_out_amount(vbase, vquote, amount_in as u128) as u64;

//         // 4. 构造buy_exact_out指令
//         let buy_ix = crate::bonk::ixs::buy::build_buy_exact_out_ix(
//             owner,
//             mint,
//             wsol,
//             creator,
//             platform_config,
//             amount_out,
//             max_amount_in,
//             0,
//         );

//         // 5. 构造sell_exact_in指令（卖出刚买到的token）
//         let sell_ix = crate::bonk::ixs::sell::build_sell_exact_in_ix(
//             owner,
//             mint,
//             wsol,
//             creator,
//             platform_config,
//             amount_out,
//             1,
//             0,
//         );

//         let ixs = vec![create_ata_ix, buy_ix.clone(), sell_ix.clone()];

//         // let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
//         //     &ixs,
//         //     Some(&owner),
//         //     &[PAYER.as_ref()],
//         //     rpc.get_latest_blockhash().await.unwrap(),
//         // );
//         // let _ = rpc
//         //     .send_and_confirm_transaction(&tx)
//         //     .await
//         //     .map(|sig| info!("main buy and sell confirmed ({}): {sig}", i + 1))
//         //     .map_err(|e| {
//         //         error!("main buy and sell failed ({}): {e}", i + 1);
//         //     });
//         let start = std::time::Instant::now();
//         let recent_hash = match retry_result!(100, rpc.get_latest_blockhash().await)
//             .expect("can't get recent block hash")
//         {
//             hash => hash,
//         };
//         let hash = HashParam::Blockhash(recent_hash);
//         let tx = FLASHBLOCK_CLIENT.build_tx(
//             &ixs,
//             &VOLUME_BOOSTER,
//             &None,
//             &hash,
//             &(None, Some(1_000)),
//             None,
//         );
//         let sig = tx.sig();
//         match retry_result!(5, tx.send().await) {
//             Ok(sig) => {
//                 info!("tx {sig} sent")
//             }
//             Err(e) => {
//                 error!("main buy and sell failed ({}): {e}", i + 1);
//                 continue;
//             }
//         };
//         match wait_tx_confirmed(&JSON_RPC_CLIENT, &sig).await {
//             Ok(s) => info!("tx {s} confirmed!"),
//             Err(e) => error!("tx {sig} not confirmed: {e}"),
//         }

//         info!("No.{} tx {sig} sent in {:?}", i + 1, start.elapsed());
//         tokio::time::sleep(Duration::from_millis(100));
//     }
// }

// pub async fn wait_tx_confirmed(
//     rpc: &solana_client::nonblocking::rpc_client::RpcClient,
//     sig: &Signature,
// ) -> Result<Signature, String> {
//     let check_interval = Duration::from_millis(500);
//     let max_wait = Duration::from_secs(5);

//     let result = timeout(max_wait, async {
//         loop {
//             match rpc.get_signature_status(sig).await {
//                 Ok(Some(status)) if status.is_ok() => {
//                     return Ok(sig.clone());
//                 }
//                 Ok(Some(status)) if status.is_err() => {
//                     return Err(format!("tx failed: {:?}", status));
//                 }
//                 Ok(Some(res)) => {
//                     warn!("unexpected status: {:?}", res);
//                 }
//                 Ok(None) => {
//                     // 未确认，继续等待
//                 }
//                 Err(e) => {
//                     // 网络错误等，可选择直接返回或继续重试
//                     return Err(format!("rpc error: {:?}", e));
//                 }
//             }
//             tokio::time::sleep(check_interval).await;
//         }
//     })
//     .await;

//     match result {
//         Ok(r) => r,
//         Err(_) => Err("timeout waiting for tx confirmation".to_string()),
//     }
// }

// async fn build_transactions(
//     ixs: &[Instruction],
//     tip: u64,
//     cu: &(Option<u32>, Option<u64>),
// ) -> (Vec<Box<dyn TxSend>>, Vec<Signature>) {
//     let payer = &VOLUME_BOOSTER;
//     let recent_hash = match retry_result!(100, JSON_RPC_CLIENT.get_latest_blockhash().await)
//         .expect("can't get recent block hash")
//     {
//         hash => hash,
//     };
//     let hash = HashParam::Blockhash(recent_hash);

//     let txs: Vec<Box<dyn TxSend>> = vec![
//         // Box::new(ASTRALANE_CLIENT.build_tx(ixs, payer, &None, &nonce_param, &None)), // astralane 暂不可用，待修复
//         Box::new(JITO_CLIENT.build_tx(ixs, payer, &Some(tip), &hash, cu, None)),
//         Box::new(NODEONE_CLIENT.build_tx(ixs, payer, &Some(tip), &hash, cu, None)),
//         Box::new(BLOCKRAZOR_CLIENT.build_tx(ixs, payer, &Some(tip), &hash, cu, None)),
//         Box::new(TEMPORAL_CLIENT.build_tx(ixs, payer, &Some(tip), &hash, cu, None)),
//         Box::new(HELIUS_CLIENT.build_tx(ixs, payer, &Some(tip), &hash, cu, None)),
//         Box::new(ZEROSLOT_CLIENT.build_tx(ixs, payer, &Some(tip), &hash, cu, None)),
//         Box::new(FLASHBLOCK_CLIENT.build_tx(ixs, payer, &Some(tip), &hash, cu, None)),
//     ];

//     // 提取所有交易的签名
//     let signatures: Vec<Signature> = txs.iter().map(|tx| tx.sig()).collect();

//     (txs, signatures)
// }

// #[tokio::test]
// async fn test_buy_and_sell() {
//     dotenv().ok();
//     Builder::new()
//         .format(|buf: &mut Formatter, record: &log::Record| {
//             let ts = buf.timestamp_micros();
//             writeln!(
//                 buf,
//                 "[{} {} {}] {}",
//                 ts,
//                 record.level(),
//                 record.target(),
//                 record.args()
//             )
//         })
//         .filter_level(log::LevelFilter::Info)
//         .init();

//     let rpc = JSON_RPC_CLIENT.clone();
//     let mint = pubkey!("13TymMBxgdLTUYSqioemwnRapQwp8apsw8zhUYDMsrNJ");
//     let creator = pubkey!("2FX2KigeEk7BVFmroVEepYD1KVgjohvUjnTv6nRzFEUA");
//     let wsol = pubkey!("So11111111111111111111111111111111111111112");
//     let owner = VOLUME_BOOSTER.pubkey();
//     let platform_config = pubkey!("4Bu96XjU84XjPDSpveTVf6LYGCkfW5FK7SNkREWcEfV4");

//     // 1. 创建ATA指令
//     let create_ata_ix =
//         spl_associated_token_account::instruction::create_associated_token_account_idempotent(
//             &owner,
//             &owner,
//             &mint,
//             &spl_token::id(),
//         );

//     // 2. 买入参数
//     let sol_decimals = 1_000_000_000u64;
//     let amount_in = 0.001.to_lamport(); // 0.001 SOL
//     let max_amount_in = 0.0011.to_lamport(); // 0.0011 SOL
//     let min_amount_out = 0.0009.to_lamport(); // 0.0009 SOL

//     // 3. 查询当前能买到多少token（用buy_exact_out的simulate）
//     // 这里只做简单估算，实际应用可用rpc.simulate_transaction或链上查询
//     // 假设1 SOL = 34_000_000_000_000 token，0.001 SOL = 34_000_000_000 token
//     // 实际应用请替换为链上预估
//     let token_per_sol = 34_000_000_000_000u64;
//     let amount_out = (amount_in as u128 * token_per_sol as u128 / sol_decimals as u128) as u64;

//     // 4. 构造buy_exact_out指令
//     let buy_ix = crate::bonk::ixs::buy::build_buy_exact_out_ix(
//         owner,
//         mint,
//         wsol,
//         creator,
//         platform_config,
//         amount_out,
//         max_amount_in,
//         0,
//     );

//     // 5. 构造sell_exact_in指令（卖出刚买到的token）
//     let sell_ix = crate::bonk::ixs::sell::build_sell_exact_in_ix(
//         owner,
//         mint,
//         wsol,
//         creator,
//         platform_config,
//         amount_out,
//         1,
//         0,
//     );

//     let ixs = [create_ata_ix, buy_ix.clone(), sell_ix.clone()];

//     let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
//         &ixs,
//         Some(&owner),
//         &[VOLUME_BOOSTER.as_ref()],
//         rpc.get_latest_blockhash().await.unwrap(),
//     );
//     let _ = rpc
//         .send_and_confirm_transaction(&tx)
//         .await
//         .map(|sig| info!("buy and sell confirmed: {sig}"))
//         .map_err(|e| {
//             error!("buy and sell failed: {e}");
//             panic!()
//         });
// }
