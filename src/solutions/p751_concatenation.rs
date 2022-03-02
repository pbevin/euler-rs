use lazy_static::lazy_static;
use num_bigint::BigInt;

euler::solution!(p751, "Concatenation coincidence", "2.223561019313554106173177".to_string());

// Looking for a fixed point of generate()
fn p751() -> String {
    let mut prev = "2.2".to_string();
    loop {
        let next = generate(&prev);
        if next == prev {
            break;
        }
        prev = next;
    }
    assert_eq!(prev.len(), 26);
    prev
}

fn generate(start: &str) -> String {
    let pos = start.find('.').unwrap();
    assert_eq!(&start.as_bytes()[..pos], b"2");
    let frac = &start[pos + 1..];
    assert!(frac.len() <= 30);
    let mut b = Fixed::parse(2, frac).unwrap();
    let mut result = "2.".to_string();
    while result.len() < 26 {
        b = step(b);
        let a = b.int();
        result.push_str(&a.to_string());
    }
    result
}

// integer and 30 digits after the point
#[derive(Debug, Clone)]
struct Fixed {
    z: BigInt,
}

impl Fixed {
    fn int(&self) -> u64 {
        (self.z.clone() / SCALE.clone()).try_into().unwrap()
    }

    fn parse(int: u64, frac: &str) -> Option<Self> {
        let frac = format!("{:0<30}", frac);
        let z2 = BigInt::parse_bytes(frac.as_bytes(), 10)?;
        let z1 = BigInt::from(int) * SCALE.clone();
        Some(Self { z: z1 + z2 })
    }

    fn replace_int_with_one(&self) -> Self {
        let scale = SCALE.clone();
        let z = (self.z.clone() % scale.clone()) + scale;
        Self { z }
    }

    fn floor(&self) -> Self {
        let scale = SCALE.clone();
        let mut z = self.z.clone() / scale.clone();
        z *= scale;
        Self { z }
    }
}

fn step(b: Fixed) -> Fixed {
    let lhs = b.floor();
    let rhs = b.replace_int_with_one();
    mul(lhs, rhs)
}

fn mul(x: Fixed, y: Fixed) -> Fixed {
    let z1z2 = x.z * y.z;
    let z = z1z2 / SCALE.clone();
    Fixed { z }
}

lazy_static! {
    static ref SCALE: BigInt = {
        let z = BigInt::from(10);
        z.pow(30)
    };
}
