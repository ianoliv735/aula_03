trait Descritivel {
    fn descrever(&self) -> String;
}

struct Pessoa{
    nome: String,
    idade: u32,
}

impl Descritivel for Pessoa {
    fn descrever(&self) -> String{
        format!("nome: {}, idade: {}", self.nome, self.idade)
    }

}

fn mostrar_descricao<T: Descritivel>(item: T){
    println!("{}", item.descrever ());
}

fn main(){
    let pessoa = Pessoa {
        nome: String::from ("João"),
        idade: 30,
    };
    let pessoa2 = Pessoa {
        nome: String::from("Ian"),
        idade: 22,
    };

    let pessoa3 = Pessoa {
        nome: String::from("Márcia"),
        idade: 46,
    };

    let pessoa4 = Pessoa{
        nome: String::from("Ivan"),
        idade: 55,
    };

    mostrar_descricao(pessoa);
    mostrar_descricao(pessoa2);
    mostrar_descricao(pessoa3);
    mostrar_descricao(pessoa4);
}