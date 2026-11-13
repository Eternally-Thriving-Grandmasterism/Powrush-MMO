//! Unix days to a Gregorian date (Howard Hinnant). No date crate.
//! An adult is 18 only after the birth month has fully passed.

pub fn civil_from_days(mut z: i64) -> (i32, u32, u32) {
    z += 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    ((y + i64::from(m <= 2)) as i32, m as u32, d as u32)
}

pub fn is_adult(today_y: i32, today_m: u32, birth_y: i32, birth_m: u32) -> bool {
    let dy = today_y - birth_y;
    dy > 18 || (dy == 18 && today_m > birth_m)
}

pub fn civil_today() -> Option<(i32, u32, u32)> {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some(civil_from_days((secs / 86_400) as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_match_known_civil_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(1), (1970, 1, 2));
        assert_eq!(civil_from_days(-1), (1969, 12, 31));
        assert_eq!(civil_from_days(10957), (2000, 1, 1));
        assert_eq!(civil_from_days(11016), (2000, 2, 29));
    }

    /// Month before (adult at 18), same month (not), month after, future year, Dec→Jan.
    #[test]
    fn adult_only_after_the_birth_month() {
        let cases = [
            (2026, 6, 2008, 5, true),
            (2026, 6, 2008, 6, false),
            (2026, 6, 2008, 7, false),
            (2026, 6, 2027, 1, false),
            (2026, 1, 2008, 12, false),
            (2026, 1, 2007, 12, true),
            (2026, 12, 2008, 1, true),
            (2020, 3, 2024, 1, false),
        ];
        for (y, m, by, bm, want) in cases {
            assert_eq!(is_adult(y, m, by, bm), want, "{y}-{m} born {by}-{bm}");
        }
    }
}
