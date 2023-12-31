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
use std::collections::HashSet;
use std::ops::{Add, Deref, Mul};
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Value(Rc<RefCell<InnerValue>>);

impl Value {
    pub fn create(data: f64) -> Value {
        let value = InnerValue::new(
            data,
            Default::default(),
            Default::default(),
            Default::default(),
        );
        Value(Rc::new(RefCell::new(value)))
    }

    pub fn new(value: InnerValue) -> Value {
        Value(Rc::new(RefCell::new(value)))
    }

    pub fn backward(&self) {
        let mut visited: HashSet<Value> = HashSet::new();
        self.borrow_mut().grad = 1.0;
        self.build_topo(&mut visited, self);
    }

    pub fn tanh(&self) -> Value {
        let result = self.borrow().data.tanh();

        let prop_fn: PropagateFn = |value| {
            let mut previous = value.prev[0].borrow_mut(); // should only have one element in previous array
            previous.grad += (1.0 - value.data.powf(2.0)) * value.grad;
        };

        Value::new(InnerValue::new(
            result,
            "tanh".to_string(),
            vec![self.clone()],
            Some(prop_fn),
        ))
    }

    // Inner function for topological Sort
    fn build_topo(&self, visited: &mut HashSet<Value>, value: &Value) {
        if !visited.contains(&value) {
            visited.insert(value.clone());

            let val = value.borrow();
            if let Some(backward_fn) = val.backward {
                backward_fn(&val);
            }

            for child in &value.borrow().prev {
                self.build_topo(visited, child);
            }
        }
    }
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
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
        let second = out.prev[1].try_borrow_mut();

        first.grad += out.grad;

        match second {
            Ok(mut s) => s.grad += out.grad,
            Err(_) => first.grad += out.grad,
        }
    };

    Value::new(InnerValue {
        data: result,
        grad: 0.0,
        op: "+".to_string(),
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
        let second = out.prev[1].try_borrow_mut();

        match second {
            Ok(mut s) => {
                first.grad += s.data * out.grad;
                s.grad += first.data * out.grad;
            }
            Err(_) => first.grad += (first.data * out.grad) * 2.0,
        }
    };

    Value::new(InnerValue {
        data: result,
        grad: 0.0,
        op: "*".to_string(),
        prev: vec![a.clone(), b.clone()], // cloning a Value will increase the count on the reference-counting (rc) pointer
        backward: Some(prop_fn),
    })
}

type PropagateFn = fn(value: &Ref<InnerValue>);

#[derive(Clone)]
pub struct InnerValue {
    data: f64,
    grad: f64,
    op: String,
    prev: Vec<Value>,
    backward: Option<PropagateFn>,
}

impl InnerValue {
    fn new(data: f64, op: String, prev: Vec<Value>, propagate: Option<PropagateFn>) -> InnerValue {
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

impl std::hash::Hash for InnerValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.to_bits().hash(state);
        self.grad.to_bits().hash(state);
        self.op.hash(state);
        self.prev.hash(state);
    }
}
