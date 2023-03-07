use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::BillableItem;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, BorshDeserialize, BorshSerialize)]
pub enum ServiceCommand {
    DecentralizedNotification {
        recipient: Pubkey,
        validator: Pubkey,
        service_id: String,
    },
    TxEmail {
        validator: Pubkey,
        from: String,
        reply_to: String,
        to: String,
        items: Vec<BillableItem>,
    },
    NewsletterEmail {
        validator: Pubkey,
        from: String,
        reply_to: String,
        to: String,
        notification: String,
        service_id: String, //HTML String,
    },
}
