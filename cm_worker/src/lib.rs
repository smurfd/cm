use ::cm_database::*;
use std::{process, str, fs, fs::OpenOptions, fs::File, io::Write, io::prelude::*};
use walkdir::{WalkDir};
use rusqlite::{Connection};
use std::convert::TryInto;

//TODO: Add project id to data to point to what project the file belongs to
//TODO: Add more stuff to here from main as functions

pub fn print_usage() {
  println!("Thanks for using Project CodeMonkey");
  println!("Usage:");
  // Initialize a new database
  println!("cm wakeup                        // this will initialize a new database project.db");
  // Consume a folders files (. is current folder)
  println!("cm eat /path/to/folder projname  // this will consume a folders files into the database");
  // Show the database
  println!("cm spit                          // this will show the database");
  // Search for something in the database
  println!("cm find TODO                     // this will search the database for TODO");
  // Kill the database
  println!("cm sleep                         // this will kill the project.db file");
  // Update database
  println!("cm update                        // this will update the project.db");
  process::exit(0x0);
}

pub fn hide_database() {
  // If you hide it twice it will add a 2nd line to .*ignore file
  let mut hidden = false;
  let mut f = std::path::Path::new(".gitignore").exists();
  if f == true {
    println!("adding to .gitignore");
    hidden = true;
    let mut file = OpenOptions::new().write(true).append(true).open(".gitignore").unwrap();
    if let Err(e) = writeln!(file, "project.db") {
      eprintln!("Couldn't write to file: {}", e);
    }
  }
  f = std::path::Path::new(".hgignore").exists();
  if f == true {
    println!("adding to .hgignore");
    hidden = true;
    let mut file = OpenOptions::new().write(true).append(true).open(".hgignore").unwrap();
    if let Err(e) = writeln!(file, "project.db") {
      eprintln!("Couldn't write to file: {}", e);
    }
  }
  if hidden == false {
    println!("Neither a .gitignore nor a .hgignore file exists");
  }
}

pub fn eat_path(db1: &Connection, path: &String, projname: &String) -> std::io::Result<()> {
  let projs: i32 = db_get_number_of_projects(db1);
  db_insert_proj(db1, &projname.to_string(), &path.to_string());

  for entry in WalkDir::new(path).into_iter() {
    let entry = entry.unwrap();

    let metadata = fs::metadata(entry.path().display().to_string())?;
    let file_type = metadata.file_type();

    if !file_type.is_dir() {
      let mut f = File::open(entry.path().display().to_string())?;
      let mut buffer = Vec::new();
      f.read_to_end(&mut buffer)?;

      let _contents = match str::from_utf8(&buffer) {
        Ok(x) => {
          db_insert_projdata(db1, &entry.path().display().to_string(), &x.to_string(), &projs);
          x
        },
        Err(_e) => {"Something went wrong"},
      };
    }
  }
  Ok(())
}

pub fn eat_path_again(db1: &Connection, path: &String, projname: &String) -> std::io::Result<()> {
  Ok(())
}


pub fn spit(db1: &Connection) {
  let proj = db_get_number_of_projects(db1) - 1;
  let file = db_get_number_of_files(db1) - 1;

  println!("There are {} projects in the database", proj);
  println!("There are {} files in the database", file);
  println!("---------");
  for i in 1..proj+1 {
    let n = db_get_project_name(db1, &i);
    let p = db_get_number_of_files_in_project(db1, i) - 1;
    println!("In project named : {}  #{} there are {} files", n, i, p);
  }
}

// TODO : Update, go through the list of files and also eat new files

pub fn find(db1: &Connection, search: &String) {
  db_find_term(db1, search);
  let x1 = &db_read_sel(db1).unwrap_or_else(|err| {
    println!("Problem: {}", err);
    process::exit(1);
  });

  println!("///////");
  println!("Searching for: {}", search);
  let mut len: i32 = 0;
  let mut cal: i32 = 1;
  while len < x1.len().try_into().unwrap() {
    for ii in x1 {
      if ii.pid() == &cal {   
        len = len + 1;
      }
    }
    println!("Found in project: {}", cal);
    cal = cal + 1;
  }
  println!("///////");
  len = 0;
  cal = 1;
  println!("{} found in these rows:", search);
  while len < x1.len().try_into().unwrap() {
    for ii in x1 {
      if ii.pid() == &cal {
        println!("{} | {} | {}", db_get_project_name(db1, &cal), db_get_file_name(db1, ii.fid()), ii.rowdata());
        len = len + 1;
      }
    }
    cal = cal + 1;
  }
  println!("///////");

/*
  for iii in x1 {
    println!("--- {:?}", iii);
    let mut i1 = x1.into_iter();
    let i1f1= i1.next().unwrap();
    let i1f = i1f1.fid();
    let i1p = i1f1.pid();
  
    let x2 = &db_read_specific_data(db1, i1p, i1f).unwrap_or_else(|err| {
      println!("Problem: {}", err);
      process::exit(1);
    });
//    println!("XXX {} {}", x1.len(), x2.len());
    let i2 = &mut x2.into_iter();
    let i2f1= i2.next().unwrap();
    let i2n = i2f1.name();
    let n1 = db_get_project_name(db1, i1p);
    println!("XXX Found {} in Proj {} : {}", search, n1, i2n);
  }
*/
/*  println!("-------");
  for b in db_read_sel(db1).unwrap() {
    let p = b.pid();
    let i = b.fid();
    let n = db_get_project_name(db1, p);
    println!("Searching for {} in Project {}",search, n);
//    println!("select = {:#?} {} {}", b.rowdata(), b.pid(), b.fid());
    for a in db_read_specific_data(db1, p, i).unwrap() {
      println!("Found in {}", a.name());
      println!("{}", b.rowdata());
      //println!("AAA = {} {:#?} {} {}", n, a.name(), p, i);
    }
    println!("");
  }*/
  println!("--------");

}

// ---- Tests below ---- 
#[cfg(test)]
mod tests {
  // TODO: Do better tests
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  } 
}
