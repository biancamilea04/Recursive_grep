use regex::Regex;
use std::io::Write;
use std::path::Path;
use std::{fs, io};
use walkdir::WalkDir;
//expresii regulate de cautat: \d+

fn name_file(path: &Path) {
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            println!("name file: {}", file_name_str);
        } else {
            println!("Nu s-a putut obtine numele fisierului ca string");
        }
    } else {
        println!("Nu exista nume de fisier");
    }
}
fn parcurgtxt(
    path: &Path,
    cuv: String,
    flag: &mut i32,
    flag_m: i32,
    _flag_i: i32,
    _flag_c: i32,
    _flag_r: i32,
) {
    let text = fs::read(path);
    match text {
        Ok(_) => {}
        Err(e) => {
            println!("Eroare la citire din fisier:\n{}", e);
            return;
        }
    }

    let mut file = String::from_utf8_lossy(&text.unwrap()).to_string();
    let mut cuvant = cuv.clone();

    let ansi_escape_regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    file = ansi_escape_regex.replace_all(&file, "").to_string();

    if _flag_i != 0 {
        file = file.to_lowercase();
        cuvant = cuv.to_lowercase();
    }

    if _flag_r != 0 {
        let regex = match Regex::new(&cuvant) {
            Ok(r) => r,
            Err(err) => {
                println!("Eroare la crearea regex-ului: {}", err);
                return;
            }
        };
        let mut flag_file = 0;
        for _mat in regex.find_iter(file.as_str()) {
            flag_file = 1;
        }

        if flag_file == 1 {
            println!("\nfound:\npath: {}", path.to_str().unwrap());

            name_file(path);

            for (count, line) in file.lines().enumerate() {
                let mut flag_match = 0;
                for _mat in regex.find_iter(line) {
                    flag_match = 1;
                }

                if flag_match == 1 {
                    if _flag_c == 1 {
                        println!("line number: {}", count + 1);
                    } else {
                        println!("line number: {}\nline: {}", count + 1, line);
                    }
                    *flag += 1;
                    if flag_m != 0 && *flag >= flag_m {
                        return;
                    }
                }
            }
        }
    } else if file.contains(cuvant.as_str()) {
        println!("\nfound:\npath: {}", path.to_str().unwrap());
        name_file(path);

        for (count, line) in file.lines().enumerate() {
            if line.contains(cuvant.as_str()) {
                if _flag_c == 1 {
                    println!("line number: {}", count + 1);
                } else {
                    println!("line number: {}\nline: {}", count + 1, line);
                }
                *flag += 1;
                if flag_m != 0 && *flag >= flag_m {
                    return;
                }
            }
        }
    }
}
fn recursivdir(comenzi: Vec<String>, _flag_m: i32, _flag_i: i32, _flag_c: i32, _flag_r: i32) {
    let dir = comenzi[comenzi.len() - 1].clone();
    let cuvant = comenzi[comenzi.len() - 2].clone();
    let mut flag = 0;

    for entry in WalkDir::new(dir) {
        if entry.is_err() {
            println!("Eroare la deschiderea directorului:{}", entry.unwrap_err());
            return;
        }
        let entry = entry.unwrap();

        let path = entry.path();
        if path.is_file() {
            parcurgtxt(
                path,
                cuvant.to_string(),
                &mut flag,
                _flag_m,
                _flag_i,
                _flag_c,
                _flag_r,
            );
            if flag >= _flag_m && _flag_m != 0 {
                return;
            }
        }
    }

    if flag == 0 {
        println!("Nu s-a gasit cuvantul {}", cuvant);
    }
}

