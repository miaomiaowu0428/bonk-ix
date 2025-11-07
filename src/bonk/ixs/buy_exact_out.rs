use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
};

use crate::bonk::ixs::{
    accounts::{self, AUTHORITY, EVENT_AUTHORITY, GLOBAL_CONFIG, PROGRAM},
    get_creator_vault, get_platform_vault, get_pool_pda, get_vault_pda,
};

const BUY_EXACT_OUT_DISCRIMINATOR: [u8; 8] = [0x18, 0xd3, 0x74, 0x28, 0x69, 0x03, 0x99, 0x38];

pub fn build_buy_exact_out_ix(
    payer: Pubkey,
    base_token_mint: Pubkey,
    quote_mint: Pubkey,
    creator: Pubkey,
    platform_config: Pubkey,
    amount_out: u64,
    maximum_amount_in: u64,
    share_fee_rate: u64,
) -> Instruction {
    // 1. 计算 authority PDA
    let program_id = PROGRAM;
    let authority = AUTHORITY;
    // 2. 计算 global_config PDA
    let global_config = GLOBAL_CONFIG;
    // 3. 计算 pool_state PDA
    let pool_state = get_pool_pda(&base_token_mint, &quote_mint).unwrap();
    // 4. 计算 event_authority PDA
    let event_authority = EVENT_AUTHORITY;
    // 5. 计算用户的 token 账户
    let user_base_token_account =
        spl_associated_token_account::get_associated_token_address(&payer, &base_token_mint);
    let user_quote_token_account = spl_associated_token_account::get_associated_token_address(
        &payer,
        &accounts::WSOL_TOKEN_ACCOUNT,
    );

    // 6. 计算池子的金库
    let base_vault_account = get_vault_pda(&pool_state, &base_token_mint).unwrap();
    let quote_vault_account = get_vault_pda(&pool_state, &accounts::WSOL_TOKEN_ACCOUNT).unwrap();

    let platform_vault = get_platform_vault(&platform_config, &quote_mint);
    let creator_vault = get_creator_vault(&creator, &quote_mint);

    // 7. SPL Token program
    let token_program = spl_token::ID;

    // BUY_EXACT_OUT_DISCRIMINATOR 需用idl里对应的8字节
    let mut data = vec![];

    data.extend_from_slice(&BUY_EXACT_OUT_DISCRIMINATOR); // 8字节
    data.extend_from_slice(&amount_out.to_le_bytes()); // u64
    data.extend_from_slice(&maximum_amount_in.to_le_bytes()); // u64
    data.extend_from_slice(&share_fee_rate.to_le_bytes()); // u64

    // 9. 构造账户元组
    let accounts = vec![
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(authority, false),
        AccountMeta::new_readonly(global_config, false),
        AccountMeta::new_readonly(platform_config, false),
        AccountMeta::new(pool_state, false),
        AccountMeta::new(user_base_token_account, false),
        AccountMeta::new(user_quote_token_account, false),
        AccountMeta::new(base_vault_account, false),
        AccountMeta::new(quote_vault_account, false),
        AccountMeta::new_readonly(base_token_mint, false),
        AccountMeta::new_readonly(quote_mint, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new_readonly(token_program, false), // base_token_program/quote_token_program
        AccountMeta::new_readonly(event_authority, false),
        AccountMeta::new_readonly(program_id, false),
        AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        AccountMeta::new(platform_vault, false),
        AccountMeta::new(creator_vault, false),
    ];

    Instruction {
        program_id,
        accounts,
        data,
    }
}
