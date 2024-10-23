#[derive(Debug)]
pub enum Major {
    ComputerScience,
    ElectricalEngineering,
    Undefined,
}

impl Major {
    pub fn classify(major:&str) {
        match major {
           "CS" => Major::CS,
           "EE" => Major::EE,
           _ => Major::Undefined
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn
}