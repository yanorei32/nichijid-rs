use std::net::SocketAddr;

use chrono::{Datelike, Local, Timelike, Weekday};
use clap::Parser;
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::info;
use tracing_subscriber;

#[must_use]
fn nonzero_digit_to_generic_yomi(digit: u32) -> &'static str {
    match digit {
        1 => "ichi",
        2 => "ni",
        3 => "san",
        4 => "yon",
        5 => "go",
        6 => "roku",
        7 => "nana",
        8 => "hachi",
        9 => "kyu",
        _ => panic!("Outside 1 to 9"),
    }
}

#[must_use]
fn positive_year_to_yomi(year: u32) -> String {
    #[must_use]
    fn least_digit_in_year_to_yomi(digit: u32) -> &'static str {
        match digit {
            4 => "yo",
            9 => "ku",
            _ => nonzero_digit_to_generic_yomi(digit),
        }
    }

    let mut s = String::new();

    if !(1..=9999).contains(&year) {
        panic!("year outside 1 to 9999");
    }

    let thousands = year / 1000;

    match thousands {
        0 => {}
        1 => s += "sen ",
        3 => s += "san-zen ",
        8 => s += "hassen ",
        v => {
            s += nonzero_digit_to_generic_yomi(v);
            s += "-sen ";
        }
    }

    let hundreds = (year % 1000) / 100;

    match hundreds {
        0 => {}
        1 => s += "hyaku ",
        3 => s += "san-byaku ",
        6 => s += "roppyaku ",
        8 => s += "happyaku ",
        v => {
            s += nonzero_digit_to_generic_yomi(v);
            s += "-hyaku ";
        }
    }

    let least_two_digits = year % 100;

    match least_two_digits {
        0 => {}
        v if v < 10 => {
            s += least_digit_in_year_to_yomi(v);
            s += " ";
        }
        v => {
            let tens = v / 10;
            let least_digit = v % 10;

            match tens {
                0 => {}
                1 => s += "juu ",
                v => {
                    s += nonzero_digit_to_generic_yomi(v);
                    s += "-juu ";
                }
            }

            if least_digit != 0 {
                s += least_digit_in_year_to_yomi(least_digit);
                s += " ";
            }
        }
    }

    s += "nen";

    s
}

#[rstest::rstest]
#[case(1858, "sen happyaku go-juu hachi nen")]
#[case(1983, "sen kyu-hyaku hachi-juu san nen")]
#[case(2002, "ni-sen ni nen")]
#[case(2023, "ni-sen ni-juu san nen")]
#[case(2112, "ni-sen hyaku juu ni nen")]
fn positive_year_to_yomi_test(#[case] input: u32, #[case] expected: &str) {
    assert_eq!(positive_year_to_yomi(input), expected)
}

#[test]
fn positive_year_to_yomi_does_not_panic() {
    for n in 1..=9999 {
        let _ = positive_year_to_yomi(n);
    }
}

#[must_use]
fn month_to_yomi(month: u32) -> String {
    let mut s = String::new();

    match month {
        4 => s += "shi ",
        7 => s += "shichi ",
        9 => s += "ku ",
        10 => s += "juu ",
        11 => s += "juu-ichi ",
        12 => s += "juu-ni ",
        v if (1..=9).contains(&v) => {
            s += nonzero_digit_to_generic_yomi(v);
            s += " ";
        }
        _ => panic!("month outside 1..=12"),
    }

    s += "gatsu";

    s
}

#[must_use]
fn day_to_yomi(day: u32) -> String {
    #[must_use]
    fn least_digit_in_day_to_yomi(digit: u32) -> &'static str {
        match digit {
            7 => "shichi",
            9 => "ku",
            _ => nonzero_digit_to_generic_yomi(digit),
        }
    }

    if !(1..=31).contains(&day) {
        panic!("day outside 1..=31");
    }

    if (1..=10).contains(&day) {
        return match day {
            1 => "tsuitachi",
            2 => "futsuka",
            3 => "mikka",
            4 => "yokka",
            5 => "itsuka",
            6 => "muika",
            7 => "nanoka",
            8 => "youka",
            9 => "kokonoka",
            10 => "touka",
            _ => unreachable!(),
        }
        .to_string();
    }

    match day {
        14 => "juu yokka".to_string(),
        20 => "hatsuka".to_string(),
        24 => "ni-juu yokka".to_string(),

        v => {
            let mut s = String::new();
            let tens = v / 10;

            match tens {
                0 => {}
                1 => s += "juu ",
                2 => s += "ni-juu ",
                3 => s += "san-juu ",
                _ => unreachable!(),
            }

            let least_digit = v % 10;

            if least_digit != 0 {
                s += least_digit_in_day_to_yomi(least_digit);
                s += " ";
            }

            s += "nichi";

            s
        }
    }
}

