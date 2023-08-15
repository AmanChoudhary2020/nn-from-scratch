use std::cell::{Ref, RefCell};
use std::ops::{Add, Deref};
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq)]
pub struct Value(Rc<RefCell<ValueInternal>>);

impl Value {
    // pub fn from<T>(t: T) -> Value
    // where
    //     T: Into<ValueInternal>,
    // {
    //     t.into()
    // }

    fn new(value: ValueInternal) -> Value {
        Value(Rc::new(RefCell::new(value)))
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

fn add(a: &Value, b: &Value) -> Value {
    let result = a.borrow().data + b.borrow().data;

    let prop_fn: PropagateFn = |out| {
        let mut first = out.previous[0].borrow_mut();
        let mut second = out.previous[1].borrow_mut();

        // first.grad += out.grad;
        // second.grad += out.grad;
    };

    Value::new(ValueInternal {
        data: result,
        gradient: Default::default(),
        propagate: Some(prop_fn),
        previous: vec![a.clone(), b.clone()],
        operation: Some("+".to_string()),
        label: None,
    })
}

type PropagateFn = fn(value: &Ref<ValueInternal>);

// // #[derive(Clone, Default, Debug)]
// pub struct ValueInternal {
//     pub data: f64,
//     pub grad: f64,
//     pub backward: Option<PropagateFn>,
//     pub prev: Vec<Value>,
//     pub op: char,
// }

pub struct ValueInternal {
    data: f64,
    gradient: f64,
    label: Option<String>,
    operation: Option<String>,
    previous: Vec<Value>,
    propagate: Option<PropagateFn>,
}

impl PartialEq for ValueInternal {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.gradient == other.gradient
            && self.label == other.label
            && self.operation == other.operation
            && self.previous == other.previous
    }
}

impl Eq for ValueInternal {}

// impl<'a> fmt::Display for ValueInternal {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Data: {}", self.data)
//     }
// }

// ---------------- ADD ---------------- //
// impl Add<ValueInternal> for ValueInternal {
//     type Output = ValueInternal;

//     fn add(self, other: ValueInternal) -> Self::Output {
//         add(&self, &other)
//     }
// }

// impl<'a, 'b> Add<&'b ValueInternal> for &ValueInternal {
//     type Output = ValueInternal;

//     fn add(self, other: &'b ValueInternal) -> Self::Output {
//         add(self, other)
//     }
// }

// impl<'a> Add<&ValueInternal> for ValueInternal {
//     type Output = ValueInternal;

//     fn add(self, other: &ValueInternal) -> Self::Output {
//         add(&self, other)
//     }
// }

// fn add(a: &ValueInternal, b: &ValueInternal) -> ValueInternal {
//     let result = a.data + b.data;

//     // let prop_fn: PropagateFn = |out| {
//     //     // let first = out.prev.split_first_mut().unwrap();
//     //     // // let mut second = out.prev.split_last_mut().unwrap();
//     //     // let test = first.0;
//     //     // test.grad += 1.0;
//     //     // // first.1[0].grad += out.grad;
//     //     let mut first = out.prev[0].borrow_mut();
//     //     let mut second = out.prev[1].borrow_mut();

//     //     first.grad += out.grad;
//     //     second.grad += out.grad;
//     // };

//     ValueInternal {
//         data: result,
//         grad: Default::default(),
//         backward: None,
//         prev: Default::default(),
//         op: '+',
//     }
// }
