//! Utils for stock market transfer-hook program

use crate::errors::StockMarketError;
use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike, Utc, Weekday};

fn is_daylight_saving_time(datetime: DateTime<Utc>) -> bool {
    let year = datetime.year();
    let dst_start = find_nth_weekday_in_month(year, 3, Weekday::Sun, 2);
    let dst_end = find_nth_weekday_in_month(year, 11, Weekday::Sun, 1);

    let dst_start_utc = Utc.with_ymd_and_hms(year, 3, dst_start, 7, 0, 0).unwrap();
    let dst_end_utc = Utc.with_ymd_and_hms(year, 11, dst_end, 6, 0, 0).unwrap();
    datetime >= dst_start_utc && datetime < dst_end_utc
}

fn convert_utc_to_et(utc_datetime: DateTime<Utc>) -> DateTime<FixedOffset> {
    let offset_hours = if is_daylight_saving_time(utc_datetime) {
        -4
    } else {
        -5
    };
    let offset = FixedOffset::east_opt(offset_hours * 3600).unwrap();
    utc_datetime.with_timezone(&offset)
}

fn find_nth_weekday_in_month(year: i32, month: u32, weekday: Weekday, nth: u32) -> u32 {
    let mut count = 0;
    for day in 1..32 {
        let date = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
        if date.month() != month {
            break; // No more days in the month
        }
        if date.weekday() == weekday {
            count += 1;
            if count == nth {
                return day;
            }
        }
    }
    unreachable!("find_nth_weekday_in_month: given combination is not possible")
}

fn is_weekday_and_time_between_9_30_and_16_et(datetime: DateTime<FixedOffset>) -> bool {
    let hour = datetime.hour();
    let minute = datetime.minute();
    let weekday = datetime.weekday();

    let is_weekday = weekday != Weekday::Sun && weekday != Weekday::Sat;
    let is_after_start = hour > 9 || (hour == 9 && minute >= 30);
    let is_before_end = hour < 16; // 4 PM is 16 in 24-hour format

    is_weekday && is_after_start && is_before_end
}

/// Checks if the stock market is open based on the utc timestamp
pub fn is_market_open(time_stamp: i64) -> bool {
    let utc_datetime = Utc
        .timestamp_opt(time_stamp, 0)
        .single()
        .ok_or(StockMarketError::ConvertUTCError)
        .unwrap();

    let ny_datetime = convert_utc_to_et(utc_datetime);

    is_weekday_and_time_between_9_30_and_16_et(ny_datetime)
}
