/// Database structures
// project
// id | projname | projdata
// --------------------------------------
// 1  | test1    | this is a test project
//
// projectdata
// id | filename | filedata | projectid
// ------------------------------------
// 1  | test1.rs | somestff | 1
// 2  | test2.rs | othrstff | 1
// 3  | src/1.rs | 1stff    | 2
//
// projectsel
// id | selection | rownr | rowdata           | fileid | projectdataid
// -------------------------------------------------------------------
// 1  | TODO      | 12    | TODO: Some stuff  | 2      | 2
// 2  | TODO      | 3     | TODO: more things | 6      | 5

use rusqlite::{params, Connection, Result};
use std::fmt::Write as FmtWrite;

#[derive(Debug)]
pub struct FileData {
  id: i32,        // unique, file id
  name: String,   // filename
  data: String,   // file contents
  pid: i32,       // project id the file belogs to
}

#[derive(Debug)]
pub struct Project {
  pr: FileData,   // project data
  pt: i32,        // project id
  name: String,   // project name
  desc: String,   // project description
}

#[derive(Debug)]
pub struct Selection {
  id: i32,        // Searched id
  find: String,   // What was searched for
  row: i32,       // row it was found on
  rowdata: String,// the data of the whole row the data was found on
  fid: i32,       // file id it belongs to
  pid: i32,       // project id it belongs to
}

impl FileData {
  pub fn new(id: i32, name: String, data: String, pid: i32) -> Self {
    Self {
      id,
      name,
      data,
      pid,
    }
  }

  pub fn id(&self) -> &i32 {
    &self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn data(&self) -> &String {
    &self.data
  }

  pub fn pid(&self) -> &i32 {
    &self.pid
  }
}

impl Project {
  pub fn pt(&self) -> &i32 {
    &self.pt
  }

  pub fn pr(&self) -> &FileData {
    &self.pr
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn desc(&self) -> &String {
    &self.desc
  }

  pub fn new(pr: FileData, pt: i32, name: String, desc: String) -> Self {
    Self {
      pr,
      pt,
      name,
      desc,
    }
  }
}

impl Selection {
  pub fn new(id: i32, find: String, row: i32, rowdata: String, fid: i32, pid: i32) -> Self {
    Self {
      id,
      find,
      row,
      rowdata,
      fid,
      pid,
    }
  }

  pub fn fid(&self) -> &i32 {
    &self.fid
  }

  pub fn pid(&self) -> &i32 {
    &self.pid
  }

  pub fn rowdata(&self) -> &String {
    &self.rowdata
  }

