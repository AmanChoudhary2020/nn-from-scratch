mod value;
use value::Value;

fn main() {
    // EXAMPLE 1:
    let x1 = Value::create(2.0);
    let x2 = Value::create(0.0);

    let w1 = Value::create(-3.0);
    let w2 = Value::create(1.0);

    let b = Value::create(6.8813735870195432);

    let x1w1 = &x1 * &w1;
    let x2w2 = &x2 * &w2;
    let x1w1x2w2 = &x1w1 + &x2w2;
    let n = &x1w1x2w2 + &b;
    let o = n.tanh();

    o.backward();

    println!("Example 1");
    println!("x1: {:?}\n", x1);
    println!("x2: {:?}\n", x2);
    println!("w1: {:?}\n", w1);
    println!("w2: {:?}\n", w2);
    println!("b: {:?}\n", b);
    println!("x1w1: {:?}\n", x1w1);
    println!("x2w2: {:?}\n", x2w2);
    println!("x1w1x2w2: {:?}\n", x1w1x2w2);
    println!("n: {:?}\n", n);
    println!("o: {:?}\n", o);

    // EXAMPLE 2:
    let a = Value::create(-2.0);
    let b = Value::create(3.0);
    let d = &a * &b;
    let e = &a + &b;
    let f = &d * &e;

    f.backward();

    println!("Example 2");
    println!("a: {:?}\n", a);
    println!("b: {:?}\n", b);
    println!("d: {:?}\n", d);
    println!("e: {:?}\n", e);
    println!("f: {:?}\n", f);
}
