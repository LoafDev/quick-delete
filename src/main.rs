use std::{fs::{read_dir, remove_file}, io::{self, Error}, path::{Path, PathBuf}};

fn input() -> Result<String, Error> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn remove_files<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fn recur_files(path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut buf = vec![];

        buf.extend(read_dir(path)?
            .filter_map(|e| match e {
                Ok(v) => Some(v),
                Err(er) => { println!("Error reading path: {er}"); None }
            })
            .filter_map(|e| match e.metadata() {
                Ok(v) => Some((e.path(), v)),
                Err(er) => { println!("Error loading metadata for {:?}: {er}",e.path()); None }
            })
            .flat_map(|(path,meta)| {
                if meta.is_dir() { match recur_files(&path) {
                    Ok(v) => v,
                    Err(er) => { println!("Error getting {:?} content: {er}", path); vec![] }
                }}
                else if meta.is_file() && !matches!(path.extension().and_then(|x| x.to_str()), Some("cpp" | "c" | "rs" | "py")) { vec![path] }
                else { vec![] }
            })
        );
        Ok(buf)
    }
    let path = recur_files(path.as_ref())?;
    path.into_iter().for_each(|p|
        if let Err(e) = remove_file(&p) {
            println!("Error deleting, {:?}:\n{e}", p);
        }
    );
    Ok(())
}

fn yes_or_no() -> io::Result<bool> {
    loop {
        let yn = input()?;
        match yn.as_str() {
            "y" | "yes" => { return Ok(true); },
            "n" | "no" => { return Ok(false); },
            _ => println!("Please input y/n or yes/no"),
        }
    }
}

fn main() -> io::Result<()> {
    println!("Please input directory!");
    let s = input()?;
    println!("Are you sure? Please double check! (y/n or yes/no)");

    if let true = yes_or_no()? {
        println!("Deleting everything files inside {}", s);
        remove_files(s)?;
    } else { println!("Aborted"); }
    Ok(())
}
