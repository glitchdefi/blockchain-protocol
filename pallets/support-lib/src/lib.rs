use sp_core::H160;
pub trait RevenueWhiteList {
    fn is_in_white_list(address: H160) -> bool;
}