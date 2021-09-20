use crate::AssetId;

pub enum Currency {
    Native,
    Wrapped(Token),
}

pub enum Token {
    ETH,
    BTC,
    // Add more coins here....,
}

impl Currency {
    pub fn asset_id(self) -> AssetId {
        match self {
            Currency::Native => 0,
            // FIXME: Make it cleaner
            Currency::Wrapped(token) => match token {
                Token::ETH => 1,
                Token::BTC => 2,
            },
        }
    }
}
