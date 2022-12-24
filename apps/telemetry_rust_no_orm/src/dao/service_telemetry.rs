use crate::DB_POOL;
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct MultipleKeyValue {
    pub n_pcs: i64,
    pub value: String,
}

pub async fn store_data_db(data: HashMap<String, HashMap<String, Option<String>>>) -> bool {
    let pool = DB_POOL.get().await;
    let tx = pool.begin().await;
    let mut tx = tx.unwrap();

    let result = sqlx::query_file!("src/dao/sql/max_req_id.sql")
        .fetch_one(&mut tx)
        .await;

    let req_id = result.unwrap().maximum.unwrap_or(0) + 1;
    println!("{}", req_id);
    let date_time = Utc::now().naive_utc();

    for (code, key_value_map) in data.into_iter() {
        for (key, value) in key_value_map.into_iter() {
            let result = sqlx::query_file!(
                "src/dao/sql/insert.sql",
                req_id,
                code,
                key,
                value,
                date_time
            )
            .execute(&mut tx)
            .await;
            if result.is_ok() {
                result.unwrap();
            } else {
                println!("{:?}", result.err());
            }
        }
    }
    let res = tx.commit().await;
    println!("{:?}", res);

    return true;
}

pub async fn telemetry_retrieve_num_comp_ram() -> Result<Vec<MultipleKeyValue>, sqlx::Error> {
    let pool = DB_POOL.get().await;

    let result = sqlx::query_file!(
        "src/dao/sql/query.sql",
        "hw_mem_total_memory",
        "hw_cpu_idientifier"
    )
    .fetch_all(pool)
    .await;

    let result = result.unwrap();

    let mut res: Vec<MultipleKeyValue> = vec![];
    for item in result {
        let miniRes = MultipleKeyValue {
            n_pcs: item.n_pcs.unwrap(),
            value: item.value.unwrap(),
        };
        res.push(miniRes);
    }

    return Ok(res);
}

pub async fn delete_all_dao() -> bool {
    let pool = DB_POOL.get().await;
    let result = sqlx::query!("delete from telemetry_no_orm").execute(pool).await;
    return true;
}
