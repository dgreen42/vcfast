use csv::Writer;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let path = env::args()
        .nth(1)
        .expect("Please enter a path to a vcf file");
    let arg_name = env::args()
        .nth(2)
        .expect("Please enter the name for the csv");
    let option = env::args().nth(3).expect("Please enter a option");
    let mut name = arg_name;
    if option == "-meta" {
        let file = open_file(&path);
        write_meta_data(file, name.clone());
    }
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
    let duration = start.elapsed();
    println!("Time to complete: {:?}", duration);
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
