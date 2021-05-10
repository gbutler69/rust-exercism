#![feature(iter_intersperse)]

use std::collections::HashMap;

struct TeamRecord {
    won: u16,
    tied: u16,
    lost: u16,
}

impl TeamRecord {
    fn increment_wins(&mut self) {
        self.won += 1;
    }
    fn increment_draws(&mut self) {
        self.tied += 1;
    }
    fn increment_losses(&mut self) {
        self.lost += 1;
    }
    fn matches_played(&self) -> u16 {
        self.won + self.tied + self.lost
    }
    fn total_points(&self) -> u16 {
        self.won * 3 + self.tied
    }
}

impl Default for TeamRecord {
    fn default() -> Self {
        Self {
            won: 0,
            tied: 0,
            lost: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut team_records = HashMap::<&str, TeamRecord>::new();
    {
        for (recnum, rec) in match_results.lines().enumerate() {
            let (team1, team2, outcome) = parse_match_record(recnum, rec);
            team_records =
                process_match_record_into_team_records(team_records, team1, team2, outcome);
        }
    }
    format_team_records_for_output(team_records)
}

fn parse_match_record(recnum: usize, rec: &str) -> (&str, &str, &str) {
    let mut fields = rec.split(';');
    (
        fields
            .next()
            .expect(&format!("Record #{:}: team 1 field missing", recnum)),
        fields
            .next()
            .expect(&format!("Record #{:}: team 2 field missing", recnum)),
        fields
            .next()
            .expect(&format!("Record #{:}: outcome field missing", recnum)),
    )
}

fn process_match_record_into_team_records<'a>(
    mut team_records: HashMap<&'a str, TeamRecord>,
    team1: &'a str,
    team2: &'a str,
    outcome: &'a str,
) -> HashMap<&'a str, TeamRecord> {
    match outcome.as_ref() {
        "win" => {
            team_records
                .entry(team1)
                .or_insert_with(|| Default::default())
                .increment_wins();
            team_records
                .entry(team2)
                .or_insert_with(|| Default::default())
                .increment_losses();
        }
        "loss" => {
            team_records
                .entry(team1)
                .or_insert_with(|| Default::default())
                .increment_losses();
            team_records
                .entry(team2)
                .or_insert_with(|| Default::default())
                .increment_wins();
        }
        "draw" => {
            team_records
                .entry(team1)
                .or_insert_with(|| Default::default())
                .increment_draws();
            team_records
                .entry(team2)
                .or_insert_with(|| Default::default())
                .increment_draws();
        }
        invalid_outcome => panic!(
            "Invalid outcome, '{}', given. Only 'win', 'loss', or 'draw' permitted",
            invalid_outcome
        ),
    }
    team_records
}

fn format_team_records_for_output(team_records: HashMap<&str, TeamRecord>) -> String {
    let header = assemble_header_formatted_team_records(&team_records);
    let formatted_records = format_team_records_as_per_header(team_records);
    header + formatted_records.as_ref()
}

fn assemble_header_formatted_team_records(team_records: &HashMap<&str, TeamRecord>) -> String {
    "Team                           | MP |  W |  D |  L |  P".to_string()
        + (if team_records.is_empty() { "" } else { "\n" })
}

fn format_team_records_as_per_header(team_records: HashMap<&str, TeamRecord>) -> String {
    let formatted_records =
        sort_team_records_by_points_desceding_then_by_team_name_ascending(team_records)
            .into_iter()
            .map(|(team, rec)| {
                format!(
                    "{team:30.30} |{played:>3} |{won:>3} |{tied:>3} |{lost:>3} |{points:>3}",
                    team = team,
                    played = rec.matches_played(),
                    won = rec.won,
                    tied = rec.tied,
                    lost = rec.lost,
                    points = rec.total_points()
                )
            })
            .intersperse("\n".to_owned())
            .collect::<String>();
    formatted_records
}

fn sort_team_records_by_points_desceding_then_by_team_name_ascending(
    team_records: HashMap<&str, TeamRecord>,
) -> Vec<(&str, TeamRecord)> {
    let mut team_records = team_records.into_iter().collect::<Vec<_>>();
    team_records.sort_by_key(|(team, _)| *team);
    team_records.reverse();
    team_records.sort_by_key(|(_, rec)| rec.total_points());
    team_records.reverse();
    team_records
}
