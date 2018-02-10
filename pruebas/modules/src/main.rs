mod calculadora;
mod group;

fn main() {
    println!("Hello, world!");
    let sum = calculadora::suma(1, 5);
    let res = calculadora::resta(1, 5);
    println!("{}\n{}", sum, res);
    println!("{}\n{}", group::name(), group::nested::name());
    println!("{}", group::remove_spaces("esto tiene espacios"));
}
