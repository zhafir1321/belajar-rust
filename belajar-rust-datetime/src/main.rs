fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    /*
    CHRONO DATE
    - Chrono memiliki dua jenis tipe data waktu, tipe data waktu yang memiliki timezone dan yang tidak memiliki timezone
    - Sekarang kita akan bahas dulu tipe data yang tidak memiliki timezone
    - Dimulai dari tipe data Date
    - Date adalah tipe data waktu tanggal bulan dan tahun
    - Date direpresentasikan dengan struct NaiveDate di Chrono
     */

    use chrono::{
        DateTime, Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime,
        TimeZone, Timelike, Utc,
    };
    use chrono_tz::{Asia, Tz};

    #[test]
    fn test_date() {
        let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        println!("Date: {}", date);
        println!("Year: {}", date.year());
        println!("Month: {}", date.month());
        println!("Day: {}", date.day());
    }

    /*
    DURATION
    - Semua tipe data waktu di Chrono adalah immutable, artinya tidak bisa berubah
    - Jadi tidak ada method yang bisa kita gunakan untuk mengubah waktu pada object yang sudah kita buat
    - Namun Chrono menyediakan operator Add (+) dan Sub (-) yang bisa kita gunakan untuk menambah/ mengurangi waktu
    - Namun operator ini, hanya bisa digunakan dengan tipe data Duration, yaitu alias untuk TimeDelta
     */

    #[test]
    fn test_duration() {
        let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        let new_date = date + Duration::days(10);
        println!("New Date: {}", new_date);
    }

    /*
    TIME
    - Chrono mendukung tipe data Time (jam menit detik nanosecond) tanpa timezone menggunakan struct NaiveTime
     */

    #[test]
    fn test_time() {
        let time = NaiveTime::from_hms_milli_opt(12, 30, 45, 500).unwrap();
        println!("Time: {}", time);
        println!("Hour: {}", time.hour());
        println!("Minute: {}", time.minute());
        println!("Second: {}", time.second());
        println!("Nanosecond: {}", time.nanosecond());
    }

    /*
    DATE TIME
    - Untuk tipe data gabungan antara Date dan Time, di Chrono bisa menggunakan tipe data NaiveDateTime
     */

    #[test]
    fn test_datetime() {
        let datetime: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
            NaiveTime::from_hms_milli_opt(12, 30, 45, 500).unwrap(),
        );
        println!("DateTime: {}", datetime);
        println!("Date: {}", datetime.date());
        println!("Year: {}", datetime.year());
        println!("Month: {}", datetime.month());
        println!("Day: {}", datetime.day());
        println!("Time: {}", datetime.time());
        println!("Hour: {}", datetime.hour());
        println!("Minute: {}", datetime.minute());
        println!("Second: {}", datetime.second());
        println!("Nanosecond: {}", datetime.nanosecond());
    }

    /*
    TIMEZONE
    - Saat kita menggunakan tipe data tanggal dan waktu, kadang kita akan bersinggungan dengan zona waktu
    - Chrono mendukung zona waktu menggunakan TimeZone

    LIMITASI LIBRARY CHRONO
    - Karena data TimeZone di seluruh dunia sangat banyak, oleh karena itu pada Library Chrono,
    hanya dibatasi 2 TimeZone saja yang disediakan, yaitu Local dan UTC
    - Jika kita ingin menggunakan TimeZone lain, maka kita harus lakukan secara manual menggunakan offset (perbedaan waktu)
     */

    #[test]
    fn test_fixed_offset() {
        let utc_datetime: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
            NaiveTime::from_hms_opt(12, 30, 45).unwrap(),
        );

        let asia_jakarta: FixedOffset = FixedOffset::east_opt(7 * 3600).unwrap();
        let asia_jakarta_datetime = asia_jakarta.from_utc_datetime(&utc_datetime);

        println!("UTC DateTime: {}", utc_datetime);
        println!("Asia/Jakarta DateTime: {}", asia_jakarta_datetime);
    }

    /*
    CHRONO TIMEZONE
    - Chrono menyediakan library terpisah untuk data seluruh TimeZone di dunia, yaitu Chrono TimeZone
    - Kita bisa install dengan menggunakan `cargo add chrono-tz`
     */

    #[test]
    fn test_timezone() {
        let utc_datetime: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
            NaiveTime::from_hms_opt(12, 30, 45).unwrap(),
        );

        let asia_jakarta = Asia::Jakarta;
        let asia_jakarta_datetime = asia_jakarta.from_utc_datetime(&utc_datetime);

        println!("UTC DateTime: {}", utc_datetime);
        println!("Asia/Jakarta DateTime: {}", asia_jakarta_datetime);
    }

    /*
    DATE DENGAN TIMEZONE
    - Date Time yang memiliki Time Zone di Chrono bisa menggunakan tipe data DateTime
     */

    #[test]
    fn test_datetime_with_timezone() {
        let utc_datetime: DateTime<Utc> = Utc::now();
        let asia_jakarta_datetime: DateTime<Tz> =
            Asia::Jakarta.from_utc_datetime(&utc_datetime.naive_utc());

        println!("UTC DateTime: {}", utc_datetime);
        println!("Asia/Jakarta DateTime: {}", asia_jakarta_datetime);

        let local_datetime: DateTime<Local> = Local::now();
        println!("Local DateTime: {}", local_datetime);
    }

    /*
    PARSING
    - Salah satu yang biasa kita lakukan ketika membuat aplikasi adalah melakukan parsing data String menjadi tipe data waktu/ tanggal
    - Chrono sudah menyediakan fitur untuk melakukan parsing, sehingga memudahkan kita ketika ingin mengkonversi dari tipe data String menjadi waktu/ tanggal
    - Untuk melakukan parsing, kita bisa menggunakan method `parse(string, format)` pada tipe data yang ada di Chrono,
    seperti NaiveDate, NaiveTime, NaiveDateTime, dan DateTime

    FORMATTING
    - Sebelum melakukan parsing, kita harus tahu bagaimana format sintaks untuk melakukan parsing nya
    - Kita bisa baca dokumentasinya pada module `strftime`
    - https://docs.rs/chrono/0.4.38/chrono/format/strftime/index.html
     */

    #[test]
    fn test_parsing() {
        let string: String = String::from("2025-12-31 12:30:45 +0700");
        let time = DateTime::parse_from_str(&string, "%Y-%m-%d %H:%M:%S %z").unwrap();

        println!("Parsed DateTime: {}", time);
        println!("Year: {}", time.year());
        println!("Month: {}", time.month());
        println!("Day: {}", time.day());
        println!("Hour: {}", time.hour());
        println!("Minute: {}", time.minute());
        println!("Second: {}", time.second());
        println!("Nanosecond: {}", time.nanosecond());
        println!("Timezone: {}", time.timezone());
    }

    /*
    FORMAT
    - Selain melakukan parsing dari String ke waktu/ tanggal, Chrono juga bisa digunakan untuk melakukan proses format (kebalikannya),
    yaitu mengubah tipe data waktu/ tanggal ke String
    - Kita bisa menggunakan method format pada semua tipe data yang ada di Chrono
     */

    #[test]
    fn test_format() {
        let local_now = Local::now();
        let formatted = local_now.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Formatted DateTime: {}", formatted);
    }
}
