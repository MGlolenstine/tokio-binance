use reqwest::{Url, Client};
use crate::param::{
    Parameters, 
};
use crate::builder::ParamBuilder;
use crate::types::*;

#[derive(Clone)]
pub struct WithdrawClient {
    pub(super) api_key: String,
    pub(super) secret_key: String,
    pub(super) url: Url,
    pub(super) client: Client
}

impl WithdrawClient {
    pub fn connect<T: Into<String>>(api_key: T, secret_key: T, url: T) -> crate::error::Result<Self> {
        Ok(Self {
            api_key: api_key.into(), 
            secret_key: secret_key.into(),
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
    }

    pub fn withdraw<'a>(&self, asset: &'a str, address: &'a str, amount: f64) -> ParamBuilder<'a, '_, WithdrawParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/withdraw.html").unwrap();

        ParamBuilder::new(
            Parameters { 
                asset: Some(asset), 
                address: Some(address),
                amount: Some(amount),
                ..Parameters::default() 
            },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn deposit_history(&self) -> ParamBuilder<'_, '_, DepositHistoryParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/depositHistory.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn withdraw_history(&self) -> ParamBuilder<'_, '_, WithdrawHistoryParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/withdrawHistory.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_deposit_address<'a>(&self, asset: &'a str) -> ParamBuilder<'a, '_, DepositAddressParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/depositAddress.html").unwrap();

        ParamBuilder::new(
            Parameters { asset: Some(asset), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_account_status(&self) -> ParamBuilder<'_, '_, AccountStatusParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/accountStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_system_status(&self) -> ParamBuilder<'_, '_, SystemStatusParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/systemStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }
    pub fn get_api_status(&self) -> ParamBuilder<'_, '_, ApiStatusParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/apiTradingStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_dustlog(&self) -> ParamBuilder<'_, '_, DustlogParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/userAssetDribbletLog.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_trade_fee(&self) -> ParamBuilder<'_, '_, TradeFeeParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/tradeFee.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_asset_detail(&self) -> ParamBuilder<'_, '_, AssetDetailsParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/assetDetail.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_sub_accounts(&self) -> ParamBuilder<'_, '_, SubAccountParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/sub-account/list.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_sub_account_tranfers<'a>(&self, email: &'a str) -> ParamBuilder<'a, '_, SubAccountTranferParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/sub-account/transfer/history.html").unwrap();

        ParamBuilder::new(
            Parameters { email: Some(email), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn tranfer_sub_account<'a>(&self, 
        from_email: &'a str, 
        to_email: &'a str, 
        asset: &'a str, 
        amount: f64
    ) -> ParamBuilder<'a, '_, TransferSubAccountParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/sub-account/transfer.html").unwrap();

        ParamBuilder::new(
            Parameters { 
                from_email: Some(from_email), 
                to_email: Some(to_email), 
                asset: Some(asset), 
                amount: Some(amount), 
                ..Parameters::default() 
            },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_sub_account_assets<'a>(&self, email: &'a str) -> ParamBuilder<'a, '_, SubAccountAssetParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/sub-account/assets.html").unwrap();

        ParamBuilder::new(
            Parameters { email: Some(email), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn dust_transfer<'a>(&self, asset: &'a str) -> ParamBuilder<'a, '_, DustTransferParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/sapi/v1/asset/dust").unwrap();

        ParamBuilder::new(
            Parameters { asset: Some(asset), ..Parameters::default() },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_asset_dividends(&self) -> ParamBuilder<'_, '_, AssetDividendParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/sapi/v1/asset/assetDividend").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }
}