# 🧮 Tabuada Completa em Rust

<img src="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-logo-white.svg" alt="Rust Logo White" width="180" />

<img src="https://www.rustacean.net/assets/rustacean-flat-happy.png" alt="Ferris the Crab" width="180" />


Este projeto exibe as tabuadas de **1 a 10**, utilizando **dois loops `for`**.  
O objetivo é praticar lógica, repetição e organização de código em Rust — ideal para iniciantes.

## 📌 Funcionalidade

O programa:

- percorre os números de **1 a 10**
- para cada número (1..=10), gera sua tabuada completa
- organiza as tabuadas com um separador visual

Exemplo de saída:

1 X 1 = 1
1 X 2 = 2
...
1 X 10 = 10
==========
2 X 1 = 2
2 X 2 = 4
...


## 🧠 O que é praticado neste projeto?

- Estrutura de repetição `for`
- Loops aninhados (loop dentro de loop)
- Manipulação de variáveis
- Multiplicação e lógica simples
- Impressão formatada com `println!`

## 📦 Código Fonte

```rust
fn main() {
    let mut numero = 1; // numero começa do 1

    for _ in 1..=10 {
        for i in 1..=10 { // repete ate o 10
            println!("{} X {} = {}", i, numero, numero * i);
        }

        println!("===============================");
        numero += 1; // aumenta o numero em 1
    }
}
