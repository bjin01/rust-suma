extern crate xmlrpc;
extern crate chrono;
//use std::{any::type_name};
use xmlrpc::{Request, Value};
use clap::{Arg, App};

/* #[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
} */

static SUMAHOST: &'static str = "http://suma1.bo2go.home/rpc/api";
static USER: &str = "bjin01";
static PWD: &str = "suselinux";

fn login() -> String {
    let suma_request = Request::new("auth.login").arg(USER).arg(PWD); 
    let request_result = suma_request.call_url(SUMAHOST);
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

fn logout(k: &String) -> i32 {
    let suma_logout_request = Request::new("auth.logout").arg(k.to_string());
    let suma_logout_result = suma_logout_request.call_url(SUMAHOST);
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

/* fn get_id(x: Value) -> i32 {
    match x {
        Value::Int(i) => {
            return i
        }
        Value::Int64(_) => todo!(),
        Value::Bool(_) => todo!(),
        Value::String(_) => todo!(),
        Value::Double(_) => todo!(),
        Value::DateTime(_) => todo!(),
        Value::Base64(_) => todo!(),
        Value::Struct(_) => todo!(),
        Value::Array(_) => todo!(),
        Value::Nil => todo!(),
        
    }
} */



fn printvalue(x: &Value, s: &Vec<String>, id_list: &mut Vec<i32>) {
    //let mut id_list: Vec<i32> = Vec::new();
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



fn get_systemid(key: &String, s: &String) -> Result<i32, &'static str> {

    let get_system_id = Request::new("system.getId").arg(String::from(key)).arg(s.to_string());
    let get_system_id_result = get_system_id.call_url(SUMAHOST);

    match get_system_id_result.unwrap().as_array() {
        Some(i) => {
            if i.len() > 0 {
                match i[0].as_struct() {
                    Some(h) => match h[&"id".to_string()].as_i32() {
                        Some(j) => return Ok(j),
                        None => Err("invalid server id"),
                    }
                    None => Err("invalid server id"),
                }
            } else {
                Err("invalid server id")
            }
        },
        None => Err("invalid server id"),
    }
}

fn main() {

    let matches = App::new("SUMA Get Something")
        .version("1.0")
        .author("Bo Jin <bo.jin@suse.com>")
        .about("make suma api calls")
        .arg(Arg::new("hostname")
            .short('h')
            .long("hostname")
            .about("SUMA host name or ip")
            .takes_value(true))
        .arg(Arg::new("user")
            .short('u')
            .long("username")
            .about("SUMA api user name")
            .takes_value(true))
        .arg(Arg::new("passwd")
            .short('p')
            .long("password")
            .about("SUMA user password")
            .takes_value(true))
        .get_matches();

    if let Some(i) = matches.value_of("hostname") {
        println!("Value for hostname: {}", i.to_string());
    }

    if let Some(i) = matches.value_of("user") {
        println!("Value for user: {}", i);
    }

    if let Some(i) = matches.value_of("passwd") {
        println!("Value for password: {}", i);
    }

    let mut id_list: Vec<i32> = Vec::new();
    let mut server_id_list: Vec<i32> = Vec::new();
    /* let search: Vec<String> = vec!["label".to_string(), "arch_name".to_string(), "packages".to_string()];
    let key = login().unwrap(); 

    let channellist = Request::new("channel.listAllChannels").arg(String::from(&key));
    let channellist_result = channellist.call_url(SUMAHOST);
    printvalue(&channellist_result.unwrap(), &search); */
    //println!("channel list: {:?}", &channellist_result.unwrap());

    //let search: Vec<String> = vec!["id".to_string(), "advisory_synopsis".to_string(), "advisory_name".to_string(), "update_date".to_string()];
    let search: Vec<String> = vec!["id".to_string()];
    let systems: Vec<String> = vec!["client02.bo2go.home".to_string(), "client01.bo2go.home".to_string()];
    let key = login();

    for s in systems {        
        let systems_id = get_systemid(&key, &s);
        match systems_id {
            Err(e) => println!("No server id found for {} - {}", &s, e),
            Ok(i) => {
                server_id_list.push(i);
                println!("list of system id: {:?} ", &server_id_list);
            }
        }        
        
    }

    if server_id_list.len() > 0 {
        for i in &server_id_list {          
            let erratalist = Request::new("system.getRelevantErrataByType").arg(String::from(&key)).arg(*i).arg(String::from("Security Advisory"));
            let erratalist_result = erratalist.call_url(SUMAHOST);
            match erratalist_result {
                Ok(i) => printvalue(&i, &search, &mut id_list),
                Err(e) => println!("no errata found: {:?}", e),
            }

            let mut value_id_list: Vec<Value> = Vec::new();
            for s in &id_list {
                value_id_list.push(Value::Int(*s));
            }
            let patch_errata = Request::new("system.scheduleApplyErrata").arg(String::from(&key)).arg(*i).arg(Value::Array(value_id_list));
            let patch_errata_result = patch_errata.call_url(SUMAHOST);
            match patch_errata_result {
                Ok(s) => {
                    print!("Patch Job ID ");
                    printvalue(&s, &search, &mut id_list);
                },
                Err(e) => println!("no patch job created because the errata list is empty. Maybe your system is up to date.: {:?}", e),
            }
            
        }
    }
    /* let erratalist = Request::new("system.getRelevantErrataByType").arg(String::from(&key)).arg(1000010001).arg(String::from("Security Advisory"));
    
    let erratalist_result = erratalist.call_url(SUMAHOST);
    
    printvalue(&erratalist_result.unwrap(), &search, &mut id_list);
    println!("id_list: {:?}", &id_list); */
    


    //let key = login().unwrap();
    //println!("session key is: {}", &key);
    //let logout_id = logout(&key);
    println!("Logout successful - {}", logout(&key));
}