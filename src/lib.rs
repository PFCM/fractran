use std::iter;
use std::ops::Mul;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl FromStr for Rational {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.trim().split('/').collect();
        if items.len() != 2 {
            return Err(format!(
                "{} components in rational, not 2\n({})",
                items.len(),
                s
            ));
        }
        let numerator = parse_int(items[0])?;
        let denominator = parse_int(items[1])?;

        Ok(Rational {
            numerator,
            denominator,
        })
    }
}

impl Mul<i64> for Rational {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        let r = Rational {
            numerator: self.numerator * rhs,
            denominator: self.denominator,
        };
        r.simplify()
    }
}

impl Rational {
    fn simplify(&self) -> Self {
        let gcd = gcd(self.numerator, self.denominator);
        Rational {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    fn as_integer(&self) -> Option<i64> {
        let r = self.simplify();
        if r.denominator == 1 {
            return Some(r.numerator);
        }
        None
    }
}

fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn parse_int(s: &str) -> Result<i64, <Rational as FromStr>::Err> {
    match s.parse::<i64>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("bad integer: {}", s)),
    }
}

#[derive(Debug)]
pub struct Program {
    fracs: Vec<Rational>,
    state: i64,
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.split(',').map(str::trim).collect();
        if items.len() < 2 {
            return Err(format!(
                "{} needs two comma separated elements, not {}",
                s,
                items.len()
            ));
        }
        let fracs: Vec<Rational> = match items[0..items.len() - 1]
            .iter()
            .map(|s| s.parse::<Rational>())
            .collect()
        {
            Ok(f) => f,
            Err(msg) => return Err(msg),
        };
        match items[items.len() - 1].parse::<i64>() {
            Ok(state) => Ok(Program { fracs, state }),
            Err(_) => Err(format!("bad final state: {}", items[items.len() - 1])),
        }
    }
}

impl Program {
    pub fn next(&mut self) -> Option<i64> {
        for f in self.fracs.iter() {
            match (*f * self.state).simplify().as_integer() {
                Some(n) => {
                    self.state = n;
                    return Some(n);
                }
                None => continue,
            }
        }
        None
    }
}

pub fn run(prog: &mut Program) -> impl Iterator<Item = i64> + '_ {
    iter::once(prog.state).chain(iter::from_fn(move || prog.next()))
}
