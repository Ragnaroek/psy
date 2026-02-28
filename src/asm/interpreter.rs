#[cfg(test)]
#[path = "./interpreter_test.rs"]
mod interpreter_test;

use std::collections::HashMap;

use crate::asm::parser::{Address, Form, Label, SExp, Symbol};

/// aar = address_arithmetic
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
