use time::Duration;
use time::PrimitiveDateTime as DateTime;

pub fn after(start: DateTime) -> DateTime {
    return start + Duration::seconds(1000000000);
}
