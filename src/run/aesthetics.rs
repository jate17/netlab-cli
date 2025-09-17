use std::collections::HashMap;

const OPTIONS_FLSM: [(&str, u32); 9] = [
    ("Host Necessari", 0),
    ("Subnet Disponibili", 1),
    ("Host x Subnet", 2),
    ("Ottieni Prefix-ID", 3),
    ("Net-ID Per Ogni SubNet", 4),
    ("Broadcast Da Net-ID", 5),
    ("Gateway Da Net-ID", 6),
    ("Host Sprecati", 7),
    ("Cerca host", 8),

];




fn spwan_options(options_raw: &[(&str, u32)]) -> String {

    let options: HashMap<String, u32> = options_raw
        .iter()
        .map(|(k, v)| (k.to_string(), *v))
        .collect();


    let mut items: Vec<(&String, &u32)> = options.iter().collect();
    items.sort_by_key(|&(_, id)| id);


    let mut work = String::new();
    for (nome, id) in items {
        work.push_str(&format!("\t\t[{}] {}\n", id, nome));
    }

    work
}




pub fn flsm_banner(){
    let eye = r#"
                    ⠁⠀⠀⠀⠀⠀⠈⢀⢠⠴⠒⢿⣉⣦⣱⡇⣧⢼⣯⣿⠲⣥⣀⡀⠁⠠⠀⠀⠀⠀
                    ⠀⠀⠀⠀⠀⠀⡀⢎⠰⣈⣵⣾⣿⣿⣿⣿⣿⣿⣿⣷⣿⣷⣯⡴⢆⡤⠀⡄⠀⠀
                    ⠀⠀⠀⠀⡀⢆⠱⣨⣶⠿⣿⠟⠋⣿⣿⣿⣿⣿⣿⣯⠛⠿⣿⣿⣷⣒⣩⠄⠀⠀
                    ⠀⠀⠐⠊⡴⢬⡾⠛⠁⠀⣿⣷⣼⣿⣿⣿⣿⣿⣿⣿⠀⠀⢈⠛⣻⡶⠶⠗⠂⠀
                    ⠀⠀⠐⢀⣴⠛⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⡠⢀⣴⡿⠉⠡⠀⠀⠀
                    ⠀⠈⠀⠺⠕⠒⠂⠀⡀⠀⠀⠈⠙⠛⠻⠛⡛⠋⠀⠀⣠⣶⣿⠿⡣⠐⠀⠀⠀⠀
                    ⠃⢀⠀⠀⠀⠀⠀⠀⠐⠀⠠⢠⡴⢦⣤⣄⣶⡶⣾⢿⠛⠟⠂⠓⠀⠀⠀⠀⠀⠀
                    ⠀⠀⠈⠐⠀⠄⢀⠀⠀⠀⠒⠁⠊⠉⡇⢛⠾⠑⠁⠉⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀
                    ⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                            By Jate17
                "Sic gorgiamus allos subjectatos nunc"

"#;

let footer = r#"

                Inserisci il numero o scrivi exit:
"#;

    let options_good = spwan_options(&OPTIONS_FLSM);

    println!("{}{}{}", eye, options_good, footer);
}





pub fn banner() {
    let eye = r#"
                    ⠁⠀⠀⠀⠀⠀⠈⢀⢠⠴⠒⢿⣉⣦⣱⡇⣧⢼⣯⣿⠲⣥⣀⡀⠁⠠⠀⠀⠀⠀
                    ⠀⠀⠀⠀⠀⠀⡀⢎⠰⣈⣵⣾⣿⣿⣿⣿⣿⣿⣿⣷⣿⣷⣯⡴⢆⡤⠀⡄⠀⠀
                    ⠀⠀⠀⠀⡀⢆⠱⣨⣶⠿⣿⠟⠋⣿⣿⣿⣿⣿⣿⣯⠛⠿⣿⣿⣷⣒⣩⠄⠀⠀
                    ⠀⠀⠐⠊⡴⢬⡾⠛⠁⠀⣿⣷⣼⣿⣿⣿⣿⣿⣿⣿⠀⠀⢈⠛⣻⡶⠶⠗⠂⠀
                    ⠀⠀⠐⢀⣴⠛⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⡠⢀⣴⡿⠉⠡⠀⠀⠀
                    ⠀⠈⠀⠺⠕⠒⠂⠀⡀⠀⠀⠈⠙⠛⠻⠛⡛⠋⠀⠀⣠⣶⣿⠿⡣⠐⠀⠀⠀⠀
                    ⠃⢀⠀⠀⠀⠀⠀⠀⠐⠀⠠⢠⡴⢦⣤⣄⣶⡶⣾⢿⠛⠟⠂⠓⠀⠀⠀⠀⠀⠀
                    ⠀⠀⠈⠐⠀⠄⢀⠀⠀⠀⠒⠁⠊⠉⡇⢛⠾⠑⠁⠉⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀
                    ⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                            By Jate17
                "Sic gorgiamus allos subjectatos nunc"

                [0] FLSM


                Inserisci il numero o scrivi exit:
"#;

    println!("{}", eye);
}

