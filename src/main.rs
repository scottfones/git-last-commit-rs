use git2::Repository;
use jiff::{SignedDuration, Timestamp};

fn main() {
    if let Ok(elapsed) = get_commit_delta() {
        print!("{}", format_elapsed(elapsed.as_secs()));
    }
}

fn format_elapsed(secs: i64) -> String {
    let minutes = (secs + 30) / 60;
    if minutes < 60 {
        return format!("{minutes}m");
    }

    let hours = (minutes + 30) / 60;
    if hours < 24 {
        return format!("{hours}h");
    }

    format!("{}d", (hours + 12) / 24)
}

fn get_commit_delta() -> Result<SignedDuration, Box<dyn std::error::Error>> {
    let repo = Repository::discover(".")?;
    let commit = repo.head()?.peel_to_commit()?;
    let commit_time = Timestamp::from_second(commit.time().seconds())?;
    Ok(Timestamp::now().duration_since(commit_time))
}

#[cfg(test)]
mod tests {
    use super::format_elapsed;

    #[test]
    fn ceil_of_largest_unit() {
        assert_eq!(format_elapsed(10), "0m");
        assert_eq!(format_elapsed(40), "1m");
        assert_eq!(format_elapsed(60), "1m");
        assert_eq!(format_elapsed(65), "1m");
        assert_eq!(format_elapsed(1_020), "17m");
        assert_eq!(format_elapsed(3_570), "1h");
        assert_eq!(format_elapsed(3_600), "1h");
        assert_eq!(format_elapsed(3_620), "1h");
        assert_eq!(format_elapsed(86_400), "1d");
        assert_eq!(format_elapsed(1_900_800), "22d");
    }
}
