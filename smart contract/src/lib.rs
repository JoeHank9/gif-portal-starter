use anchor_lang::prelude::*;

declare_id!("2zJAMh5JQep9Tw6cayGk4pPnNjR3vm3Cg4orQs5inU3G");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_profiles = 0;
    base_account.total_subs = 0;
    Ok(())
  }

  // The function to create profile creators. We also reference the user from the Context
  pub fn add_profile(ctx: Context<AddProfile>, nickname: String, twitter: String, instagram: String, youtube: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let profile = ProfileStruct {
      nickname: nickname.to_string(),
      user_address: *user.to_account_info().key,
      twitter: twitter.to_string(),
      instagram: instagram.to_string(),
      youtube: youtube.to_string()
    };
		
	// Add it to the profile_list vector.
    base_account.profile_list.push(profile);
    base_account.total_profiles += 1;
    Ok(())
  }

  // The function to create profile creators. We also reference the user from the Context
  pub fn add_subscription(ctx: Context<AddProfile>, id: u64, price: u64, description: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    assert!(base_account.total_subs < 5, "You can only add 5 subs");
	// Build the struct.
    let sub = SubscriptionStruct {
      user_address: *user.to_account_info().key,
      id: id,
      price: price,
      description: description.to_string()
    };
		
	// Add it to the profile_list vector.
    base_account.subscription_list.push(sub);
    base_account.total_subs += 1;
    Ok(())
  }

}

// Add the signer who calls the AddProfile method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddProfile<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProfileStruct {
    pub user_address: Pubkey,
    pub nickname: String,
    pub twitter: String,
    pub instagram: String,
    pub youtube: String
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SubscriptionStruct {
    pub user_address: Pubkey,
    pub id: u64,
    pub price: u64,
    pub description: String
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MySubscriptionStruct {
    pub user_address: Pubkey,
    pub total_subscriptions: u64,
    pub subscriptions: Vec<SubscriptionStruct>
}


// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_profiles: u64,
    pub total_subs: u8,
    // Attach a Vector of type ItemStruct to the account.
    pub profile_list: Vec<ProfileStruct>,
    pub mysubscription_list: Vec<MySubscriptionStruct>,
    pub subscription_list: Vec<SubscriptionStruct>
}

