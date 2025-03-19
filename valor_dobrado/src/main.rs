fn main() {
    // Função para dobrar um número do tipo i32
    fn double(num: i32) -> i32 {
        num * 2
    }

    fn double_int(num: i32) -> i32 {
        num * 2
    }

    // Função que dobra floats
    fn double_float(num: f64) -> f64 {
        num * 2.0
    }

    let int: i32 = 32;
    let big_int = 10;
    let float = 1.2;

    let doubled_int = double_int(int);
    let doubled_big_int = double_int(big_int);
    let doubled_float = double_float(float);

    // Mostrando os resultados
    println!("int original: {}, int dobrado: {}", int, doubled_int);
    println!("big_int original: {}, big_int dobrado: {}", big_int, doubled_big_int);
    println!("float original: {}, float dobrado: {:.2}", float, doubled_float);
}