use std::ops;
use std::cmp;
use std::fmt;

#[derive(Debug, Clone)]
pub enum NumericType {
    Integer(i32),
    Float(f32)
}

impl NumericType {
    pub fn to_bool(&self) -> bool {
        match *self {
            NumericType::Integer(ref val) =>
                if *val == 0 { false } else { true },
            NumericType::Float(ref val) =>
                if *val == 0.0 { false } else { true }
        }
    }

    pub fn pow(&self, num: &NumericType) -> NumericType {
        match (self, num) {
            (&NumericType::Integer(val1), &NumericType::Integer(val2)) => {
                if val2 < 0 {
                    NumericType::Float(1.0 / val1.pow(val2.abs() as u32) as f32)
                } else {
                    NumericType::Integer(val1.pow(val2 as u32))
                }
            },
            (&NumericType::Integer(val1), &NumericType::Float(val2)) => {
                if val2 < 0.0 {
                    NumericType::Float(1.0 / (val1 as f32).powf(val2.abs()))
                } else {
                    NumericType::Float((val1 as f32).powf(val2))
                }
            },
            (&NumericType::Float(val1), &NumericType::Integer(val2)) => {
                if val2 < 0 {
                    NumericType::Float(1.0 / val1.powi(val2.abs()))
                } else {
                    NumericType::Float(val1.powi(val2))
                }
            },
            (&NumericType::Float(val1), &NumericType::Float(val2)) => {
                if val2 < 0.0 {
                    NumericType::Float(1.0 / val1.powf(val2.abs()))
                } else {
                    NumericType::Float(val1.powf(val2))
                }
            }
        }
    }
}

impl fmt::Display for NumericType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NumericType::Integer(num) => write!(f, "{}", num),
            NumericType::Float(num) => write!(f, "{}", num)
        }
    }
}

impl cmp::PartialEq for NumericType {
    fn eq(&self, other: &NumericType) -> bool {
        match (self, other) {
            (&NumericType::Integer(val1), &NumericType::Integer(val2)) => {
                val1 == val2
            },
            (&NumericType::Integer(val1), &NumericType::Float(val2)) => {
                (val1 as f32) == val2
            },
            (&NumericType::Float(val1), &NumericType::Integer(val2)) => {
                val1 == (val2 as f32)
            },
            (&NumericType::Float(val1), &NumericType::Float(val2)) => {
                val1 == val2
            }
        }
    }

    fn ne(&self, other: &NumericType) -> bool {
        match (self, other) {
            (&NumericType::Integer(val1), &NumericType::Integer(val2)) => {
                val1 != val2
            },
            (&NumericType::Integer(val1), &NumericType::Float(val2)) => {
                (val1 as f32) != val2
            },
            (&NumericType::Float(val1), &NumericType::Integer(val2)) => {
                val1 != (val2 as f32)
            },
            (&NumericType::Float(val1), &NumericType::Float(val2)) => {
                val1 != val2
            }
        }
    }
}

impl cmp::PartialOrd for NumericType {
    fn partial_cmp(&self, other: &NumericType) -> Option<cmp::Ordering> {
        match (self, other) {
            (&NumericType::Integer(val1), &NumericType::Integer(val2)) => {
                val1.partial_cmp(&val2)
            },
            (&NumericType::Integer(val1), &NumericType::Float(val2)) => {
                (val1 as f32).partial_cmp(&val2)
            },
            (&NumericType::Float(val1), &NumericType::Integer(val2)) => {
                val1.partial_cmp(&(val2 as f32))
            },
            (&NumericType::Float(val1), &NumericType::Float(val2)) => {
                val1.partial_cmp(&val2)
            }
        }
    }
}

impl ops::Add for NumericType {
    type Output = NumericType;

    fn add(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs + rhs)
            },
            (NumericType::Integer(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs as f32 + rhs)
            },
            (NumericType::Float(lhs), NumericType::Integer(rhs)) => {
                NumericType::Float(lhs + rhs as f32)
            },
            (NumericType::Float(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs + rhs)
            }
        }
    }
}

impl ops::BitAnd for NumericType {
    type Output = NumericType;

    fn bitand(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs & rhs)
            },
            _ => panic!("Bitwise AND applies to Value::Number(Integer)")
        }
    }
}

impl ops::BitOr for NumericType {
    type Output = NumericType;

    fn bitor(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs | rhs)
            },
            _ => panic!("Bitwise OR applies to Value::Number(Integer)")
        }
    }
}

impl ops::BitXor for NumericType {
    type Output = NumericType;

    fn bitxor(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs ^ rhs)
            },
            _ => panic!("Bitwise XOR applies to Value::Number(Integer)")
        }
    }
}

impl ops::Div for NumericType {
    type Output = NumericType;

    fn div(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Float(lhs as f32 / rhs as f32)
            },
            (NumericType::Integer(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs as f32 / rhs)
            },
            (NumericType::Float(lhs), NumericType::Integer(rhs)) => {
                NumericType::Float(lhs / rhs as f32)
            },
            (NumericType::Float(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs / rhs)
            }
        }
    }
}

impl ops::Mul for NumericType {
    type Output = NumericType;

    fn mul(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs * rhs)
            },
            (NumericType::Integer(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs as f32 * rhs)
            },
            (NumericType::Float(lhs), NumericType::Integer(rhs)) => {
                NumericType::Float(lhs * rhs as f32)
            },
            (NumericType::Float(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs * rhs)
            }
        }
    }
}

impl ops::Neg for NumericType {
    type Output = NumericType;

    fn neg(self) -> NumericType {
        match self {
            NumericType::Integer(val) => NumericType::Integer(-val),
            NumericType::Float(val)   => NumericType::Float(-val)
        }
    }
}

impl ops::Not for NumericType {
    type Output = NumericType;

    fn not(self) -> NumericType {
        match self {
            NumericType::Integer(val) => NumericType::Integer(!val),
            NumericType::Float(_)   => panic!("bad operand type for unary ~")
        }
    }
}

impl ops::Rem for NumericType {
    type Output = NumericType;

    fn rem(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs % rhs)
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Shl<NumericType> for NumericType {
    type Output = NumericType;

    fn shl(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs << rhs)
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Shr<NumericType> for NumericType {
    type Output = NumericType;

    fn shr(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs >> rhs)
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Sub for NumericType {
    type Output = NumericType;

    fn sub(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs - rhs)
            },
            (NumericType::Integer(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs as f32 - rhs)
            },
            (NumericType::Float(lhs), NumericType::Integer(rhs)) => {
                NumericType::Float(lhs - rhs as f32)
            },
            (NumericType::Float(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs - rhs)
            }
        }
    }
}
