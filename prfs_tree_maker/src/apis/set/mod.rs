mod grow;
mod leaves;

use self::grow::grow_tree;
use crate::TreeMakerError;
use prfs_db_interface::{Account, Database, Node};
use rust_decimal::Decimal;

pub struct SetType {
    pub table_label: String,
    pub query: String,
}

lazy_static::lazy_static! {
    static ref WEI_200: SetType = SetType {
        table_label: "balances_20230327".to_string(),
        query: "wei >= 277200000000000000 and wei < 277300000000000000".to_string(),
    };
}

pub async fn run(db: Database) -> Result<(), TreeMakerError> {
    leaves::make_leaves(db, &*WEI_200).await?;

    Ok(())
}
