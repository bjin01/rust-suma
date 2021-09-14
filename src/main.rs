extern crate xmlrpc;
extern crate clap;
use clap::{Arg, App};
use xmlrpc::{Request, Value};
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SumaInfo {
    hostname: String,
    user_name: String,
    password: String,
    advisory_type: String, 
    output_fields: Vec<String>,
    servers: Vec<String>,
}

impl SumaInfo {
    fn new(file: &String) -> SumaInfo {
        let mut f = File::open(file).expect("Could not read file");
        let mut buffer = String::new();

        f.read_to_string(&mut buffer).expect("failed to read file into buffer as string.");
        let deserialized_map: SumaInfo = match serde_yaml::from_str(&buffer) {
            Ok(i) => i,
            Err(_) => panic!("getting yaml failed.")
        };
        return deserialized_map
    }
}

fn login(s: &SumaInfo) -> String {
    let suma_request = Request::new("auth.login").arg(String::from(&s.user_name)).arg(String::from(&s.password)); 
    let request_result = suma_request.call_url(String::from(&s.hostname));
    match &request_result {
        Err(e) => {
            println!("Could not login to SUMA server. {}", e);
            std::process::exit(1);
        },
        Ok(i) => match i.as_str() {
            
            Some(q) => return q.to_string(),
            None => std::process::exit(1),
        }
    }
}

fn logout(k: &String, s: &SumaInfo) -> i32 {
    let suma_logout_request = Request::new("auth.logout").arg(k.to_string());
    let suma_logout_result = suma_logout_request.call_url(String::from(&s.hostname));
    match &suma_logout_result {
        Err(e) => {
            println!("Could not logout. {}", e);
            std::process::exit(1);
        },
        Ok(i) => match i.as_i32() {
            Some(q) => return q,
            None => std::process::exit(1),
        }
    }
}


fn printvalue(x: &Value, s: &Vec<String>, id_list: &mut Vec<i32>) {
    match x {
        Value::Int(i) => {
            println!("{}", i);
        }
        Value::Bool(b) => {
            println!("{}", b);
        }
        Value::String(s) => {
            println!("{}", s);
        }
        Value::DateTime(date_time) => {
            println!("{}", date_time);
        }
        Value::Struct(ref map) => {
            for (ref name, ref value) in map {
                if s.len() > 0 {
                    s.into_iter().for_each(|i| {
                        if i.eq(&name.to_string()) {
                            print!("{}: ", name);
                            printvalue(*value, s, id_list);
                        }
                    });
                } else {
                    printvalue(*value, s, id_list);
                }

                if name.to_string().eq(&"id".to_string()) {
                    {
                        id_list.push(value.as_i32().unwrap());   
                    };
                }
            }
        }
        Value::Array(ref array) => {
            
            for value in array {
                printvalue(value, s, id_list);
                println!("")
            }
        }
        Value::Nil => {
            println!("nil");
        }
        
        Value::Int64(_i) => println!("{:?}", _i),
        Value::Double(_d) =>  println!("{:?}", _d),
        Value::Base64(_b) => println!("{:?}", _b),
    }
}



fn get_systemid(key: &String, s: &String, z: &SumaInfo) -> Result<i32, &'static str> {

    let get_system_id = Request::new("system.getId").arg(String::from(key)).arg(s.to_string());
    let get_system_id_result = get_system_id.call_url(String::from(&z.hostname));

    match get_system_id_result.unwrap().as_array() {
        Some(i) => {
            if i.len() > 0 {
                match i[0].as_struct() {
                    Some(h) => match h[&"id".to_string()].as_i32() {
                        Some(j) => return Ok(j),
                        None => Err("invalid server id, no integer found."),
                    }
                    None => Err("invalid server id, no struct found."),
                }
            } else {
                Err("invalid server id in array.")
            }
        },
        None => Err("invalid server id, no array."),
    }
}

fn main() -> Result<(), serde_yaml::Error> {

    let matches = App::new("SUSE Manager - just patch")
        .version("0.1.0")
        .author("Bo Jin <bo.jin@suse.com>")
        .about("patch systems by calling suma xmlrpc api")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .takes_value(true)
                 .help("yaml config file with login"))
        .get_matches();
    let yaml_file = matches.value_of("config").unwrap_or("test.yaml");

    let mut suma_info: SumaInfo = SumaInfo::new(&String::from(yaml_file));
    suma_info.hostname.insert_str(0, "http://");
    suma_info.hostname.push_str("/rpc/api");
    println!("suma host api url: {:?}", &suma_info.hostname);

    
    let mut server_id_list: Vec<i32> = Vec::new();
    let key = login(&suma_info);

    for s in &suma_info.servers {        
        let systems_id = get_systemid(&key, &s, &suma_info);
        match systems_id {
            Err(e) => println!("No server id found for {} - {}", &s, e),
            Ok(i) => {
                server_id_list.push(i);
                //println!("list of system id: {:?} ", &server_id_list);
            }
        }        
        
    }

    if server_id_list.len() > 0 {
        for i in &server_id_list {     
            let mut patch_id_list: Vec<i32> = Vec::new();
            
            if suma_info.advisory_type.trim().len() <= 1 {
                let erratalist = Request::new("system.getRelevantErrata").arg(String::from(&key)).arg(*i);
                let erratalist_result = erratalist.call_url(String::from(&suma_info.hostname));
                
                match erratalist_result {
                    Ok(i) => printvalue(&i, &suma_info.output_fields, &mut patch_id_list),
                    Err(e) => println!("no errata found: {:?}", e),
                }
            } else {
                let erratalist = Request::new("system.getRelevantErrataByType").arg(String::from(&key)).arg(*i).arg(String::from(&suma_info.advisory_type));
                let erratalist_result = erratalist.call_url(String::from(&suma_info.hostname));
                match erratalist_result {
                    Ok(i) => printvalue(&i, &suma_info.output_fields, &mut patch_id_list),
                    Err(e) => println!("no errata found: {:?}", e),
                }
            }

            let mut value_id_list: Vec<Value> = Vec::new();
            for s in &patch_id_list {
                value_id_list.push(Value::Int(*s));
            }
            let patch_errata = Request::new("system.scheduleApplyErrata").arg(String::from(&key)).arg(*i).arg(Value::Array(value_id_list));
            let patch_errata_result = patch_errata.call_url(String::from(&suma_info.hostname));
            match patch_errata_result {
                Ok(s) => {
                    print!("Patch Job ID ");
                    printvalue(&s, &suma_info.output_fields, &mut patch_id_list);
                },
                Err(e) => println!("no patch job created because the errata list is empty. Maybe your system is up to date.: {:?}", e),
            }
        }
    }

    println!("Logout successful - {}", logout(&key, &suma_info));
    Ok(())
}