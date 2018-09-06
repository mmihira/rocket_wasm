use timestamp::{TimeStamp};
use ::schema;
use ::schema::candles;
use ::schema::candles::dsl::candles as candles_dsl;
use diesel::prelude::{PgConnection};
use diesel::{Identifiable, ExpressionMethods, QueryDsl, RunQueryDsl, BoolExpressionMethods};
use diesel::result::Error as DieselError;

#[table_name="candles"]
#[primary_key(period, start_time, end_time)]
#[derive(Debug, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct Candle {
    pub period: i32,
    pub start_time: TimeStamp,
    pub end_time: TimeStamp,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub vol: f32,
    pub rsi: f32,
    pub sma_9: f32,
    pub sma_12: f32,
    pub sma_26: f32,
    pub ema_9: f32,
    pub ema_12: f32,
    pub ema_26: f32
}

impl Candle{
    pub fn save_as_new(&self, conn: &PgConnection) ->  Result<Candle, DieselError> {
        ::diesel::insert_into(candles::table)
            .values(self)
            .get_result(conn)
    }

    pub fn update(&self, conn: &PgConnection) -> Result<Candle, DieselError> {
        ::diesel::update(self)
            .set(self)
            .get_result(conn)
    }

    pub fn save_or_update(&self, conn: &PgConnection) -> Result<Candle, DieselError> {
        let exists = candles_dsl.find(self.id()).get_result::<Candle>(conn);

        match exists {
            Ok(existing) => self.update(conn),
            Err(_) => self.save_as_new(conn)
        }
    }
}
