use crate::DB_POOL;
use chrono::Utc;
use serde::{Serialize, de::Error};
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

    let result = sqlx::query_as("select max(request_id) from telemetry_no_orm")
        .fetch_one(&mut tx)
        .await;


   let result:(i32,) = result.unwrap_or((0,)) ;  



    let req_id = result.0 + 1;
    let date_time = Utc::now().naive_utc();

    for (code, key_value_map) in data.into_iter() {
        for (key, value) in key_value_map.into_iter() {
            let result = sqlx::query(
                "INSERT INTO telemetry_no_orm (request_id, code, property, value, created_at) VALUES($1, $2, $3, $4, $5)"
                            ).bind(req_id).bind(
                                &code).bind(
                                key).bind(
                                value).bind(
                                date_time)
                
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

    return true;
}

pub async fn telemetry_retrieve_num_comp_ram() -> Result<Vec<MultipleKeyValue>, sqlx::Error> {
    let pool = DB_POOL.get().await;

    let result= sqlx::query_as(
        "       SELECT
        telemetry_no_orm.value,
        COUNT(telemetry_no_orm.value) AS n_pcs
        FROM
        telemetry_no_orm
        WHERE
        telemetry_no_orm.property = $1
        AND telemetry_no_orm.request_id IN (
            SELECT
            MAX(telemetry_no_orm.request_id)
            FROM
            telemetry_no_orm
            WHERE
            telemetry_no_orm.property = $2
            GROUP BY
            property,
            value
        )
        GROUP BY
        telemetry_no_orm.code,
        telemetry_no_orm.property,
        telemetry_no_orm.value
        ORDER BY
        telemetry_no_orm.property ASC"
    ).bind("hw_mem_total_memory")
     .bind("hw_cpu_idientifier")
    .fetch_all(pool)
    .await;


    println!("{:?}", result);
    
let result: Vec<(String, i64)> = result.unwrap();

    let mut res: Vec<MultipleKeyValue> = vec![];
    for item in result {
        let miniRes = MultipleKeyValue {
            n_pcs: item.1,
            value: item.0,
        };
        res.push(miniRes);
    }

    return Ok(res);
}

pub async fn delete_all_dao() -> bool {
    let pool = DB_POOL.get().await;
    let result = sqlx::query("delete from telemetry_no_orm").execute(pool).await;
    return true;
}
