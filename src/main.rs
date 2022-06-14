use libdict;

const ARGS_USAGE: &str = "<.dict.dz> <.index>";
const INTERNAL_DELIMITER: &str = "❄";

fn main() {
    let current_exe = std::env::current_exe().unwrap();
    let current_exe = current_exe.to_str().unwrap();
    let usage = format!("{} {}", current_exe, ARGS_USAGE);

    let mut args = std::env::args();
    args.next();
    let dict_file = args.next().expect(&usage);
    let index_file = args.next().expect(&usage);

    let mut definitions: Vec<String> = vec![];
    let mut dict = libdict::load_dictionary_from_file(dict_file, index_file).unwrap();
    let mut sorted_index: Vec<_> = dict.word_index.iter().collect();
    sorted_index.sort_by(|a, b| b.0.cmp(a.0));

    //let metadata = HashMap::new();

    for (indexword, (start, length)) in sorted_index {
        if indexword.starts_with("00database") {
        } else {
            let def = dict.dict_reader.fetch_definition(*start, *length).unwrap();
            definitions.push(def);
        }
    }

    let contents = definitions.iter().enumerate().map(|(i, def)| {
        let (unparsed_name_and_metadata, definition_with_newlines) = def.split_once("\n").unwrap();
        let definition_with_delimiters = definition_with_newlines.trim().replace("\n", INTERNAL_DELIMITER);
        format!("\"{}\",\"{}\",\"{}\"", i, unparsed_name_and_metadata, definition_with_delimiters)
    }).collect::<Vec<String>>().join("\n");

    println!("{}", contents);

    //println!("{}", definitions.iter().map(|d| format!("\"{}\",\"{}\"", w, d.replace("\n", "❄"))).collect::<Vec<String>>().join("\n") );
    // TODO: create yaml configs
}