fn fn_help() {
    println!();
    println!("--help--");
    println!(
        "UTILIZARE: grep [OPTIUNE].. \"PATTERNS\" \"CALEA/CATRE/DIRECTOR\"\n\
    Cauta recursiv un pattern in fisierele dintr-un director sau in fiecare subdirector din path-ul dat\n\
    Exemplu: grep \"cat\" \"cats\""
    );
    println!();
    println!(
        "Optiuni disponibile:\n\
    -m --max_number_of_lines numarul maxim de linii care se vor afisa\n\
    =>Argumentul urmator trebuie sa fie un numar natural\n\
    -i --ignore_case ignora diferenta dintre majuscule si minuscule din pattern si fisier\n\
    -c --only_count afiseaza doar numarul liniei gasite\n\
    -r --regex_searching cautare dupa o expresie regulata\n\
    =>Pentru aceasta optiune in locul PATTERNULUI se poate introduce expresia regulata"
    );
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
    } else {
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
            if comenzi_linie[index].starts_with('-') {
                if comenzi_linie[index] == "--help" {
                    continue;
                } else if comenzi_linie[index] == "-m"
                    || comenzi_linie[index] == "--max_number_of_lines"
                {
                    if (comenzi_linie.iter().filter(|&s| s == "-m").count()
                        + comenzi_linie
                            .iter()
                            .filter(|&s| s == "--max_number_of_lines")
                            .count())
                        < 2
                    {
                        if comenzi_linie[index + 1].chars().all(|c| c.is_ascii_digit()) {
                            flag_m = comenzi_linie[index + 1].parse::<i32>().unwrap();
                        } else {
                            println!("Argument invalid pentru max_number_of_lines.\n-m sau --max_number_of_lines trebuie sa fie precedat de un numar natural .");
                            return;
                        }
                    } else {
                        println!("Argumentul \"-m\" sau \"--max_number_of_lines\" se regaseste de prea multe ori.");
                        return;
                    }
                } else if comenzi_linie[index] == "-i" || comenzi_linie[index] == "--ignore_case" {
                    if (comenzi_linie.iter().filter(|&s| s == "-i").count()
                        + comenzi_linie
                            .iter()
                            .filter(|&s| s == "--ignore_case")
                            .count())
                        < 2
                    {
                        flag_i = 1;
                    } else {
                        println!("Argumentul \"-i\" sau \"--ignore_case\" se regaseste de prea multe ori.");
                        return;
                    }
                } else if comenzi_linie[index] == "-c" || comenzi_linie[index] == "--only_count" {
                    if (comenzi_linie.iter().filter(|&s| s == "-c").count()
                        + comenzi_linie
                            .iter()
                            .filter(|&s| s == "--only_count")
                            .count())
                        < 2
                    {
                        flag_c = 1;
                    } else {
                        println!("Argumentul \"-c\" sau \"--only_count\" se regaseste de prea multe ori.");
                        return;
                    }
                } else if comenzi_linie[index] == "-r"
                    || comenzi_linie[index] == "--regex_searching"
                {
                    if (comenzi_linie.iter().filter(|&s| s == "-r").count()
                        + comenzi_linie
                            .iter()
                            .filter(|&s| s == "--regex_searching")
                            .count())
                        < 2
                    {
                        match Regex::new(&comenzi_linie[comenzi_linie.len() - 2]) {
                            Ok(_) => {
                                flag_r = 1;
                            }
                            Err(_) => {
                                println!("Argument invalid regular expression.\n");
                                return;
                            }
                        }
                    } else {
                        println!("Argumentul \"-r\" sau \"--regex_searching\" se regaseste de prea multe ori.");
                        return;
                    }
                } else {
                    println!(
                        "Argumente invalide.\n{} nu exista!\nIncearca comanda grep --help",
                        comenzi_linie[index]
                    );
                    return;
                }
            }
            index += 1;
        }

        let mut comenzi: Vec<String> = Vec::new();
        index = 1;

        while index < comenzi_linie.len() {
            comenzi.push(comenzi_linie[index].clone());
            index += 1;
        }

        recursivdir(comenzi_linie, flag_m, flag_i, flag_c, flag_r);
    }
}

