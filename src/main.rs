use std::io::Write;
use std::{fs, io};
use walkdir::WalkDir;

// cum sa rezolv problema cu pathul
// optiuni combinate?
//cargo run???
fn parcurgtxt(
    path: &str,
    cuvant: &str,
    flag: &mut i32,
    flag_m: i32,
    flag_i: i32,
    flag_c: i32,
    flag_r: i32,
) -> std::io::Result<()> {
    println!("{}", path);
    let file = fs::read_to_string(path)?;

    let mut count = 1;
    if file.contains(cuvant) {
        println!("\nfound:\npath: {}", path);
        for linie in file.lines() {
            if linie.contains(cuvant) {
                println!("line number: {}\nline: {}", count, linie);
                *flag = 1;
            }
            count += 1;
        }
    }

    Ok(())
}
//sa fac un vector de flaguri?
fn recursivdir(
    comenzi: Vec<String>,
    flag_m: i32,
    flag_i: i32,
    flag_c: i32,
    flag_r: i32,
) -> std::io::Result<()> {
    let mut dir = comenzi[comenzi.len() - 1].clone();
    println!("{}", dir);

    let cuvant = comenzi[comenzi.len() - 2].clone();

    let mut flag = 0;

    dir = dir.replace('\\', "/");

    for entry in WalkDir::new(dir) {
        let entry = entry?;

        if let Some(path) = entry.path().to_str() {
            if path.ends_with(".txt") {
                parcurgtxt(
                    path,
                    cuvant.to_string().as_str(),
                    &mut flag,
                    flag_m,
                    flag_i,
                    flag_c,
                    flag_r,
                )?;
            }
        }
    }

    if flag == 0 {
        println!("Nu s-a gasit cuvantul {}", cuvant);
    }

    Ok(())
}

fn fn_help() {
    println!();
    println!("--help--");
    println!("UTILIZARE: grep [OPTIUNE].. PATTERNS PATH/CATRE/FOLDER");
    println!(
        "Cauta recursiv un pattern intr-un director sau in fiecare subdirector din path-ul dat"
    );
    println!("Exemplu: grep cat cats");
    println!();
    println!("Optiuni disponibile:");
    println!("  -m --max-number-of-lines numarul maxim de linii care se vor afisa");
    println!(
        "  -i --ignore-case ignora diferenta dintre majuscula si minuscula din string si text"
    );
    println!("  -c --only-count afiseaza doar numarul de linii din fisier");
    println!("  -r --regex-searching cautarea unei expresii regulate");
    println!();
    println!("Utilizati comanda exit pentru a iesi din program");
}

fn fn_grep(comenzi_linie: Vec<String>) {
    if comenzi_linie.len() < 2 {
        println!("Numar de argumente invalid.\nIncearca comanda grep --help");
        return;
    }

    if comenzi_linie[1] == "--help" {
        fn_help();
    }
    //if comenzi_linie.contains(&String::from("help")) {
    //println!("Incearca grep --help");
    //return;
    //}
    // else {
    else {
        if comenzi_linie.len() < 3 {
            println!("Numar de argumente invalid.\nIncearca comanda grep --help");
            return;
        }

        let mut flag_m = 0;
        let mut flag_i = 0;
        let mut flag_c = 0;
        let mut flag_r = 0;

        let mut index = 0;

        while index < comenzi_linie.len() - 2 {
            if comenzi_linie[index] == "-m" {
                flag_m = comenzi_linie[index + 1].parse::<i32>().unwrap();
            } else if comenzi_linie[index] == "-i" {
                flag_i = 1;
            } else if comenzi_linie[index] == "-c" {
                flag_c = 1;
            } else if comenzi_linie[index] == "-r" {
                flag_r = index.try_into().unwrap();
            }

            index = index + 1;
        }

        println!(
            "Flaguri active\nflag_m={},flag_i={},flag_c={},flag_r={}",
            flag_m, flag_i, flag_c, flag_r
        );

        let mut comenzi: Vec<String> = Vec::new();
        let mut index = 1;

        while index < comenzi_linie.len() {
            comenzi.push(comenzi_linie[index].clone());
            index += 1;
        }

        let rez = recursivdir(comenzi_linie, flag_m, flag_i, flag_c, flag_r);
        if rez.is_err() {
            println!("{}", rez.unwrap_err());
        }
    }
}

fn comanda_terminal(comenzi_linie: Vec<String>) {
    if comenzi_linie.is_empty() {
        println!("Nu s-au introdus comenzi de la tastatura");
        return;
    }

    if comenzi_linie[0] == "exit" {
        return;
    }

    if comenzi_linie[0] != "grep" {
        println!("Comanda introdusa gresit");
        println!("Incearca grep --help");
    } else if comenzi_linie[0] == "grep" {
        fn_grep(comenzi_linie);
    }
}

fn main() {
    let mut input: String = String::new();

    println!("Hei!\nDaca aveti nevoie de ajutor introduceti comanda grep --help");
    println!("Daca doriti sa iesiti din program introduceti comanda exit");
    println!("Have fun!");

    while input.trim() != "exit" {
        println!();
        print!(">");

        io::stdout().flush().unwrap();

        let mut comenzi_linie: Vec<String> = Vec::new();
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if !input.is_empty() {
            let comenzi_linie_str: Vec<&str> = input.split_whitespace().collect();

            for c in comenzi_linie_str {
                comenzi_linie.push(String::from(c));
            }
        }
        comanda_terminal(comenzi_linie);
    }
}