  pub fn find(&self) -> &String {
    &self.find
  }
}


// TODO: add so instead of like "find" its "search" and "wakeup" its "init"
pub fn db_init_init() -> Result<Connection> {
  Connection::open("project.db")
}

fn db_init(db: &Connection, s: String) {
  match db.execute(&s, params![]) {
    Ok(updated) => println!("{} Table created", updated),
    Err(err) => println!("Table already exists {}", err),
  }
}   

pub fn db_initialize(db: &Connection) {
  db_init(db, "CREATE TABLE project (id INTEGER PRIMARY KEY AUTOINCREMENT, projname TEXT NOT NULL, projdata BLOB)".to_string());
  db_init(db, "CREATE TABLE projectdata (id INTEGER PRIMARY KEY AUTOINCREMENT, filename TEXT NOT NULL, filedata BLOB, projectid INTEGER NOT NULL)".to_string());
  db_init(db, "CREATE TABLE projectsel (id INTEGER PRIMARY KEY, selection TEXT NOT NULL, rownr INTEGER, rowdata TEXT NOT NULL, fileid INTEGER, projectdataid INTEGER)".to_string());
}

pub fn db_insert(db: &Connection, p: &Project) {
  let check_proj: &i32 = &1;
  let check_data: &i32 = &2;

  if p.pt() == check_proj {
    match db.execute("INSERT INTO project (projname, projdata) VALUES (?1, ?2)", 
    params![p.pr().name(), p.pr().data()]) {
      Ok(updated) => {println!("update"); updated},
      Err(_err) => {println!("error: {:?}", _err);0},
    };
  } else if p.pt() == check_data {
    match db.execute("INSERT INTO projectdata (filename, filedata, projectid) VALUES (?1, ?2, ?3)", 
    params![p.pr().name(), p.pr().data(), p.pr().pid()]) {
      Ok(updated) => {println!("Pupdate"); updated},
      Err(_err) => {println!("error: {:?}", _err);0}, 
    };  
  }       
}         

pub fn db_insert_proj(db: &Connection, name: &String, desc: &String) {
  let projs: i32 = db_get_number_of_projects(db);
  match db.execute("INSERT INTO project (id, projname, projdata) VALUES (?1, ?2, ?3)",
  params![projs, name.to_string(), desc.to_string()]) {
    Ok(updated) => {updated},
    Err(_err) => {println!("error: {:?}", _err);0},
  };
}

pub fn db_insert_projdata(db: &Connection, filename: &String, filedata: &String, projid: &i32) {
  match db.execute("INSERT INTO projectdata (filename, filedata, projectid) VALUES (?1, ?2, ?3)",
  params![filename.to_string(), filedata.to_string(), projid]) {
    Ok(updated) => {updated},
    Err(_err) => {println!("error: {:?}", _err);0},
  };
}

pub fn db_print(db: &Connection, p: &Project) {
  for a in db_read(&db, &p) {
    println!("newproj = {:#?}", a);
  }       
}         

pub fn db_read(db: &Connection, p: &Project) -> Result<Vec<FileData>> {
  let mut data1 = db.prepare("SELECT id, projname, projdata FROM project")?;
  let mut data2 = db.prepare("SELECT id, filename, filedata, projectid FROM projectdata")?;
  let check_proj: &i32 = &1;
  let check_data: &i32 = &2;

  let it1 = data1.query_map(params![], |row| {
    Ok(FileData::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
  })?;

  let it2 = data2.query_map(params![], |row| {
    Ok(FileData::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
  })?;

  let mut r = Vec::new();
  if p.pt() == check_proj {
    for r_r in it1 {
      r.push(r_r?);
    }
  } else if p.pt() == check_data {
    for r_r in it2 {
      r.push(r_r?);
    }
  }
  Ok(r)
}

pub fn db_read_specific_data(db: &Connection, proj: &i32, file: &i32) -> Result<Vec<FileData>> {
  let mut s = String::new();
  let mut r = Vec::new();

  writeln!(&mut s, "select id, filename, filedata, projectid from projectdata where id={} and projectid={}", file, proj).unwrap();
  let mut data = db.prepare(&s)?;
  let it1 = data.query_map(params![], |row| {
    Ok(FileData::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
  })?;
  if file == proj {}
  for r_r in it1 {
    r.push(r_r?);
  }
  Ok(r)
}

pub fn db_get_number_of_files(db: &Connection) -> i32 {
  let mut data = db.prepare("SELECT Count(*) FROM projectdata").unwrap();
  #[derive(Debug)]
  struct File {
    pub id: i32,
  }
  impl File {
    pub fn id(&self) -> &i32 {
      &self.id
    }
  }
  let it = data.query_map(params![], |row| {
    Ok(File {id: row.get(0)?})
  }).unwrap();
  
  let mut r = Vec::new();
  for r_r in it {
    r.push(r_r.unwrap());
  }
  *r[0].id() + 1
}

pub fn db_get_number_of_files_in_project(db: &Connection, projid: i32) -> i32 {
  let mut sel = String::new();
  writeln!(&mut sel, "SELECT Count(*) FROM projectdata where projectid={}", projid).unwrap();

  let mut data = db.prepare(&sel).unwrap();
  #[derive(Debug)]
  struct File {
    pub id: i32,
  }
  impl File {
    pub fn id(&self) -> &i32 {
      &self.id
    }
  }
  let it = data.query_map(params![], |row| {
    Ok(File {id: row.get(0)?})
  }).unwrap();

  let mut r = Vec::new();
  for r_r in it {
    r.push(r_r.unwrap());
  }
  *r[0].id() + 1
}

pub fn db_get_number_of_projects(db: &Connection) -> i32 {
  let mut data = db.prepare("SELECT Count(*) FROM project").unwrap();
  #[derive(Debug)]
  struct Project {
    pub id: i32,
  }
  impl Project {
    pub fn id(&self) -> &i32 {
      &self.id
    }
  }
  let it = data.query_map(params![], |row| {
    Ok(Project {id: row.get(0)?})
  }).unwrap();

  let mut r = Vec::new();
  for r_r in it {
    r.push(r_r.unwrap());
  }
  *r[0].id() + 1
}

pub fn db_get_project_name(db: &Connection, projid: &i32) -> String {
  let mut sel = String::new();
  writeln!(&mut sel, "SELECT projname FROM project where id={}", projid).unwrap();

  let mut data = db.prepare(&sel).unwrap();
  #[derive(Debug)]
  struct Project {
    pub name: String,
  }
  impl Project {
    pub fn name(&self) -> &String {
      &self.name
    }
  }
  let it = data.query_map(params![], |row| {
    Ok(Project {name: row.get(0)?})
  }).unwrap();

  let mut r = Vec::new();
  for r_r in it {
    r.push(r_r.unwrap());
  }
  r[0].name().to_string()
}

pub fn db_check_if_proj_exists(db: &Connection, projd: &String) -> bool {
  let mut sel = String::new();
  writeln!(&mut sel, "SELECT EXISTS(SELECT id FROM project WHERE projdata=\"{}\")", projd).unwrap();

  let mut data = db.prepare(&sel).unwrap();
  #[derive(Debug)]
  struct Project {
    pub id: i32,
  }
  impl Project {
    pub fn id(&self) -> &i32 {
      &self.id
    }
  }
  let it = data.query_map(params![], |row| {
    Ok(Project {id: row.get(0)?})
  }).unwrap();

  let mut r = Vec::new();
  for r_r in it {
    r.push(r_r.unwrap());
  }
  if *r[0].id() == 1 {
    true
  } else {
    false
  }
}

pub fn db_get_project_id(db: &Connection, projname: &String) -> i32 {
  let mut sel = String::new();
  writeln!(&mut sel, "SELECT id FROM project where projname={}", projname).unwrap();

  let mut data = db.prepare(&sel).unwrap();
  #[derive(Debug)]
  struct Project {
    pub id: i32,
  }
  impl Project {
    pub fn id(&self) -> &i32 {
      &self.id
    }
  }
  let it = data.query_map(params![], |row| {
    Ok(Project {id: row.get(0)?})
  }).unwrap();

  let mut r = Vec::new();
  for r_r in it {
    r.push(r_r.unwrap());
  }
  *r[0].id()
}

pub fn db_get_file_name(db: &Connection, fileid: &i32) -> String {
  let mut sel = String::new();
  writeln!(&mut sel, "SELECT filename FROM projectdata where id={}", fileid).unwrap();

  let mut data = db.prepare(&sel).unwrap();
  #[derive(Debug)]
  struct Project {
    pub name: String,
  }
  impl Project {
    pub fn name(&self) -> &String {
      &self.name
    }
  }
  let it = data.query_map(params![], |row| {
    Ok(Project {name: row.get(0)?})
  }).unwrap();

  let mut r = Vec::new();
  for r_r in it {
    r.push(r_r.unwrap());
  }
  r[0].name().to_string()
}

pub fn db_find_term(db: &Connection, search: &String) {
//  let f: i32 = db_get_number_of_files(db)-1;
//  let p: i32 = db_get_number_of_projects(db)-1;
  let mut selid: i32 = 0;

  for j in 0..db_get_number_of_projects(db) {
    for i in 0..db_get_number_of_files_in_project(db, j) {
      let mut rowc: i32 = 1;

      for a in &db_read_specific_data(db, &j, &i).unwrap() {
        let finds = a.data().find(search);
        match finds {
          Some(_x) => {
            let lines = a.data().lines();
            for row in lines {
              let trimmed: String = row.trim().to_string();
              if trimmed.contains(search) {
                match db.execute("INSERT INTO projectsel (id, selection, rownr, rowdata, fileid, projectdataid) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", 
                params![selid, search, rowc, trimmed, i, j]) {
                  Ok(updated) => {println!("{} {}", i, j);updated},
                  Err(_err) => 0,
                };
                selid = selid + 1;
              }
              rowc = rowc + 1;
            }
          },
          None => (),
        }
      }
    }
  }
}

pub fn db_read_sel(db: &Connection) -> Result<Vec<Selection>> {
  let mut data1 = db.prepare("SELECT id, selection, rownr, rowdata, fileid, projectdataid FROM projectsel")?;

  let it1 = data1.query_map(params![], |row| {
    Ok(Selection::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?))
  })?;

  let mut r = Vec::new();
    for r_r in it1 {
      r.push(r_r?);
  }
  Ok(r)
}

pub fn db_read_sel_from_project(db: &Connection, proj: i32) -> Result<Vec<Selection>> {
  let mut sel = String::new();
  writeln!(&mut sel, "SELECT id, selection, rownr, rowdata, fileid, projectdataid FROM projectsel where projectdataid={}", proj).unwrap();

  let mut data = db.prepare(&sel).unwrap();

//  let mut data1 = db.prepare("SELECT id, selection, rownr, rowdata, fileid, projectdataid FROM projectsel")?;
      
  let it1 = data.query_map(params![], |row| {
    Ok(Selection::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?))
  })?;
  
  let mut r = Vec::new();
    for r_r in it1 {
      r.push(r_r?);
  } 
  Ok(r)
}


// ----- Tests below ----
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
