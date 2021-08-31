// storage for this module runtime

decl_storage! {
    trait Store for Module<T: Trait> as Template {
        TotalSupply get(total_supply): u64 = 88_888_888;
        BalanceOf get(balance_of): map T::AccountId => u64;
    }
}