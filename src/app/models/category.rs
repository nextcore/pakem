use pakem_derive::PakemModel;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, PakemModel, Clone)]
#[table("categories")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}
