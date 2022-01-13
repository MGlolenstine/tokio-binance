use crate::builder::ParamBuilder;
use crate::param::Parameters;
use crate::types::*;
use reqwest::{Client, Url};

/// Client for dealing with withdrawals and sub accounts.
#[derive(Clone)]
pub struct WithdrawalClient {
    pub(super) api_key: String,
    pub(super) secret_key: String,
    pub(super) url: Url,
    pub(super) client: Client,
}

impl WithdrawalClient {
    /// Creates new client instance.
    /// # Example
    ///
    /// ```no_run
    /// use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn connect<A, S, U>(api_key: A, secret_key: S, url: U) -> crate::error::Result<Self>
    where
        A: Into<String>,
        S: Into<String>,
        U: Into<String>,
    {
        Ok(Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            url: url.into().parse::<Url>()?,
            client: Client::new(),
        })
    }
    /// Submit a withdraw request.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .withdraw("BNB", "<public-address>", 5.00)
    ///     //optional: Secondary address identifier for coins like XRP,XMR etc.
    ///     .with_address_tag("<tag>")
    ///     // optional: Description of the address.
    ///     .with_name("<description>")
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn withdraw<'a>(
        &self,
        asset: &'a str,
        address: &'a str,
        amount: f64,
    ) -> ParamBuilder<'a, '_, WithdrawParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
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
            Some(secret_key),
        )
    }
    /// Fetch deposit history.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::hours(23);
    ///
    /// let response = client
    ///     .get_deposit_history()
    ///     // optional: filter by asset; gets all assets by default.
    ///     .with_asset("BNB")
    ///     // optional: 0(0:pending,6: credited but cannot withdraw, 1:success)
    ///     .with_status(1)
    ///     // optional: get deposits from; gets the most recent deposits by default.
    ///     .with_start_time(start)
    ///     // optional: get deposits until; default is now.
    ///     .with_end_time(end)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_deposit_history(&self) -> ParamBuilder<'_, '_, DepositHistoryParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/depositHistory.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch withdraw history.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::hours(23);
    ///
    /// let response = client
    ///     .get_withdraw_history()
    ///     // optional: filter by asset; gets all assets by default.
    ///     .with_asset("BNB")
    ///     // optional: 0(0:Email Sent,1:Cancelled 2:Awaiting Approval 3:Rejected 4:Processing 5:Failure 6Completed)
    ///     .with_status(6)
    ///     // optional: get deposits from; gets the most recent deposits by default.
    ///     .with_start_time(start)
    ///     // optional: get deposits until; default is now.
    ///     .with_end_time(end)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_withdraw_history(&self) -> ParamBuilder<'_, '_, WithdrawHistoryParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/withdrawHistory.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch deposit address.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_deposit_address("BNB")
    ///     // optional: Boolean.
    ///     .with_status(true)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_deposit_address<'a>(
        &self,
        asset: &'a str,
    ) -> ParamBuilder<'a, '_, DepositAddressParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/depositAddress.html").unwrap();

        ParamBuilder::new(
            Parameters {
                asset: Some(asset),
                ..Parameters::default()
            },
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch account status detail.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_account_status()
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_account_status(&self) -> ParamBuilder<'_, '_, AccountStatusParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/accountStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch system status.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_system_status()
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_system_status(&self) -> ParamBuilder<'_, '_, SystemStatusParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/systemStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch account api trading status detail.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_api_status()
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_api_status(&self) -> ParamBuilder<'_, '_, ApiStatusParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/apiTradingStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch small amounts of assets exchanged BNB records.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_dustlog()
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_dustlog(&self) -> ParamBuilder<'_, '_, DustlogParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/userAssetDribbletLog.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch trade fee.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_trade_fee()
    ///     // optional: filter by symbol; gets all symbols by default.
    ///     .with_symbol("BNBUSDT")
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_trade_fee(&self) -> ParamBuilder<'_, '_, TradeFeeParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/tradeFee.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch asset detail.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_asset_detail()
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_asset_detail(&self) -> ParamBuilder<'_, '_, AssetDetailParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/assetDetail.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch sub account list.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_sub_accounts()
    ///     // optional: Sub-account email.
    ///     .with_email("<email>")
    ///     // optional: Sub-account status: enabled or disabled.
    ///     .with_status("enabled")
    ///     // optional: default value: 1.
    ///     .with_page(2)
    ///     // optional: limit the amount of sub accounts; default 500.
    ///     .with_limit(100)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_sub_accounts(&self) -> ParamBuilder<'_, '_, SubAccountParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/sub-account/list.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Fetch transfer history list
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::days(99);
    ///
    /// let response = client
    ///     .get_transfer_history("<email>")
    ///     // optional: get history from; default return the history with in 100 days
    ///     .with_start_time(start)
    ///     // optional: get history until; default is now.
    ///     .with_end_time(end)
    ///     // optional: default value: 1.
    ///     .with_page(2)
    ///     // optional: limit the amount of sub accounts; default 500.
    ///     .with_limit(100)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_transfer_history<'a>(
        &self,
        email: &'a str,
    ) -> ParamBuilder<'a, '_, SubAccountTranferParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url
            .join("/wapi/v3/sub-account/transfer/history.html")
            .unwrap();

        ParamBuilder::new(
            Parameters {
                email: Some(email),
                ..Parameters::default()
            },
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Execute sub-account transfer.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .tranfer_sub_account("<from_email>", "<to_email>", "BNB", 5.00)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tranfer_sub_account<'a>(
        &self,
        from_email: &'a str,
        to_email: &'a str,
        asset: &'a str,
        amount: f64,
    ) -> ParamBuilder<'a, '_, TransferSubAccountParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
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
            Some(secret_key),
        )
    }
    /// Fetch sub-account assets.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_sub_account_assets("<email>")
    ///     // optional: filter by symbol; gets all symbols by default.
    ///     .with_symbol("BNBUSDT")
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_sub_account_assets<'a>(
        &self,
        email: &'a str,
    ) -> ParamBuilder<'a, '_, SubAccountAssetParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/wapi/v3/sub-account/assets.html").unwrap();

        ParamBuilder::new(
            Parameters {
                email: Some(email),
                ..Parameters::default()
            },
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Convert dust assets to BNB.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     // restricted to one asset at a time.
    ///     .dust_transfer("ETH")
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn dust_transfer<'a>(&self, asset: &'a str) -> ParamBuilder<'a, '_, DustTransferParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/sapi/v1/asset/dust").unwrap();

        ParamBuilder::new(
            Parameters {
                asset: Some(asset),
                ..Parameters::default()
            },
            client.post(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Query asset dividend record.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WithdrawalClient, BINANCE_US_URL};
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = WithdrawalClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::days(99);
    ///
    /// let response = client
    ///     .get_asset_dividends()
    ///     // optional: filter by asset; gets all assets by default.
    ///     .with_asset("BNB")
    ///     // optional: get records from; gets recent records by default.
    ///     .with_start_time(start)
    ///     // optional: get records until; default is now.
    ///     .with_end_time(end)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_asset_dividends(&self) -> ParamBuilder<'_, '_, AssetDividendParams> {
        let Self {
            api_key,
            secret_key,
            url,
            client,
        } = self;
        let url = url.join("/sapi/v1/asset/assetDividend").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
}
