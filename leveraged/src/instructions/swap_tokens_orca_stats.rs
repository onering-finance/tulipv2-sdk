use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct NewSerumSwap<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    // ensure global account of leveraged farm matches the given global account
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    // #[account(mut)]
    // pub user_farm_manager: Loader<'info, UserFarmManager>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    // i believe this is the wallet that will receive funds??
    // or is this the wallet that will contain the price coin
    #[account(mut)]
    pub pc_wallet: AccountInfo<'info>,
    pub market: MarketAccounts<'info>,
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub dex_program: AccountInfo<'info>,
    pub vault_signer: AccountInfo<'info>,
}

// Market accounts are the accounts used to place orders against the dex minus
// common accounts, i.e., program ids, sysvars, and the `pc_wallet`.
#[derive(Accounts)]
pub struct MarketAccounts<'info> {
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    // The `spl_token::Account` that funds will be taken from, i.e., transferred
    // from the user into the market's vault.
    //
    // For bids, this is the base currency. For asks, the quote.
    #[account(mut)]
    pub order_payer_token_account: AccountInfo<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    // PDA owner of the DEX's token accounts for base + quote currencies.
    pub vault_signer: AccountInfo<'info>,
    // User wallets.
    #[account(mut)]
    pub coin_wallet: AccountInfo<'info>,
}

pub fn swap_tokens_orca_stats<'info>(
    accounts: NewSerumSwap<'info>,
    serum_fee_recipient: &AccountInfo<'info>,
    lending_market_account: &AccountInfo<'info>,
    lending_market_authority: &AccountInfo<'info>,
    lending_program: &AccountInfo<'info>,
    lp_mint: &AccountInfo<'info>,
    position_info_account: &AccountInfo<'info>,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("swap_tokens_orca_stats")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());

    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new(serum_fee_recipient.key(), false));
    accounts.push(AccountMeta::new_readonly(
        lending_market_account.key(),
        false,
    ));
    accounts.push(AccountMeta::new_readonly(
        lending_market_authority.key(),
        false,
    ));
    accounts.push(AccountMeta::new_readonly(lending_program.key(), false));
    accounts.push(AccountMeta::new(lp_mint.key(), false));
    accounts.push(AccountMeta::new(position_info_account.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts,
        data: ix_data,
    })
}
