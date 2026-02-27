use bench_core::foo::Foo;
use rand;

fn main() {
    let mut rng = rand::rng();
    let foo = Foo::random(&mut rng);

    dbg!(&foo);
}
