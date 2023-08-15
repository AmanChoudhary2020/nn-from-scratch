mod value;

use value::Value;

fn main() {
    // let w1 = Value {
    //     data: -3.0,
    //     grad: Default::default(),
    //     backward: None,
    //     prev: Default::default(),
    //     op: Default::default(),
    // };
    // let w2 = Value {
    //     data: 1.0,
    //     grad: Default::default(),
    //     backward: None,
    //     prev: Default::default(),
    //     op: Default::default(),
    // };

    // let b = Value {
    //     data: 6.8813735870195432,
    //     grad: Default::default(),
    //     backward: None,
    //     prev: Default::default(),
    //     op: Default::default(),
    // };

    // let x1w1 = &x1 * &w1;
    // let x2w2 = &x2 * &w2;
    // let x1w1x2w2 = &x1w1 + &x2w2;
    // let n = &x1w1x2w2 + &b;
    // let mut o = n.tanh();

    // o.grad = 1.0;
    // o.backward.unwrap()(&mut o);

    // println!("{:?}", o.prev[0].grad);
    // println!("n grad: {:?}", n.grad);
}
