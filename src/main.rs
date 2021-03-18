use cm_worker::*;
use cm_database::*;
use std::{env, fs};

fn main() -> std::io::Result<()> {
  // Get arguments and check amount of arguments
  let args: Vec<String> = env::args().collect();
  let arg_len = args.len();

  if &arg_len < &2 {
    print_usage();
  }
  let mode = &args[1];
  let mode_eat = &"eat".to_string();
  let mode_wakeup = &"wakeup".to_string();
  let mode_spit = &"spit".to_string();
  let mode_sleep = &"sleep".to_string();
  let mode_find = &"find".to_string();
  let mode_hide = &"hide".to_string();
  let mode_update = &"update".to_string();

  if (&arg_len < &2) || (mode == mode_eat && &arg_len < &4) || (mode == mode_find && &arg_len < &3) {
    print_usage();
  }

  let f = std::path::Path::new("project.db").exists();
  let db = &db_init_init().unwrap();

  if mode == mode_wakeup || f == false {
    db_initialize(db);
  } else if mode == mode_eat {
    let path = &args[2];
    let name = &args[3];
    eat_path(db, &path.to_string(), &name.to_string())?;
  } else if mode == mode_spit {
    spit(db);
  } else if mode == mode_sleep {
    fs::remove_file("project.db")?;
  } else if mode == mode_find {
    let search = &args[2];
    find(db, search);
  } else if mode == mode_hide {
    // If you hide it twice it will add a 2nd line to .*ignore file
    hide_database();
  } else if mode == mode_update {
    let path = &args[2];
    if db_check_if_proj_exists(db, path) {

    }
  }
  Ok(())
}
