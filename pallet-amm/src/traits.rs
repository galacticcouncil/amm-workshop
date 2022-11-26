pub trait Create<Assets> {
    type AssetId;
    type Error;
    fn create_share_asset(assets: Assets) -> Result<Self::AssetId, Self::Error>;
}

pub trait AccountIdFor<Assets> {
    type AccountId;
    type Error;

    fn create_account_id(assets: Assets) -> Result<Self::AccountId, Self::Error>;
}
