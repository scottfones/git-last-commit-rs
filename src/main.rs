use git2::Repository;
use jiff::{Span, SpanRound, Timestamp, ToSpan, Unit, Zoned, tz::TimeZone};

fn main() {
    if let Ok(span) = get_relative_commit_time() {
        let output = format!("{span:?}")
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .to_owned();
        print!("{output}");
    }
}

fn get_relative_commit_time() -> Result<Span, Box<dyn std::error::Error>> {
    // discover() traverses parent directories to find .git
    let repo = Repository::discover(".")?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;

    let commit_time = Zoned::new(
        Timestamp::from_second(commit.time().seconds())?,
        TimeZone::UTC,
    )
    .checked_add(commit.time().offset_minutes().minutes())?;

    let now = Zoned::now().with_time_zone(TimeZone::UTC);
    let duration = commit_time.duration_until(&now);
    let options = SpanRound::new()
        .largest(Unit::Day)
        .smallest(Unit::Minute)
        .relative(&commit_time);
    Span::try_from(duration)?.round(options).map_err(From::from)
}
