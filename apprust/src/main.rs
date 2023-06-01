use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: i32,
    nome: String,
    descricao: String,
    categoria: Categoria,
    modo: Modo,
    como_estou: String,
    como_quero_estar: String,
    o_que_farei: String,
    quando_farei: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Modo {
    CARENCIA,
    NECESSIDADE,
    FRAQUEZA,
    LIMITE
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Categoria {
    Familia,
    Social,
    Trabalho,
    Financeiro,
    VidadeOracao,
    Humor,
    Emocoes,
    Sentimentos,
    Afetos,
    Paixões
}

fn ler_arquivo_json() -> Vec<Item> {
    let mut file = File::open("data.json").expect("Arquivo não encontrado");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Não foi possível ler o arquivo");
    let data: Vec<Item> = serde_json::from_str(&contents)
    .expect("Não foi possível desserializar o JSON");
    data
}

fn escrever_arquivo_json(items: Vec<Item>) {
    println!("Escrevendo arquivo JSON");
    let file_name = "data.json";
    let mut file = File::create(file_name).expect("Erro ao criar arquivo");
    let transaction_str = serde_json::to_string(&items).unwrap();
    file.write_all(transaction_str.as_bytes()).expect("Erro ao escrever no arquivo");
}

fn mostrar_opcoes_modo() {
    println!("-------------------------");
    println!("1 - CARÊNCIA");
    println!("2 - NECESSIDADE");
    println!("3 - FRAQUEZA");
    println!("4 - LIMITE");
    println!("Digite uma opção de modo: ");
}

fn mostrar_opcoes_categoria() {
    println!("-------------------------");
    println!("1 - Família");
    println!("2 - Social");
    println!("3 - Trabalho");
    println!("4 - Financeiro");
    println!("5 - Vida de Oração");
    println!("6 - Humor");
    println!("7 - Emoções");
    println!("8 - Sentimentos");
    println!("9 - Afetos");
    println!("10 - Paixões");
    println!("Digite uma opção de categoria: ");
}

fn adicionar(nome: String, descricao: String, modo: i32, categoria: i32, como_estou: String, como_quero_estar: String, o_que_farei: String, quando_farei: String) {
    let mut file: Vec<Item> = ler_arquivo_json();
    println!("Tamanho do arquivo: {:?}", file.len());
    let id = if file.len() != 0 { file[file.len()-1].id + 1 } else { 1 };
    let modo_escolha = match modo {
        1 => Modo::CARENCIA,
        2 => Modo::NECESSIDADE,
        3 => Modo::FRAQUEZA,
        _ => Modo::LIMITE,
    };
    let categoria_escolha = match categoria {
        1 => Categoria::Familia,
        2 => Categoria::Social,
        3 => Categoria::Trabalho,
        4 => Categoria::Financeiro,
        5 => Categoria::VidadeOracao,
        6 => Categoria::Humor,
        7 => Categoria::Emocoes,
        8 => Categoria::Sentimentos,
        9 => Categoria::Afetos,
        _ => Categoria::Paixões
    };
    let new_transaction = Item {
        id: id,
        nome: nome,
        descricao: descricao,
        modo: modo_escolha,
        categoria: categoria_escolha,
        como_estou: como_estou,
        como_quero_estar: como_quero_estar,
        o_que_farei: o_que_farei,
        quando_farei: quando_farei
    };
    file.push(new_transaction);
    escrever_arquivo_json(file);
    println!("Adicionando transação");
}

fn remover(id: i32) {
    let mut file: Vec<Item> = ler_arquivo_json();
    let index = file.iter().position(|x| x.id == id).unwrap();
    file.remove(index);
    escrever_arquivo_json(file);
    println!("Operação removida");
}

fn listar() {
    let file = ler_arquivo_json();
    println!("Transações: {:?} ", file.len());
    for item in file.into_iter() {
        println!("-------------------------");
        println!("Id: {:?}", item.id);
        println!("Nome: {:?}", item.nome);
        println!("Descrição: {:?}", item.descricao);
        println!("Modo: {:?}", item.modo);
        println!("Categoria: {:?}", item.categoria);
        println!("Como estou: {:?}", item.como_estou);
        println!("Como quero estar: {:?}", item.como_quero_estar);
        println!("O que farei: {:?}", item.o_que_farei);
        println!("Quando farei: {:?}", item.quando_farei);
        println!("-------------------------");
    }
}

fn editar(id: i32) {
    let mut file = ler_arquivo_json();
    let mut item = file.iter_mut()
        .find(|x| x.id == id).unwrap();
    println!("-------------------------");
    println!("Id: {:?}", item.id);
    println!("Nome: {:?}", item.nome);
    println!("Descrição: {:?}", item.descricao);
    println!("Modo: {:?}", item.modo);
    println!("Categoria: {:?}", item.categoria);
    println!("Como estou: {:?}", item.como_estou);
    println!("Como quero estar: {:?}", item.como_quero_estar);
    println!("O que farei: {:?}", item.o_que_farei);
    println!("Quando farei: {:?}", item.quando_farei);
    println!("-------------------------");
    println!("Digite o nome: ");
    let mut nome = String::new();
    std::io::stdin().read_line(&mut nome).expect("Erro ao ler nome");
    println!("Digite a descrição: ");
    let mut descricao = String::new();
    std::io::stdin().read_line(&mut descricao).expect("Erro ao ler descrição");
    println!("Digite a modo: ");
    let mut modo = String::new();
    std::io::stdin().read_line(&mut modo).expect("Erro ao ler operação");
    println!("Digite a categoria: ");
    let mut categoria = String::new();
    std::io::stdin().read_line(&mut categoria).expect("Erro ao ler categoria");
    println!("Digite como estou: ");
    let mut como_estou = String::new();
    std::io::stdin().read_line(&mut como_estou).expect("Erro ao ler como estou");
    println!("Digite como quero estar: ");
    let mut como_quero_estar = String::new();
    std::io::stdin().read_line(&mut como_quero_estar).expect("Erro ao ler como quero estar");
    println!("Digite o que farei: ");
    let mut o_que_farei = String::new();
    std::io::stdin().read_line(&mut o_que_farei).expect("Erro ao ler o que farei");
    println!("Digite quando farei: ");
    let mut quando_farei = String::new();
    std::io::stdin().read_line(&mut quando_farei).expect("Erro ao ler quando farei");
    item.nome = nome.trim().to_string();
    item.descricao = descricao.trim().to_string();
    item.categoria = match categoria.trim().parse::<i32>().unwrap() {
        1 => Categoria::Familia,
        2 => Categoria::Social,
        3 => Categoria::Trabalho,
        4 => Categoria::Financeiro,
        5 => Categoria::VidadeOracao,
        6 => Categoria::Humor,
        7 => Categoria::Emocoes,
        8 => Categoria::Sentimentos,
        9 => Categoria::Afetos,
        _ => Categoria::Paixões
    };
    item.modo = match modo.trim().parse::<i32>().unwrap() {
        1 => Modo::CARENCIA,
        2 => Modo::NECESSIDADE,
        3 => Modo::FRAQUEZA,
        _ => Modo::LIMITE,
    };
    item.como_estou = como_estou.trim().to_string();
    item.como_quero_estar = como_quero_estar.trim().to_string();
    item.o_que_farei = o_que_farei.trim().to_string();
    item.quando_farei = quando_farei.trim().to_string();
    escrever_arquivo_json(file);
    println!("Operação editada");
}


fn menu() {
    println!("1 - Adicionar");
    println!("2 - Remover");
    println!("3 - Listar");
    println!("4 - Editar");
    println!("5 - Sair");
    println!("Digite uma opção: ");
    let mut opcao = String::new();
    std::io::stdin().read_line(&mut opcao).expect("Erro ao ler opção");
    match opcao.trim().parse::<i32>() {
        Ok(1) => {
            println!("Digite o nome: ");
            let mut nome = String::new();
            std::io::stdin().read_line(&mut nome).expect("Erro ao ler nome");
            println!("Digite a descrição: ");
            let mut descricao = String::new();
            std::io::stdin().read_line(&mut descricao).expect("Erro ao ler descrição");
            mostrar_opcoes_modo();
            let mut modo = String::new();
            std::io::stdin().read_line(&mut modo).expect("Erro ao ler modo");
            mostrar_opcoes_categoria();
            let mut categoria = String::new();
            std::io::stdin().read_line(&mut categoria).expect("Erro ao ler categoria");
            println!("Digite como estou: ");
            let mut como_estou = String::new();
            std::io::stdin().read_line(&mut como_estou).expect("Erro ao ler como estou");
            println!("Digite como quero estar: ");
            let mut como_quero_estar = String::new();
            std::io::stdin().read_line(&mut como_quero_estar).expect("Erro ao ler como quero estar");
            println!("Digite o que farei: ");
            let mut o_que_farei = String::new();
            std::io::stdin().read_line(&mut o_que_farei).expect("Erro ao ler o que farei");
            println!("Digite quando farei: ");
            let mut quando_farei = String::new();
            std::io::stdin().read_line(&mut quando_farei).expect("Erro ao ler quando farei");
            adicionar(
                nome.trim().to_string(), 
                descricao.trim().to_string(),
                modo.trim().parse::<i32>().unwrap(), 
                categoria.trim().parse::<i32>().unwrap(),
                como_estou.trim().to_string(),
                como_quero_estar.trim().to_string(),
                o_que_farei.trim().to_string(),
                quando_farei.trim().to_string()
            );
        },
        Ok(2) => {
            println!("Digite o id da transação: ");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id).expect("Erro ao ler id");
            remover(id.trim().parse::<i32>().unwrap());
        },
        Ok(3) => {
            listar();
        },
        Ok(4) => {
            println!("Digite o id da transação: ");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id).expect("Erro ao ler id");
            editar(id.trim().parse::<i32>().unwrap());
        },
        Ok(5) => {
            println!("Saindo...");
            std::process::exit(0);
        },
        _ => {
            println!("Opção inválida");
        }
    }
}

fn main() {
    loop {
        menu();
    }   
}
