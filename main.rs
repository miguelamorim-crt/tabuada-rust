fn main() {

    for numero in 1..=10 {
         for i in 1..=10 { // repete ate o 10
             println!("{} X {} = {}", i, numero, numero * i);
        }

        println!("===============================");
        numero += 1; // aumenta o numero em 1
    }
    
}