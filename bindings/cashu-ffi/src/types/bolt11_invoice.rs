use std::{ops::Deref, str::FromStr, sync::Arc};

use cashu::Bolt11Invoice as Bolt11InvoiceSdk;

use crate::{error::Result, Amount};

pub struct Bolt11Invoice {
    inner: Bolt11InvoiceSdk,
}

impl Deref for Bolt11Invoice {
    type Target = Bolt11InvoiceSdk;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Bolt11Invoice {
    pub fn new(bolt11: String) -> Result<Self> {
        Ok(Self {
            inner: Bolt11InvoiceSdk::from_str(&bolt11)?,
        })
    }

    pub fn as_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn amount(&self) -> Option<Arc<Amount>> {
        self.inner
            .amount_milli_satoshis()
            .map(|a| Arc::new(Amount::from_msat(a)))
    }
}
