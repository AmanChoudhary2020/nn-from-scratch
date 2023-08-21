/*
NOTES:
`prev` member of InnerValue:
- goal: should contain information pertaining to all previous nodes leading up to the node
        represented by the current InnerValue
- tried storing a vector of references to previous InnerValue objects
    - issue with lifetimes
- RefCell: gives runtime checked borrowing (as opposed to compile-time checked borrowing)
- Rc: allows nodes in neural network graph to be sharable
*/

use std::cell::{Ref, RefCell};
use std::ops::{Add, Deref, Mul};
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Value(Rc<RefCell<InnerValue>>);

impl Value {
    pub fn new(value: InnerValue) -> Value {
        Value(Rc::new(RefCell::new(value)))
    }

    pub fn backward(&self) {
        let borrowed_value = self.borrow();
        if let Some(prop_fn) = borrowed_value.backward {
            prop_fn(&borrowed_value);
        }
    }
}

impl<T: Into<f64>> From<T> for Value {
    fn from(data: T) -> Value {
        Value::new(InnerValue::new(
            data.into(),
            Default::default(),
            Vec::new(),
            None,
        ))
    }
}

impl Deref for Value {
    type Target = Rc<RefCell<InnerValue>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ------------ ADD ------------ //
impl Add<Value> for Value {
    type Output = Value;

    fn add(self, other: Value) -> Self::Output {
        add(&self, &other)
    }
}

impl<'a, 'b> Add<&'b Value> for &Value {
    type Output = Value;

    fn add(self, other: &'b Value) -> Self::Output {
        add(self, other)
    }
}

fn add(a: &Value, b: &Value) -> Value {
    let result = a.borrow().data + b.borrow().data;

    let prop_fn: PropagateFn = |out| {
        let mut first = out.prev[0].borrow_mut();
        let mut second = out.prev[1].borrow_mut();

        first.grad += out.grad;
        second.grad += out.grad;
    };

    Value::new(InnerValue {
        data: result,
        grad: 0.0,
        op: '+',
        prev: vec![a.clone(), b.clone()], // cloning a Value will increase the count on the reference-counting (rc) pointer
        backward: Some(prop_fn),
    })
}

// ------------ MUL ------------ //
impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Self::Output {
        mul(&self, &other)
    }
}

impl<'a, 'b> Mul<&'b Value> for &Value {
    type Output = Value;

    fn mul(self, other: &'b Value) -> Self::Output {
        mul(self, other)
    }
}

fn mul(a: &Value, b: &Value) -> Value {
    let result = a.borrow().data * b.borrow().data;

    let prop_fn: PropagateFn = |out| {
        let mut first = out.prev[0].borrow_mut();
        let mut second = out.prev[1].borrow_mut();

        first.grad += second.data * out.grad;
        second.grad += first.data * out.grad;
    };

    Value::new(InnerValue {
        data: result,
        grad: 0.0,
        op: '*',
        prev: vec![a.clone(), b.clone()], // cloning a Value will increase the count on the reference-counting (rc) pointer
        backward: Some(prop_fn),
    })
}

type PropagateFn = fn(value: &Ref<InnerValue>);

#[derive(Clone)]
pub struct InnerValue {
    data: f64,
    grad: f64,
    op: char,
    prev: Vec<Value>,
    backward: Option<PropagateFn>,
}

impl InnerValue {
    fn new(data: f64, op: char, prev: Vec<Value>, propagate: Option<PropagateFn>) -> InnerValue {
        InnerValue {
            data,
            grad: 0.0,
            op,
            prev,
            backward: propagate,
        }
    }
}

impl PartialEq for InnerValue {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.grad == other.grad
            && self.op == other.op
            && self.prev == other.prev
    }
}

impl Eq for InnerValue {}

impl std::fmt::Debug for InnerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InnerValue")
            .field("data", &self.data)
            .field("gradient", &self.grad)
            .field("operation", &self.op)
            .field("previous", &self.prev)
            .finish()
    }
}
