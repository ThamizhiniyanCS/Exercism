use time::{OffsetDateTime, PrimitiveDateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let offset_datetime =
        OffsetDateTime::from_unix_timestamp(start.assume_utc().unix_timestamp() + 1000000000)
            .unwrap();

    PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time())
}
