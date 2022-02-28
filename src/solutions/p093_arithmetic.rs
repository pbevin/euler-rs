// The problem is to generate expressions with 4 digits and 3 binops,
// and determine which end results are reachable.
//
// There are 5 expr templates:
//   a + (b + (c + d))
//   a + ((b + c) + d)
//   (a + b) + (c + d)
//   (a + (b + c)) + d
//   ((a + b) + c) + d
//
// So we can enumerate expression trees by substituting
// digits 1..9 for a, b, c, and d, and operations for +

use std::collections::BTreeMap;

use euler::Best;
use fixedbitset::FixedBitSet;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

// In fact, we don't create expression trees. An expression is just the
// 4 digits, 3 binops, and a type.
struct Expr {
    expr_type: ExprType,
    nums: [f64; 4],
    ops: [Op; 3],
}

enum ExprType {
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
}

impl Expr {
    fn eval(&self) -> Option<usize> {
        let [a, b, c, d] = self.nums;
        let [op1, op2, op3] = self.ops;
        let num = match self.expr_type {
            ExprType::Type1 => bin(op1, a, bin(op2, b, bin(op3, c, d)?)?)?,
            ExprType::Type2 => bin(op1, a, bin(op3, bin(op2, b, c)?, d)?)?,
            ExprType::Type3 => bin(op2, bin(op1, a, b)?, bin(op3, c, d)?)?,
            ExprType::Type4 => bin(op3, bin(op1, a, bin(op2, b, c)?)?, d)?,
            ExprType::Type5 => bin(op3, bin(op2, bin(op1, a, b)?, c)?, d)?,
        };

        // We're only interested in positive whole-number outputs (even though the intermediate
        // values can be fractional).
        if num > 0.0 && (num - num.round()).abs() < 0.001 {
            Some(num.round() as usize)
        } else {
            None
        }
    }
}

euler::solution!(p093, "Arithmetic expressions", 1258);

fn bin(op: Op, lhs: f64, rhs: f64) -> Option<f64> {
    match op {
        Op::Add => Some(lhs + rhs),
        Op::Sub => Some(lhs - rhs),
        Op::Mul => Some(lhs * rhs),
        Op::Div => {
            if rhs == 0.0 {
                None
            } else {
                Some(lhs / rhs)
            }
        }
    }
}

pub fn p093() -> usize {
    let mut hash = BTreeMap::new();
    let it = (1..9)
        .cartesian_product(1..9)
        .cartesian_product(1..9)
        .cartesian_product(1..9);
    for (((a, b), c), d) in it {
        if let Some(abcd) = sorted([a, b, c, d]) {
            let entry = hash
                .entry(abcd)
                .or_insert_with(|| FixedBitSet::with_capacity(10_000));

            let a = a as f64;
            let b = b as f64;
            let c = c as f64;
            let d = d as f64;

            for (op1, op2, op3) in gen_ops() {
                for expr_type in [
                    ExprType::Type1,
                    ExprType::Type2,
                    ExprType::Type3,
                    ExprType::Type4,
                    ExprType::Type5,
                ] {
                    let expr = Expr {
                        expr_type,
                        nums: [a, b, c, d],
                        ops: [op1, op2, op3],
                    };
                    if let Some(val) = expr.eval() {
                        entry.insert(val);
                    }
                }
            }
        }
    }

    let mut best = Best::<(usize, String)>::new();
    for (k, mut v) in hash {
        v.insert(0);
        v.toggle_range(..);
        let lowest_unreachable = v.ones().next().unwrap();
        best.max((lowest_unreachable, k));
    }

    best.into_inner().unwrap().1.parse().unwrap()
}

fn sorted(mut nums: [i32; 4]) -> Option<String> {
    nums.sort_unstable();
    if nums[0] == nums[1] || nums[1] == nums[2] || nums[2] == nums[3] {
        None
    } else {
        Some(format!("{}{}{}{}", nums[0], nums[1], nums[2], nums[3]))
    }
}

fn gen_ops() -> impl Iterator<Item = (Op, Op, Op)> {
    let ops = &[Op::Add, Op::Sub, Op::Mul, Op::Div];
    let it = ops
        .iter()
        .cartesian_product(ops.iter())
        .cartesian_product(ops.iter());
    it.map(|((op1, op2), op3)| (*op1, *op2, *op3))
}
