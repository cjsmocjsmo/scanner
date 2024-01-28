use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <apath>");
        return;
    }

    let apath = Path::new(&args[1]);
    let mut ext_vec = Vec::new();
    let mut count = 0;

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            count += 1;
            let fname = e.path().to_string_lossy().to_string();
            let ext_split = fname.split(".").collect::<Vec<&str>>();
            let ext = ext_split.last().unwrap().to_string();
            if ext_vec.contains(&ext) {
                continue;
            } else {
                ext_vec.push(ext);
            }
        }
    }
    println!("Start count: {}\nFiles removed: {:#?}", count, &ext_vec);
}