#[test]
fn positive_month_to_yomi_does_not_panic() {
    for n in 1..=31 {
        let _ = day_to_yomi(n);
    }
}

#[must_use]
fn weekday_to_yomi(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "getsu youbi",
        Weekday::Tue => "ka youbi",
        Weekday::Wed => "sui youbi",
        Weekday::Thu => "moku youbi",
        Weekday::Fri => "kin youbi",
        Weekday::Sat => "do youbi",
        Weekday::Sun => "nichi youbi",
    }
}

#[must_use]
fn hour_to_yomi(hour: u32) -> String {
    #[must_use]
    fn least_digit_in_hour_to_yomi(digit: u32) -> &'static str {
        match digit {
            4 => "yo",
            7 => "shichi",
            9 => "ku",
            _ => nonzero_digit_to_generic_yomi(digit),
        }
    }

    if !(0..=23).contains(&hour) {
        panic!("hour outside 0..=23");
    }

    let mut s = String::new();

    match hour {
        0 => s += "rei ",

        v => {
            let tens = v / 10;
            let least_digit = v % 10;

            match tens {
                0 => {}
                1 => s += "juu ",
                2 => s += "ni-juu ",
                _ => panic!(),
            }

            if least_digit != 0 {
                s += least_digit_in_hour_to_yomi(least_digit);
                s += " ";
            }
        }
    }

    s += "ji";

    s
}

#[must_use]
fn minute_to_yomi(minute: u32) -> String {
    #[must_use]
    fn nonzero_last_digit_with_unit(digit: u32) -> &'static str {
        match digit {
            1 => "ippun",
            2 => "ni hun",
            3 => "san hun",
            4 => "yon hun",
            5 => "go hun",
            6 => "roppun",
            7 => "nana hun",
            8 => "happun",
            9 => "kyu hun",
            _ => panic!("given digit not in 1..=9"),
        }
    }

    if !(0..=59).contains(&minute) {
        panic!("minute outside 0..=59");
    }

    let tens = minute / 10;
    let least_digit = minute % 10;

    match minute {
        0 => "rei hun".to_string(),

        _ if least_digit == 0 => match tens {
            1 => "juppun".to_string(),
            _ => format!("{} juppun", nonzero_digit_to_generic_yomi(tens)),
        },

        _ => {
            let mut s = String::new();

            match tens {
                0 => {}
                1 => s += "juu ",
                _ => {
                    s += nonzero_digit_to_generic_yomi(tens);
                    s += "-juu ";
                }
            }

            s += nonzero_last_digit_with_unit(least_digit);

            s
        }
    }
}

#[must_use]
fn sec_to_yomi(sec: u32) -> String {
    let mut s = String::new();

    let tens = sec / 10;
    let least_digit = sec % 10;

    if sec == 0 {
        return "rei byou".to_string();
    }

    match tens {
        0 => {}
        1 => s += "juu ",
        _ => {
            s += nonzero_digit_to_generic_yomi(tens);
            s += "-juu ";
        }
    }

    if least_digit != 0 {
        s += nonzero_digit_to_generic_yomi(least_digit);
        s += " ";
    }

    s += "byou";

    s
}

fn format_datetime<T: Datelike + Timelike>(t: &T) -> String {
    let dy = &positive_year_to_yomi(t.year() as u32);
    let dm = &month_to_yomi(t.month());
    let dd = &day_to_yomi(t.day());
    let dw = &weekday_to_yomi(t.weekday());
    let th = &hour_to_yomi(t.hour());
    let tm = &minute_to_yomi(t.minute());
    let ts = &sec_to_yomi(t.second());

    format!("{dy} {dm} {dd} {dw} {th} {tm} {ts}\r\n")
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(default_value = "0.0.0.0:13")]
    host: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let c = Cli::parse();
    let listener = TcpListener::bind(c.host).await?;
    let addr = listener.local_addr()?;
    info!("Server is ready on {addr}");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("Accept request from: {addr}");

        tokio::spawn(async move {
            let s = format_datetime(&Local::now());
            if let Err(v) = socket.write_all(&s.into_bytes()).await {
                info!("Something went wrong: {v}");
            };
        });
    }
}
