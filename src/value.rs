use std::borrow::BorrowMut;
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Clone, Default, Debug)]
pub struct Value {
    pub data: f64,
    pub grad: f64,
    pub backward: Option<PropagateFn>,
    pub prev: Vec<Value>,
    pub op: char,
}

impl Value {
    pub fn tanh(&self) -> Value {
        let result = self.data.tanh();
        let prop_fn: PropagateFn = |out| {
            let previous = out.prev.split_at_mut(1).0[0].borrow_mut();
            previous.grad += (1.0 - out.data.powi(2)) * out.grad;
        };

        Value {
            data: result,
            grad: Default::default(),
            backward: Some(prop_fn),
            prev: vec![self.clone()],
            op: '+',
        }
    }
}

impl<'a> fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data: {}", self.data)
    }
}

type PropagateFn = fn(value: &mut Value);

// ---------------- ADD ---------------- //
impl<'a> Add<Value> for Value {
    type Output = Value;

    fn add(self, other: Value) -> Self::Output {
        add(&self, &other)
    }
}

impl<'a, 'b> Add<&'b Value> for &'a Value {
    type Output = Value;

    fn add(self, other: &'b Value) -> Self::Output {
        add(self, other)
    }
}

impl<'a> Add<&Value> for Value {
    type Output = Value;

    fn add(self, other: &Value) -> Self::Output {
        add(&self, other)
    }
}

fn add<'a>(a: &Value, b: &Value) -> Value {
    let result = a.data + b.data;

    let prop_fn: PropagateFn = |out| {
        let previous = out.prev.split_at_mut(1);
        previous.0[0].grad += out.grad;
        previous.1[0].grad += out.grad;
    };

    Value {
        data: result,
        grad: Default::default(),
        backward: Some(prop_fn),
        prev: vec![a.clone(), b.clone()],
        op: '+',
    }
}

// ---------------- MUL ---------------- //
impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Self::Output {
        mul(&self, &other)
    }
}

impl<'a, 'b> Mul<&'b Value> for &'a Value {
    type Output = Value;

    fn mul(self, other: &'b Value) -> Self::Output {
        mul(self, other)
    }
}

impl Mul<&Value> for Value {
    type Output = Value;

    fn mul(self, other: &Value) -> Self::Output {
        mul(&self, other)
    }
}

fn mul(a: &Value, b: &Value) -> Value {
    let result = a.data * b.data;

    let prop_fn: PropagateFn = |out| {
        let previous = out.prev.split_at_mut(1);
        previous.0[0].grad += previous.1[0].grad * out.grad;
        previous.1[0].grad += previous.0[0].grad * out.grad;
    };

    Value {
        data: result,
        grad: Default::default(),
        backward: Some(prop_fn),
        prev: vec![a.clone(), b.clone()],
        op: '*',
    }
}
