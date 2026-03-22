#[cfg(test)]
#[path = "./interpreter_test.rs"]
mod interpreter_test;

use std::collections::HashMap;

use crate::asm::parser::{Address, Form, Label, SExp, Symbol};

/// eval_aar, aar = address_arithmetic
/// only able to to evaluate address arithmetic computation
pub fn eval_aar(sexp: &SExp, label_addresses: &HashMap<Label, Address>) -> Result<Address, String> {
    match sexp {
        SExp::Symbol(Symbol::Label(lbl)) => {
            let may_address = label_addresses.get(lbl);
            if let Some(address) = may_address {
                Ok(*address)
            } else {
                Err(format!("undefined label: {:?}", lbl))
            }
        }
        SExp::Form(form) => {
            let op_sym = match &form.op {
                Symbol::Sym(sym) => sym,
                invalid => {
                    return Err(format!(
                        "illegal arithmetic address operator: {:?}",
                        invalid
                    ));
                }
            };

            match op_sym.as_str() {
                "-" => eval_aar_minus(form, label_addresses),
                "+" => eval_aar_plus(form, label_addresses),
                invalid => {
                    return Err(format!(
                        "illegal arithmetic address operator: {:?}",
                        invalid
                    ));
                }
            }
        }
        invalid => Err(format!(
            "illegal address arithmetic expression: {:?}",
            invalid
        )),
    }
}

fn eval_aar_minus(
    form: &Form,
    label_addresses: &HashMap<Label, Address>,
) -> Result<Address, String> {
    if form.exps.len() <= 1 {
        return Err(format!(
            "-: invalid number or arguments {}",
            form.exps.len()
        ));
    }

    let mut address = eval_aar(&form.exps[0], label_addresses)?;
    for i in 1..(form.exps.len()) {
        let exp = &form.exps[i];
        let exp_address = eval_aar(exp, label_addresses)?;
        if address.0 < exp_address.0 {
            return Err("-: negative address".to_string());
        }
        address = Address(address.0 - exp_address.0);
    }
    Ok(address)
}

fn eval_aar_plus(
    form: &Form,
    label_addresses: &HashMap<Label, Address>,
) -> Result<Address, String> {
    let mut address = Address(0);
    for exp in &form.exps {
        let exp_address = eval_aar(exp, label_addresses)?;
        address = Address(address.0 + exp_address.0);
    }
    Ok(address)
}

pub const CONST_OP_SHIFT_LEFT: &str = "<<";
pub const CONST_OP_BITWISE_OR: &str = "|";

/// eval_const
/// const expression evaluation. Every variable in the expression tree must evaluate to a constant value.
/// If not, an error is returned.
pub fn eval_const(exp: &SExp, const_values: &HashMap<String, i64>) -> Result<i64, String> {
    match exp {
        SExp::Immediate(val) => Ok(*val),
        SExp::Symbol(Symbol::Sym(name)) => {
            let maybe_val = const_values.get(name);
            if let Some(val) = maybe_val {
                Ok(*val)
            } else {
                Err(format!("no constant value for symbol: {}", name))
            }
        }
        SExp::Form(form) => {
            let op_name = match &form.op {
                Symbol::Sym(op_name) => op_name,
                illegal => return Err(format!("illegal constant form: {:?}", illegal)),
            };
            match op_name.as_str() {
                CONST_OP_SHIFT_LEFT => eval_const_left_shift(form, const_values),
                CONST_OP_BITWISE_OR => eval_const_bitwise_or(form, const_values),
                _ => Err(format!("illegal constant op: {:?}", op_name)),
            }
        }
        illegal => Err(format!("not a constant expression: {:?}", illegal)),
    }
}

fn eval_const_left_shift(form: &Form, const_values: &HashMap<String, i64>) -> Result<i64, String> {
    if form.exps.len() != 2 {
        return Err(format!("<<: needs exactly 2 parameters"));
    }

    let v = eval_const(&form.exps[0], const_values)?;
    let shift = eval_const(&form.exps[1], const_values)?;
    Ok(v << shift)
}

fn eval_const_bitwise_or(form: &Form, const_values: &HashMap<String, i64>) -> Result<i64, String> {
    if form.exps.len() != 2 {
        return Err(format!("|: needs exactly 2 parameters"));
    }

    let v1 = eval_const(&form.exps[0], const_values)?;
    let v2 = eval_const(&form.exps[1], const_values)?;
    Ok(v1 | v2)
}
