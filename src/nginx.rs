// use std::io::Read;
// use std::fs::File;

// use nginx_config::parse_main;

// pub fn read_config() {
//     let mut buf = String::with_capacity(1024);
//     let path = format!("conf/default.conf");
//     let mut f = File::open(&path).unwrap();
//     f.read_to_string(&mut buf).unwrap();
//     // println!("{}", buf);
//     let ast = parse_main(&buf).unwrap();
//     println!("{}", ast);
//     // assert_eq!(ast.to_string(), buf);
// }
