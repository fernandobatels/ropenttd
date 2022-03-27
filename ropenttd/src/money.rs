//! Currency and money related API

/// Money representation
#[derive(Debug, PartialEq)]
pub struct Money {
    /// Original money value, without exchange
    pub original: i64,
    /// Exchanged value
    pub value: i64,
    pub currency: Currency
}

impl Money {
    /// Create a new money instance
    pub fn new(original: i64, currency: Currency) -> Money {

        let value = currency.exchange(original);

        Money {
            original,
            value,
            currency
        }
    }

    /// Convert the money to another currency
    pub fn exchange(&self, currency: Currency) -> Money {
        Money::new(self.original, currency)
    }
}

/// Currency exchange settings and more
#[derive(Debug, PartialEq)]
pub struct Currency {
    pub exchange_rate: u16,
    pub name: &'static str
}

impl Currency {
    /// Convert the value to the currency
    pub fn exchange(&self, value: i64) -> i64 {
        self.exchange_rate as i64 * value
    }
}

/// Supported currencies
pub mod currencies {
    use crate::money::Currency;

    // Currencies from https://github.com/OpenTTD/OpenTTD/blob/61c6fd30557409e57d6c93d27ff4816ce3d60483/src/currency.cpp#L28

    /// British Pound
    pub const GBP: Currency = Currency { exchange_rate: 1, name: "GBP" };
    /// US Dollar
    pub const USD: Currency = Currency { exchange_rate: 2, name: "USD" };
    /// Euro
    pub const EUR: Currency = Currency { exchange_rate: 2, name: "EUR" };
    /// Japanese Yen
    pub const JPY: Currency = Currency { exchange_rate: 220, name: "JPY" };
    /// Austrian Schilling
    pub const ATS: Currency = Currency { exchange_rate: 27, name: "ATS" };
    /// Belgian Franc
    pub const BEF: Currency = Currency { exchange_rate: 81, name: "BEF" };
    /// Swiss Franc
    pub const CHF: Currency = Currency { exchange_rate: 2, name: "CHF" };
    /// Czech Koruna
    pub const CZK: Currency = Currency { exchange_rate: 41, name: "CZK" };
    /// Deutsche Mark
    pub const DEM: Currency = Currency { exchange_rate: 4, name: "DEM" };
    /// Danish Krona
    pub const DKK: Currency = Currency { exchange_rate: 11, name: "DKK" };
    /// Spanish Peseta
    pub const ESP: Currency = Currency { exchange_rate: 33, name: "ESP" };
    /// Finish Markka
    pub const FIM: Currency = Currency { exchange_rate: 12, name: "FIM" };
    /// French Franc
    pub const FRF: Currency = Currency { exchange_rate: 13, name: "FRF" };
    /// Greek Drachma
    pub const GRD: Currency = Currency { exchange_rate: 681, name: "GRD" };
    /// Hungarian Forint
    pub const HUF: Currency = Currency { exchange_rate: 378, name: "HUF" };
    /// Icelandic Krona
    pub const ISK: Currency = Currency { exchange_rate: 130, name: "ISK" };
    /// Italian Lira
    pub const ITL: Currency = Currency { exchange_rate: 3873, name: "ITL" };
    /// Dutch Gulden
    pub const NLG: Currency = Currency { exchange_rate: 4, name: "NLG" };
    /// Norwegian Krone
    pub const NOK: Currency = Currency { exchange_rate: 12, name: "NOK" };
    /// Polish Zloty
    pub const PLN: Currency = Currency { exchange_rate: 6, name: "PLN" };
    /// Romenian Leu
    pub const RON: Currency = Currency { exchange_rate: 5, name: "RON" };
    /// Russian Rouble
    pub const RUR: Currency = Currency { exchange_rate: 50, name: "RUR" };
    /// Slovenian Tolar
    pub const SIT: Currency = Currency { exchange_rate: 479, name: "SIT" };
    /// Swedish Krona
    pub const SEK: Currency = Currency { exchange_rate: 13, name: "SEK" };
    /// Turkish Lira
    pub const YTL: Currency = Currency { exchange_rate: 3, name: "YTL" };
    /// Slovak Kornuna
    pub const SKK: Currency = Currency { exchange_rate: 60, name: "SKK" };
    /// Brazilian Real
    pub const BRL: Currency = Currency { exchange_rate: 4, name: "BRL" };
    /// Estonian Krooni
    pub const EEK: Currency = Currency { exchange_rate: 31, name: "EEK" };
    /// Lithuanian Litas
    pub const LTL: Currency = Currency { exchange_rate: 4, name: "LTL" };
    /// South Korean Won
    pub const KRW: Currency = Currency { exchange_rate: 1850, name: "KRW" };
    /// South African Rand
    pub const ZAR: Currency = Currency { exchange_rate: 13, name: "ZAR" };
    /// Custom currency
    pub const CUSTOM: Currency = Currency { exchange_rate: 1, name: "CUSTOM" };
    /// Georgian Lari
    pub const GEL: Currency = Currency { exchange_rate: 3, name: "GEL" };
    /// Iranian Rial
    pub const IRR: Currency = Currency { exchange_rate: 4901, name: "IRR" };
    /// New Russian Ruble
    pub const RUB: Currency = Currency { exchange_rate: 80, name: "RUB" };
    /// Mexican Peso
    pub const MXN: Currency = Currency { exchange_rate: 24, name: "MXN" };
    /// New Taiwan Dollar
    pub const NTD: Currency = Currency { exchange_rate: 40, name: "NTD" };
    /// Chinese Renminbi
    pub const CNY: Currency = Currency { exchange_rate: 8, name: "CNY" };
    /// Hong Kong Dollar
    pub const HKD: Currency = Currency { exchange_rate: 10, name: "HKD" };
    /// Indian Rupee
    pub const INR: Currency = Currency { exchange_rate: 90, name: "INR" };
    /// Indonesian Rupiah
    pub const IDR: Currency = Currency { exchange_rate: 19, name: "IDR" };
    /// Malaysian Ringgit
    pub const MYR: Currency = Currency { exchange_rate: 5, name: "MYR" };
}

#[cfg(test)]
mod test {

    use crate::money::{Money, Currency, currencies};

    #[test]
    pub fn raw_exchanges() {
        assert_eq!(4, currencies::BRL.exchange(1));
        assert_eq!(80, currencies::RUB.exchange(1));
        assert_eq!(2, currencies::USD.exchange(1));
    }

    #[test]
    pub fn money_exchanges() {
        let dolar = Money::new(1, currencies::USD);
        let real = dolar.exchange(currencies::BRL);

        assert_eq!(Money {
            original: 1,
            value: 4,
            currency: Currency {
                exchange_rate: 4,
                name: "BRL"
            },
        }, real);
    }
}
