use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    rh_factor: RhFactor,
}

// FromStr for Antigen
impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

// FromStr for RhFactor
impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

// FromStr for BloodType
impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let (antigen_str, rh_str) = if s.ends_with('+') || s.ends_with('-') {
            (&s[..len - 1], &s[len - 1..])
        } else {
            return Err(());
        };

        Ok(BloodType {
            antigen: antigen_str.parse()?,
            rh_factor: rh_str.parse()?,
        })
    }
}

// Ord for BloodType
impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen
            .cmp(&other.antigen)
            .then(self.rh_factor.cmp(&other.rh_factor))
    }
}

// Debug implementation
impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen, rh)
    }
}

// BloodType Methods
impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let rh_ok = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => matches!(other.rh_factor, RhFactor::Negative),
        };

        let antigen_ok = match (&self.antigen, &other.antigen) {
            (Antigen::AB, _) => true,
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        };

        rh_ok && antigen_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        all_blood_types()
            .into_iter()
            .filter(|b| self.can_receive_from(b))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        all_blood_types()
            .into_iter()
            .filter(|b| b.can_receive_from(self))
            .collect()
    }
}

// All blood types utility function
fn all_blood_types() -> Vec<BloodType> {
    let antigens = vec![
        Antigen::O,
        Antigen::A,
        Antigen::B,
        Antigen::AB,
    ];
    let rh_factors = vec![
        RhFactor::Negative,
        RhFactor::Positive,
    ];

    let mut result = vec![];
    for ag in &antigens {
        for rh in &rh_factors {
            result.push(BloodType {
                antigen: ag.clone(),
                rh_factor: rh.clone(),
            });
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blood_type_from_str() {
        let blood_type: BloodType = "A+".parse().unwrap();
        assert_eq!(blood_type.antigen, Antigen::A);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);

        let blood_type: BloodType = "O-".parse().unwrap();
        assert_eq!(blood_type.antigen, Antigen::O);
        assert_eq!(blood_type.rh_factor, RhFactor::Negative);

        let blood_type: BloodType = "AB+".parse().unwrap();
        assert_eq!(blood_type.antigen, Antigen::AB);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);
    }
    #[test]
    fn test_blood_type_can_receive_from() {
        let blood_type_a: BloodType = "A+".parse().unwrap();
        let blood_type_b: BloodType = "B+".parse().unwrap();
        let blood_type_ab: BloodType = "AB+".parse().unwrap();
        let blood_type_o: BloodType = "O-".parse().unwrap();

        assert!(blood_type_a.can_receive_from(&blood_type_a));
        assert!(!blood_type_a.can_receive_from(&blood_type_b));
        assert!(blood_type_ab.can_receive_from(&blood_type_a));
        assert!(blood_type_ab.can_receive_from(&blood_type_b));
        assert!(!blood_type_o.can_receive_from(&blood_type_a));
    }
}
