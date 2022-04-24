use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct CPU {
    pub user: f64,
    pub system: f64,
    pub idle: f64,
}


impl FromStr for CPU {
    type Err = std::num::ParseIntError;

    // Parses CPU usage from TOP into an instance of CPU
    // format = 'CPU usage: 8.82% user, 14.70% sys, 76.47% idle'
    fn from_str(top_cpu_usage: &str) -> Result<Self, Self::Err> {
        let mut usages = top_cpu_usage.split_whitespace()
            .filter(|s| s.ends_with("%"))
            .map(|s| s.strip_suffix("%").unwrap())
            .map(|s| s.parse::<f64>().unwrap());

        let user = usages.next().unwrap();
        let system = usages.next().unwrap();
        let idle = usages.next().unwrap();

        Ok(CPU { user, system, idle })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let activity = "CPU usage: 8.82% user, 14.70% sys, 76.47% idle";
        let cpu = CPU::from_str(activity).unwrap();

        assert_eq!(cpu, CPU { user: 8.82, system: 14.70, idle: 76.47 });
    }
}