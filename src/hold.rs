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
        let csv_name = name.push_str(".csv");
        let mut wrt = Writer::from_path(csv_name).unwrap();
        for line in file.lines() {
            let line = line.unwrap();
            if !line.starts_with("##") {
                let line_split: Vec<_> = line.split("\t").collect();
                let vec = Vec::from(line_split);
                wrt.write_record(vec);
            } else {
                println!("meta data is being skipped. Use option -meta to write meta data file");
            }
        }
    }

    if option == "-vcf" {
        let file = open_file(&path);
        write_vcf_subset(file, name.clone());
    }

    let duration = start.elapsed();
    println!("Time to complete: {:?}", duration);
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

fn write_vcf_subset(file: BufReader<File>, name: String) {
    let mut file_name = String::from(name);
    file_name.push_str(".vcf");
    if Path::new(&file_name).exists() {
        println!(
            "{} already exists, please move it to create new meta data file",
            &file_name
        );
    } else {
        File::create(&file_name).expect("Could not create file");
        let mut write_file = File::options().append(true).open(&file_name).unwrap();
        let mut count = 0;
        while count != 10 {
            for line in file.lines() {
                let line = line.unwrap();
                if !line.starts_with("##") {
                    writeln!(&mut write_file, "{}", line).expect("could not write line");
                    count += 1;
                }
            }
        }
    }
}
