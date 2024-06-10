use std::clone;
// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, AccountId};

#[near(serializers = [borsh, json])]
pub struct Verify_nft{  //실제 nft로 ! 
    dumb:String,
}

// 변경! 