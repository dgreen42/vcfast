use csv::Writer;
use rusqlite::{Connection, Result};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;
use std::time::Instant;

fn main() {
    env::set_var("RUST_BACKTRACE", "0");

    let start = Instant::now();
    let path = env::args()
        .nth(1)
        .expect("Please enter a path to a vcf file");
    let arg_name = env::args()
        .nth(2)
        .expect("Please enter the name for the csv");
    let option = env::args().nth(3).expect("Please enter a option");
    let mut name = arg_name;
    let file = open_file(&path);
    write_meta_data(file, name.clone());

    if option == "-csv" {
        let file = open_file(&path);
        name.push_str(".csv");
        let mut wrt = Writer::from_path(&name).unwrap();
        for line in file.lines() {
            let line = line.unwrap();
            if !line.starts_with("##") {
                get_info(&line);
            } else {
                println!("meta data is being skipped. Use option -meta to write meta data file");
            }
        }
    }
    if option == "-sqlite" {
        let file = open_file(path);
        let mut full_name = name.clone();
        full_name.push_str(".sqlite");
        create_db(file, name);
    }
    let duration = start.elapsed();
    println!("Time to complete: {:?}", duration);
}

fn create_db(file: BufReader<File>, file_name: String) {
    if Path::new(&file_name).exists() {
        println!(
            "{} already exists, please move db so it is not overwritten",
            &file_name
        );
    } else {
        let con = Connection::open_in_memory().expect("Could not create db");
        let mut first_line = String::new();
        let mut temp_file = file.lines();
        let mut line_count = 0;
        assert!(first_line.is_empty());
        while first_line.is_empty() {
            line_count += 1;
            let some_line = temp_file.next().unwrap().unwrap();
            if some_line.starts_with("##") {
                continue;
            } else {
                first_line.push_str(&some_line);
            }
        }
        let mut q1 = String::from("CREATE TABLE ");
        q1.push_str(&file_name);
        q1.push_str(" (");
        let data = prepare_line(first_line.clone()).to_lowercase();
        q1.push_str(&data);
        q1.push_str(");");
        let mut types = String::from("TEXT, INTEGER, TEXT, TEXT, TEXT, FLOAT, TEXT, BLOB, BLOB,");
        let flc: Vec<_> = first_line.split("\t").collect();
        let first_line_len = &flc.len();
        let sub = 9 as usize;
        let add_blobs = first_line_len - sub;
        for add in 0..add_blobs {
            if &add != &add_blobs {
                types.push_str(" BLOB,");
            } else {
                types.push_str(" BLOB");
            }
        }
        println!("{:?}", types);

        con.execute(&q1, ()).expect("Could not create db");

        let mut query_entry = String::from("INSERT INTO ");
        query_entry.push_str(&file_name);
        query_entry.push_str(" VALUES (");
        let mut test_count = 0;
        for entry in temp_file {
            test_count += 1;
            println!("{:?}", test_count);
            let entry = prepare_line(entry.unwrap());
            query_entry.push_str(&entry);
            query_entry.push_str(");");
            println!("{:?}", &query_entry);
            con.execute(&query_entry, ())
                .expect("Could not write entry");
        }
    }
}

fn put_together(data: String, types: String) -> String {
    let final_string = String::new();

    final_string
}

fn prepare_line(line: String) -> String {
    let tab_sp = line.split("\t");
    let mut commas = String::new();
    let last_element = tab_sp.clone().last().unwrap();
    for element in tab_sp {
        let fixed_element = remove_hash_tag(&element.replace(",", ";"));
        commas.push_str(&fixed_element);
        if element != last_element {
            commas.push_str(",");
        }
    }
    commas
}

fn remove_hash_tag(element: &str) -> String {
    let mut new = String::new();
    let mut sp = element.chars();
    let sp_l: Vec<char> = sp.clone().collect();
    let sp_len = sp_l.len();
    if sp.next().unwrap() == '#' {
        for ch in 1..sp_len {
            new.push_str(&sp_l[ch].to_string());
        }
    } else {
        new.push_str(&element);
    }
    new
}

fn get_info(line: &String) {
    let tab_sp: Vec<_> = line.split("\t").collect();
    let chrom = tab_sp[0];
    let pos = tab_sp[1];
    let id = tab_sp[2];
    let refer = tab_sp[3];
    let alt = tab_sp[4];
    let qual = tab_sp[5];
    let fltr = tab_sp[6];
    let mut info = String::new();

    for i in 7..tab_sp.len() {
        if tab_sp[i].contains("=") {
            info.push_str(tab_sp[i]);
        }
    }
    let len_vec: Vec<_> = info.split(";").collect();
    let info_len = len_vec.len();
    let form = tab_sp[info_len];
    println!("{:?}", form);
}

fn open_file<P>(path: P) -> BufReader<File>
where
    P: AsRef<Path>,
{
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
}

fn write_meta_data(file: BufReader<File>, name: String) {
    let mut file_name = String::from(name);
    file_name.push_str(".txt");
    if Path::new(&file_name).exists() {
        println!(
            "{} already exists, please move it to create new meta data file",
            &file_name
        );
    } else {
        File::create(&file_name).expect("Could not create file");
        let mut write_file = File::options().append(true).open(&file_name).unwrap();
        for line in file.lines() {
            let line = line.unwrap();
            if line.starts_with("##") {
                writeln!(&mut write_file, "{}", line).expect("could not write line");
            } else {
                break;
            }
        }
    }
}
