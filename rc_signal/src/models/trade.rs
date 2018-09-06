use ::schema;
use ::schema::trades;
use diesel::prelude::{PgConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use ::schema::trades::dsl::trades as trades_dsl;
use timestamp::TimeStamp;

#[derive(Debug, Identifiable,  Queryable)]
pub struct Trade {
    pub id: i32,
    pub tid: i32,
    pub timestamp: TimeStamp,
    pub vol: f32,
    pub price: f32
}

#[table_name="trades"]
#[derive(Debug, Insertable)]
pub struct NewTrade {
    pub tid: i32,
    pub timestamp: i64,
    pub vol: f32,
    pub price: f32
}

impl Trade {
    pub fn in_timestamp_range(conn: &PgConnection, start: TimeStamp, end: TimeStamp) -> Vec<Trade> {
        trades_dsl.filter(trades::timestamp.between(start, end))
            .get_results::<Trade>(conn)
            .unwrap()
    }
}

impl NewTrade {
    pub fn save_as_new(&self, conn: &PgConnection) ->  Result<Trade, ::diesel::result::Error> {
        ::diesel::insert_into(trades::table)
            .values(self)
            .get_result(conn)
    }
}