fn args_slice(input: &str, args: &mut Vec<String>) {
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut escaped = false;
    let mut found_quoted_arg = false;

    for ch in input.trim().chars() {
        match (ch, in_quotes, escaped) {
            ('"', false, false) => {
                in_quotes = true;
                found_quoted_arg = true;
                current_arg.push('"');
            }
            ('"', true, false) => {
                in_quotes = false;
                current_arg.push('"');
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            ('\\', true, false) => {
                escaped = true;
                current_arg.push('/');
            }
            (c, true, true) => {
                current_arg.push(c);
                escaped = false;
            }
            (c, true, false) => {
                current_arg.push(c);
            }
            (c, false, _) if c.is_whitespace() => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            (c, false, _) => {
                if found_quoted_arg {
                    return;
                }
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        if found_quoted_arg {
            println!("Comanda invalida");
            return;
        }
        args.push(current_arg);
    }
}

fn test_input(input: &str, flag: &mut i32) {
    let args: Vec<&str> = input.split_whitespace().collect();
    if args[0] == "exit" && args.len() != 1 {
        println!("Comanda exit executata gresit!\nIncearca exit");
        *flag = 0;
    } else if args[0] == "exit" && args.len() == 1 && *flag == 1 {
        return;
    } else if args[0] != "grep" && *flag == 1 {
        println!("Se accepta doar comanda \"exit\" sau sa inceapa cu cuvantul \"grep\".");
        *flag = 0;
    } else if args.len() > 2 && args[0] == "grep" && args[1] == "--help" && *flag == 1 {
        println!("Comanda \"grep --help\" scrisa gresit.");
        *flag = 0;
    } else if args.len() == 2 && args[0] == "grep" && args[1] == "--help" && *flag == 1 {
        fn_help();
        return;
    } else if args[0] == "grep" && args.len() < 3 && *flag == 1 {
        println!("Comanda grep nu are suficiente argumente.\nFormat: grep [OPTIUNE].. \"PATTERNS\" \"CALEA/CATRE/DIRECTOR\"");
        *flag = 0;
    } else if *flag == 1 {
        let mut argv: Vec<String> = Vec::new();
        args_slice(input, &mut argv);

        let mut index = 1;
        while index < args.len() {
            if !args[index].starts_with("-")
                && (!args[index].starts_with("\"") && !args[index].ends_with("\""))
                && !(args[index].chars().all(|c| c.is_ascii_digit())
                    && (args[index - 1] == "-m" || args[index - 1] == "--max_number_of_lines")  )
            {
                println!(
                    "Argumentul introdus este invalid.\nArgumentul:{}",
                    args[index]
                );
                *flag = 0;
                break;
            }
            index += 1;
        }
    }
}

fn input_slice(input: &str, args: &mut Vec<String>, flag: &mut i32) {
    test_input(input, &mut *flag);

    if *flag == 1 {
        let mut current_arg = String::new();
        let mut in_quotes = false;
        let mut escaped = false;
        let mut found_quoted_arg = false;
        for ch in input.trim().chars() {
            match (ch, in_quotes, escaped) {
                ('"', false, false) => {
                    in_quotes = true;
                    found_quoted_arg = true;
                }
                ('"', true, false) => {
                    in_quotes = false;
                    if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }
                }
                ('\\', true, false) => {
                    escaped = true;
                    current_arg.push('\\');
                }
                (c, true, true) => {
                    current_arg.push(c);
                    escaped = false;
                }
                (c, true, false) => {
                    current_arg.push(c);
                }
                (c, false, _) if c.is_whitespace() => {
                    if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }
                }
                (c, false, _) => {
                    if found_quoted_arg {
                        println!("Comanda invalida");
                        return;
                    }
                    current_arg.push(c);
                }
            }
        }

        if !current_arg.is_empty() {
            if found_quoted_arg {
                println!("Comanda introdusa gresit");
                return;
            }
            args.push(current_arg);
        }
    }
}

fn main() {
    let mut input: String = String::new();

    println!(
        "Hei!\n\
    Daca aveti nevoie de ajutor introduceti comanda grep --help\n\
    Daca doriti sa iesiti din program introduceti comanda exit"
    );
    println!("Have fun!");

    while input.trim() != "exit" {
        println!();
        print!(">");

        io::stdout().flush().unwrap();

        input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if !input.trim().is_empty() {
            if input.matches('"').count() == 4 && input.trim().ends_with('"') {
                let mut comenzi_linie = Vec::new();
                let mut flag = 1;

                input_slice(&input, &mut comenzi_linie, &mut flag);

                if flag == 1 && comenzi_linie[0] != "exit" {
                    fn_grep(comenzi_linie);
                }
            } else {
                let mut comenzi_linie = Vec::new();
                let mut flag = 1;

                input_slice(&input, &mut comenzi_linie, &mut flag);

                if flag == 1 {
                    if comenzi_linie.is_empty() {
                        println!("Nu s-au introdus comenzi de la tastatura");
                        continue;
                    } else if flag == 0 {
                        continue;
                    } else if comenzi_linie[0] == "exit" {
                        return;
                    } else if comenzi_linie.len() == 2
                        && comenzi_linie[0] == "grep"
                        && comenzi_linie[1] == "--help"
                    {
                        continue;
                    } else {
                        println!("Input invalid.\nIncearca grep --help");
                    }
                } else {
                    println!("Input invalid.\nIncearca grep --help");
                }
            }
        } else {
            println!("Nu s-a introdus niciun input.\nIncearca grep --help");
        }
    }
}
