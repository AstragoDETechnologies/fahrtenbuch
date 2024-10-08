use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
// #[sqlx(type_name = "fuel_type", rename_all = "lowercase")]
#[repr(i32)]
pub enum FuelType {
    Gasoline = 0,
    Diesel = 1,
    Electric = 2,
    Other = 3,
}

impl TryFrom<i32> for FuelType {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FuelType::Gasoline),
            1 => Ok(FuelType::Diesel),
            2 => Ok(FuelType::Electric),
            3 => Ok(FuelType::Other),
            _ => Err("No associated value".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
// #[sqlx(type_name = "gasoline_type", rename_all = "lowercase")]
#[repr(i32)]
pub enum GasolineType {
    Super = 0,
    SuperPlus = 1,
    SuperE10 = 2,
}

impl TryFrom<i32> for GasolineType {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GasolineType::Super),
            1 => Ok(GasolineType::SuperPlus),
            2 => Ok(GasolineType::SuperE10),
            _ => Err("No associated value".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelPrice {
    id: i32,
    price: f32,
    date: chrono::DateTime<chrono::Utc>,
    fuel_type: FuelType,
    gasoline_type: Option<GasolineType>,
}

impl TryFrom<&PgRow> for FuelPrice {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &PgRow) -> Result<Self, Self::Error> {
        let fuel_price = FuelPrice {
            id: value.try_get("id")?,
            price: value.try_get("price")?,
            date: value.try_get("date")?,
            fuel_type: FuelType::try_from(value.try_get::<i32, &str>("fuel_type").unwrap())?,
            // Explaination: https://chatgpt.com/share/66f9222e-be98-8000-8378-57cdb6ec51a8
            gasoline_type: value
                .try_get::<Option<i32>, _>("gasoline_type")?
                .map(GasolineType::try_from)
                .transpose()?,
        };

        Ok(fuel_price)
    }
}

// ===== Utility Structs

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertFuelPrice {
    pub price: f32,
    pub fuel_type: FuelType,
    pub gasoline_type: Option<GasolineType>,
}
