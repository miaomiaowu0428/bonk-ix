use solana_sdk::pubkey::Pubkey;

use crate::bonk::ixs::accounts::PROGRAM;

pub mod buy;
pub mod buy_exact_out;
pub mod sell;

pub mod seeds {
    pub const POOL_SEED: &[u8] = b"pool";
    pub const POOL_VAULT_SEED: &[u8] = b"pool_vault";
}

pub mod accounts {
    use solana_sdk::{pubkey, pubkey::Pubkey};

    pub const AUTHORITY: Pubkey = pubkey!("WLHv2UAZm6z4KyaaELi5pjdbJh6RESMva1Rnn8pJVVh");
    pub const GLOBAL_CONFIG: Pubkey = pubkey!("6s1xP3hpbAfFoNtUNF8mfHsjr2Bd97JxFJRWLbL6aHuX");
    pub const BONK_CONFIG: Pubkey = pubkey!("FfYek5vEz23cMkWsdJwG2oa6EphsvXSHrGpdALN4g6W1");
    pub const LAUNCHLAB_CONFIG: Pubkey = pubkey!("4Bu96XjU84XjPDSpveTVf6LYGCkfW5FK7SNkREWcEfV4");
    pub const TOKEN_PROGRAM: Pubkey = spl_token::ID;
    pub const EVENT_AUTHORITY: Pubkey = pubkey!("2DPAtwB8L12vrMRExbLuyGnC7n2J5LNoZQSejeQGpwkr");
    pub const WSOL_TOKEN_ACCOUNT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
    pub const PROGRAM: Pubkey = pubkey!("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj");

    pub const PLATFORM_FEE_RATE: u128 = 100; // 1%
    pub const PROTOCOL_FEE_RATE: u128 = 25; // 0.25%
    pub const SHARE_FEE_RATE: u128 = 0; // 0%
}

pub fn get_pool_pda(base_mint: &Pubkey, quote_mint: &Pubkey) -> Option<Pubkey> {
    let seeds: &[&[u8]; 3] = &[seeds::POOL_SEED, base_mint.as_ref(), quote_mint.as_ref()];
    let program_id: &Pubkey = &accounts::PROGRAM;
    let pda: Option<(Pubkey, u8)> = Pubkey::try_find_program_address(seeds, program_id);
    pda.map(|pubkey| pubkey.0)
}

pub fn get_vault_pda(pool_state: &Pubkey, mint: &Pubkey) -> Option<Pubkey> {
    let seeds: &[&[u8]; 3] = &[seeds::POOL_VAULT_SEED, pool_state.as_ref(), mint.as_ref()];
    let program_id: &Pubkey = &accounts::PROGRAM;
    let pda: Option<(Pubkey, u8)> = Pubkey::try_find_program_address(seeds, program_id);
    pda.map(|pubkey| pubkey.0)
}

pub fn get_platform_vault(platform_config: &Pubkey, mint: &Pubkey) -> Pubkey {
    let (vault, _) =
        Pubkey::find_program_address(&[platform_config.as_ref(), mint.as_ref()], &PROGRAM);
    vault
}
pub fn get_creator_vault(creator: &Pubkey, mint: &Pubkey) -> Pubkey {
    let (vault, _) = Pubkey::find_program_address(&[creator.as_ref(), mint.as_ref()], &PROGRAM);
    vault
}

// {
//   "epoch": {
//     "type": "u64",
//     "data": "842"
//   },
//   "auth_bump": {
//     "type": "u8",
//     "data": 250
//   },
//   "status": {
//     "type": "u8",
//     "data": 0
//   },
//   "base_decimals": {
//     "type": "u8",
//     "data": 6
//   },
//   "quote_decimals": {
//     "type": "u8",
//     "data": 9
//   },
//   "migrate_type": {
//     "type": "u8",
//     "data": 0
//   },
//   "supply": {
//     "type": "u64",
//     "data": "1000000000000000"
//   },
//   "total_base_sell": {
//     "type": "u64",
//     "data": "793100000000000"
//   },
//   "virtual_base": {
//     "type": "u64",
//     "data": "1073025605596382"
//   },
//   "virtual_quote": {
//     "type": "u64",
//     "data": "30000852951"
//   },
//   "real_base": {
//     "type": "u64",
//     "data": "0"
//   },
//   "real_quote": {
//     "type": "u64",
//     "data": "8"
//   },
//   "total_quote_fund_raising": {
//     "type": "u64",
//     "data": "85000000000"
//   },
//   "quote_protocol_fee": {
//     "type": "u64",
//     "data": "29674"
//   },
//   "platform_fee": {
//     "type": "u64",
//     "data": "0"
//   },
//   "migrate_fee": {
//     "type": "u64",
//     "data": "0"
//   },
//   "vesting_schedule": {
//     "type": {
//       "defined": {
//         "name": "VestingSchedule"
//       }
//     },
//     "data": {
//       "total_locked_amount": {
//         "type": "u64",
//         "data": "0"
//       },
//       "cliff_period": {
//         "type": "u64",
//         "data": "0"
//       },
//       "unlock_period": {
//         "type": "u64",
//         "data": "0"
//       },
//       "start_time": {
//         "type": "u64",
//         "data": "0"
//       },
//       "allocated_share_amount": {
//         "type": "u64",
//         "data": "0"
//       }
//     }
//   },
//   "global_config": {
//     "type": "pubkey",
//     "data": "6s1xP3hpbAfFoNtUNF8mfHsjr2Bd97JxFJRWLbL6aHuX"
//   },
//   "platform_config": {
//     "type": "pubkey",
//     "data": "4Bu96XjU84XjPDSpveTVf6LYGCkfW5FK7SNkREWcEfV4"
//   },
//   "base_mint": {
//     "type": "pubkey",
//     "data": "13TymMBxgdLTUYSqioemwnRapQwp8apsw8zhUYDMsrNJ"
//   },
//   "quote_mint": {
//     "type": "pubkey",
//     "data": "So11111111111111111111111111111111111111112"
//   },
//   "base_vault": {
//     "type": "pubkey",
//     "data": "2aatWywHaQUVNra2vU7xrzmeNjrxWhwCfc5ZVUNWzZLZ"
//   },
//   "quote_vault": {
//     "type": "pubkey",
//     "data": "3JvKgTNkFDBDacDQgKMME6nNDTUnE5V1Pd131eQHk6hp"
//   },
//   "creator": {
//     "type": "pubkey",
//     "data": "2FX2KigeEk7BVFmroVEepYD1KVgjohvUjnTv6nRzFEUA"
//   },
//   "token_program_flag": {
//     "type": "u8",
//     "data": 0
//   },
//   "amm_creator_fee_on": {
//     "type": {
//       "defined": {
//         "name": "AmmCreatorFeeOn"
//       }
//     },
//     "data": {
//       "QuoteToken": {}
//     }
//   },
//   "padding": {
//     "type": {
//       "array": [
//         "u8",
//         62
//       ]
//     },
//     "data": [
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0,
//       0
//     ]
//   }
// }
