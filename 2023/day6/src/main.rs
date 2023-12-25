use std::str::FromStr;

fn main() -> Result<(), RecordsError> {
    let input = "\
Time:        53     83     72     88
Distance:   333   1635   1289   1532
        ";
    let records: Records = input.parse()?;
    let total = records.calculate_total_ways();

    println!("ways {}", total);

    let combine = records.calculate_combine();
    println!("combine {}", combine);
    Ok(())
}

#[derive(Debug, PartialEq)]
struct RecordsError;

struct Records(Vec<Record>);

struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    fn calculate(&self) -> u64 {
        let mut ways = 0;
        for i in 0..=self.time {
            let speed = i;
            let distance = speed * (self.time - i);
            if distance > self.distance {
                ways += 1;
            }
        }
        dbg!(
            "time {} distance {} ways {}",
            &self.time,
            &self.distance,
            &ways
        );
        ways
    }
}

impl Records {
    fn calculate_total_ways(&self) -> u64 {
        let records = &self.0;
        let mut total = 1;
        for record in records {
            total *= record.calculate();
        }
        total
    }

    fn calculate_combine(&self) -> u64 {
        let record = Record {
            time: self.0.iter().fold(0, |acc, r| {
                acc * 10u64.pow(number_of_digits(r.time)) + r.time
            }),
            distance: self.0.iter().fold(0, |acc, r| {
                acc * 10u64.pow(number_of_digits(r.distance)) + r.distance
            }),
        };
        record.calculate()
    }
}
fn number_of_digits(n: u64) -> u32 {
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}
impl FromStr for Record {
    type Err = RecordsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split_whitespace().collect();

        Ok(Record {
            time: split[0].parse::<u64>().expect("can not parse u6"),
            distance: split[1].parse::<u64>().expect("can not parse u6"),
        })
    }
}

impl FromStr for Records {
    type Err = RecordsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut time = vec![];
        let mut distances = vec![];

        for (ix, line) in s.lines().enumerate() {
            if ix == 0 {
                if let Some(time_str) = line.strip_prefix("Time: ") {
                    for t in time_str.split_whitespace().map(u64::from_str) {
                        let t = t.map_err(|_| RecordsError)?;
                        time.push(t);
                    }
                } else {
                    return Err(RecordsError);
                }
            }

            if ix == 1 {
                if let Some(distance_str) = line.strip_prefix("Distance: ") {
                    for d in distance_str.split_whitespace().map(u64::from_str) {
                        let d = d.map_err(|_| RecordsError)?;
                        distances.push(d);
                    }
                } else {
                    return Err(RecordsError);
                }
            }
        }

        let mut records = vec![];
        for i in 0..time.len() {
            let fmt = format!("{} {}", time[i], distances[i]);
            let record: Record = fmt.parse()?;
            records.push(record);
        }
        Ok(Records(records))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200
    ";

    #[test]
    fn test_sample() -> Result<(), RecordsError> {
        let records: Records = SAMPLE.parse()?;
        let total = records.calculate_total_ways();
        let combine = records.calculate_combine();

        assert_eq!(total, 288);
        assert_eq!(combine, 71503);
        Ok(())
    }
}
