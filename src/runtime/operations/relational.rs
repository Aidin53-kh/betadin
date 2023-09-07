// use crate::runtime::value::Value;

// pub fn greater(lhs: Value, rhs: Value) -> Result<Value, String> {

//     match lhs {
//         Value::Int(l) => match rhs {
//             Value::Int(r) => Ok(Value::Bool(l > r)),
//             Value::Float(r) => Ok(Value::Bool(l as f32 > r)),
//             other => error_other!(Int, Greater, other),
//         },
//         Value::Float(l) => match rhs {
//             Value::Int(r) => Ok(Value::Bool(l > r as f32)),
//             other => error_other!(Float, Greater, other),
//         },
//         Value::String(_) => match rhs {
//             other => error_other!(String, Greater, other),
//         },
//         Value::Bool(_) => match rhs {
//             other => error_other!(Bool, Greater, other),
//         },
//         Value::Null => match rhs {
//             other => error_other!(Null, LowerEquals, other),
//         },
//         Value::List(_) => todo!(),
//         Value::BuiltInFn(_) => todo!(),
//         Value::BuiltInMethod(_) => todo!(),
//     }
// }

// pub fn lower(lhs: Value, rhs: Value) -> Result<Value, String> {
//     match lhs {
//         Value::Int(l) => match rhs {
//             Value::Int(r) => Ok(Value::Bool(l < r)),
//             other => error_other!(Int, Lower, other),
//         },
//         Value::Float(l) => match rhs {
//             Value::Int(r) => Ok(Value::Bool(l < r as f32)),
//             other => error_other!(Float, Lower, other),
//         },
//         Value::String(_) => match rhs {
//             other => error_other!(String, Lower, other),
//         },
//         Value::Bool(_) => match rhs {
//             other => error_other!(Bool, Lower, other),
//         },
//         Value::Null => match rhs {
//             other => error_other!(Null, LowerEquals, other),
//         },
//         Value::List(_) => todo!(),
//         Value::BuiltInFn(_) => todo!(),
//         Value::BuiltInMethod(_) => todo!(),
//     }
// }

// pub fn equals(lhs: Value, rhs: Value) -> Result<Value, String> {
//     match lhs {
//         Value::Int(l) => match rhs {
//             Value::Int(r) => Ok(Value::Bool(l == r)),
//             other => error_other!(Int, Equals, other),
//         },
//         Value::Float(l) => match rhs {
//             Value::Float(r) => Ok(Value::Bool(l == r)),
//             other => error_other!(Float, Equals, other),
//         },
//         Value::String(l) => match rhs {
//             Value::String(r) => Ok(Value::Bool(l == r)),
//             other => error_other!(String, Equals, other),
//         },
//         Value::Bool(l) => match rhs {
//             Value::Bool(r) => Ok(Value::Bool(l == r)),
//             other => error_other!(Bool, Equals, other),
//         },
//         Value::Null => match rhs {
//             other => error_other!(Null, LowerEquals, other),
//         },
//         Value::List(_) => todo!(),
//         Value::BuiltInFn(_) => todo!(),
//         Value::BuiltInMethod(_) => todo!(),
//     }
// }

// pub fn not_equals(lhs: Value, rhs: Value) -> Result<Value, String> {
//     match lhs {
//         Value::Int(l) => match rhs {
//             Value::Int(r) => Ok(Value::Bool(l != r)),
//             other => error_other!(Int, NotEquals, other),
//         },
//         Value::Float(l) => match rhs {
//             Value::Float(r) => Ok(Value::Bool(l != r)),
//             other => error_other!(Float, NotEquals, other),
//         },
//         Value::String(l) => match rhs {
//             Value::String(r) => Ok(Value::Bool(l != r)),
//             other => error_other!(String, NotEquals, other),
//         },
//         Value::Bool(l) => match rhs {
//             Value::Bool(r) => Ok(Value::Bool(l != r)),
//             other => error_other!(Bool, NotEquals, other),
//         },
//         Value::Null => match rhs {
//             other => error_other!(Null, LowerEquals, other),
//         },
//         Value::List(_) => todo!(),
//         Value::BuiltInFn(_) => todo!(),
//         Value::BuiltInMethod(_) => todo!(),
//     }
// }

// pub fn greater_equals(lhs: Value, rhs: Value) -> Result<Value, String> {
//     match lhs {
//         Value::Int(l) => match rhs {
//             Value::Int(r) => Ok(Value::Bool(l >= r)),
//             other => error_other!(Int, GreaterEquals, other),
//         },
//         Value::Float(l) => match rhs {
//             Value::Float(r) => Ok(Value::Bool(l >= r)),
//             other => error_other!(Float, GreaterEquals, other),
//         },
//         Value::String(_) => match rhs {
//             other => error_other!(String, GreaterEquals, other),
//         },
//         Value::Bool(_) => match rhs {
//             other => error_other!(Bool, GreaterEquals, other),
//         },
//         Value::Null => match rhs {
//             other => error_other!(Bool, GreaterEquals, other),
//         },
//         Value::List(_) => todo!(),
//         Value::BuiltInFn(_) => todo!(),
//         Value::BuiltInMethod(_) => todo!(),
//     }
// }

// pub fn lower_equals(lhs: Value, rhs: Value) -> Result<Value, String> {
//     match lhs {
//         Value::Int(l) => match rhs {
//             Value::Int(r) => Ok(Value::Bool(l <= r)),
//             other => error_other!(Int, LowerEquals, other),
//         },
//         Value::Float(l) => match rhs {
//             Value::Float(r) => Ok(Value::Bool(l <= r)),
//             other => error_other!(Float, LowerEquals, other),
//         },
//         Value::String(_) => match rhs {
//             other => error_other!(String, LowerEquals, other),
//         },
//         Value::Bool(_) => match rhs {
//             other => error_other!(Bool, LowerEquals, other),
//         },
//         Value::Null => match rhs {
//             other => error_other!(Null, LowerEquals, other),
//         },
//         Value::List(_) => todo!(),
//         Value::BuiltInFn(_) => todo!(),
//         Value::BuiltInMethod(_) => todo!(),
//     }
// }
