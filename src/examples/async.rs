extern crate cql_bindgen;

mod examples_util;
use examples_util::*;

use cql_bindgen::*;

static NUM_CONCURRENT_REQUESTS:usize = 1000;

fn insert_into_async(session: &mut CassSession, key: &str) {
    unsafe {
        let query = str2ref("INSERT INTO async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
        let futures = &mut Vec::with_capacity(NUM_CONCURRENT_REQUESTS);

        for i in 0..NUM_CONCURRENT_REQUESTS {
            let statement = cass_statement_new(query, 6);
            let key = &format!("{}{}", key, i);
            let bbool = if i % 2 == 0 { cass_true } else { cass_false };

            cass_statement_bind_string(statement, 0, str2ref(key));
            cass_statement_bind_bool(statement, 1, bbool);
            cass_statement_bind_float(statement, 2, i as f32 / 2.0f32);
            cass_statement_bind_double(statement, 3, i as f64 / 200.0);
            cass_statement_bind_int32(statement, 4, i as i32 * 10);
            cass_statement_bind_int64(statement, 5, i as i64 * 100);

            futures.push(cass_session_execute(session, statement));
            cass_statement_free(statement);
        }

        for i in 0..NUM_CONCURRENT_REQUESTS {
            let future = &mut*futures[i];
            cass_future_wait(future);

            match cass_future_error_code(future) {
                CASS_OK => {}
                _ => {
                    print_error(future);
                }
            }
            cass_future_free(future);
        }
    }
}

pub fn main() {
    unsafe {
        let cluster = create_cluster().unwrap();
        let session = &mut*cass_session_new();

        match connect_session(session, cluster) {
            Ok(()) => {
                execute_query(session,
                              "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': \
                               'SimpleStrategy', 'replication_factor': '3' };")
                    .unwrap();

                execute_query(session,
                              "CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl \
                               double,i32 int, i64 bigint, PRIMARY KEY (key));")
                    .unwrap();

                execute_query(session, "USE examples").unwrap();
                insert_into_async(session, "test");

                let close_future = cass_session_close(session);
                cass_future_wait(close_future);
                cass_future_free(close_future);
                println!("Success inserting test");
            }

            rc => println!("Error: {:?}", rc),
        }
        cass_cluster_free(cluster);
        cass_session_free(session);
    }
}