use bench_core::Foo;
use rand;

fn main() {
    let mut rng = rand::rng();
    let foo = Foo::random(&mut rng);

    dbg!(&foo);
}
