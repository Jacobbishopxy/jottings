//! A mock commands collection from Spark tasks
//!
//! mocking

#[allow(dead_code)]
#[derive(Debug)]
enum Task<'a> {
    Information(Information, Command<'a>),
    TimeSeries(TimeSeries, Command<'a>),
    Finance(Finance, Command<'a>),
    Other,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Information {
    AShareInformationWind,
    AShareInformationCitics,
    AShareCalendar,
    Other,
}

#[allow(dead_code)]
#[derive(Debug)]
enum TimeSeries {
    AShareTradingSuspension,
    AShareEXRightDividend,
    AShareEODPrices,
    Other,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Finance {
    AShareBalanceSheet,
    AShareCashFlow,
    AShareIncome,
    Other,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Command<'a> {
    SyncAll,
    DailyUpsert(&'a str),
    DailyDelete(&'a str),
    TimeFromUpsert(&'a str),
    TimeFromDelete(&'a str),
    TimeRangeUpsert(&'a str, &'a str),
    TimeRangeDelete(&'a str, &'a str),
    Initialize(&'a str),
    Other,
}

#[allow(dead_code)]
fn task_classifier<'a>(args: &'a [&str]) -> Task<'a> {
    match args {
        ["Finance", "AShareCashFlow", "DailyUpsert", date, ..] => {
            Task::Finance(Finance::AShareCashFlow, Command::DailyUpsert(date))
        }
        _ => Task::Other,
    }
}

#[test]
fn fake_command_success() {
    let args = vec!["Finance", "AShareCashFlow", "DailyUpsert", "20220905"];

    let task = task_classifier(&args);

    println!("{:?}", task);
}
