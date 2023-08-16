use std::cell::{Ref, RefCell};
use std::ops::{Add, Deref};
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Value(Rc<RefCell<ValueInternal>>);

impl Value {
    pub fn new(value: ValueInternal) -> Value {
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
    fn from(t: T) -> Value {
        Value::new(ValueInternal::new(
            t.into(),
            Default::default(),
            Vec::new(),
            None,
        ))
    }
}

impl Deref for Value {
    type Target = Rc<RefCell<ValueInternal>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

    Value::new(ValueInternal {
        data: result,
        grad: 2.0, // TODO: Change this back to 0.0
        op: '+',
        prev: vec![a.clone(), b.clone()],
        backward: Some(prop_fn),
    })
}

type PropagateFn = fn(value: &Ref<ValueInternal>);

#[derive(Clone)]
pub struct ValueInternal {
    data: f64,
    grad: f64,
    op: char,
    prev: Vec<Value>,
    backward: Option<PropagateFn>,
}

impl ValueInternal {
    fn new(data: f64, op: char, prev: Vec<Value>, propagate: Option<PropagateFn>) -> ValueInternal {
        ValueInternal {
            data,
            grad: 0.0,
            op: op,
            prev: prev,
            backward: propagate,
        }
    }
}

impl PartialEq for ValueInternal {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.grad == other.grad
            && self.op == other.op
            && self.prev == other.prev
    }
}

impl Eq for ValueInternal {}

// fn add(a: &ValueInternal, b: &ValueInternal) -> ValueInternal {
//     let result = a.data + b.data;

//     let prop_fn: PropagateFn = |out| {
//         // let first = out.prev.split_first_mut().unwrap();
//         // // let mut second = out.prev.split_last_mut().unwrap();
//         // let test = first.0;
//         // test.grad += 1.0;
//         // // first.1[0].grad += out.grad;
//         let mut first = out.prev[0].borrow_mut();
//         let mut second = out.prev[1].borrow_mut();

//         first.grad += out.grad;
//         second.grad += out.grad;
//     };

//     ValueInternal {
//         data: result,
//         grad: Default::default(),
//         op: '+',
//         prev: vec![a.clone(), b.clone()],
//         backward: Some(prop_fn),
//     }
// }

impl std::fmt::Debug for ValueInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueInternal")
            .field("data", &self.data)
            .field("gradient", &self.grad)
            .field("operation", &self.op)
            .field("previous", &self.prev)
            .finish()
    }
}
