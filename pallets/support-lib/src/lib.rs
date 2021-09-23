#![cfg_attr(not(feature = "std"), no_std)]

use primitive_types::H160;
pub trait RevenueWhiteList {
    fn is_in_white_list(address: H160) -> bool;
}
