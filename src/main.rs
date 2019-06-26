use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::cmp::max;
use std::cmp::min;

struct birthday{
    year: u32,
    month: u32,
    day: u32,
}
impl birthday {
    fn print_birthday(&self){
        println!("DATE : {}-{}-{}", &self.year, &self.month, &self.day);
    }

}

struct mem_list{
    id: i32,
    name: String,
    date: birthday,
    address: String,
    comment: String,
}

impl mem_list{
    fn print_list(&self){
        // println!("print list.");
        println!("ID : {}",&self.id);
        println!("NAME : {}",&self.name);
        &self.date.print_birthday();
        println!("ADDRESS : {}",&self.address);
        println!("COMMENT : {}",&self.comment);
    }

    fn to_csv(&self) -> String{
        format!("{},{},{}-{}-{},{},{}\n",
                &self.id,
                &self.name,
                &self.date.year,
                &self.date.month,
                &self.date.day,
                &self.address,
                &self.comment)
    }

    fn is_have(&self, arg: &str) -> bool {
        let date =format!("{}-{}-{}",
                            &self.date.year,
                            &self.date.month,
                            &self.date.day);
        (self.id.to_string() == arg) ||
        (self.name == arg) ||
        (date == arg) ||
        (self.address == arg) ||
        (self.comment == arg)
    }
}

fn main(){
    let mut mem_list : Vec<mem_list> = Vec::with_capacity(100000);

    println!("Hello, World...");

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        parse_line(&s,&mut mem_list);
    }
}

fn cmd_quit(){
    println!("Bye.");
    ::std::process::exit(1);
}

fn cmd_check(list: &mut Vec<mem_list>){
    println!("List item number : {}", list.len());
}

fn cmd_print(list: &mut Vec<mem_list>, arg: &str){
    let item : isize = list.len() as isize;
    if item == 0 {
        println!("No data in the member list.");
    }
    else{
        let arg_num = arg.parse();
        match arg_num{
            Ok(_) => (),
            Err(_) => {
                println!("Wrong argument.");
                return;
            }
        }
        let num : isize = arg_num.unwrap();
        println!("Print.");
        if num > 0{
            println!("plus");
            println!("{}",&num);
            for printer in list[0..min(item,num) as usize].iter() {
                printer.print_list();
            }
        }
        else if num < 0{
            println!("minus");
            let value : isize = max(0, item + num);
            for printer in list[(value as usize)..].iter() {
                printer.print_list();
            }
        }
        else {
            for printer in list.iter(){
                printer.print_list();
            }
        }
    }
}

fn cmd_read(list: &mut Vec<mem_list>, arg: &str){
    let open = File::open(arg);
    match open{
        Ok(_) => {
            println!("Opened [{}]",arg);
            ()},
        Err(_) => {
            println!("Cannot open file.");
            return;
        }
    }
    let file = BufReader::new(open.unwrap());
    for l in file.lines(){
        insert_data(&l.unwrap(), list);
    }
        println!("Read complete.");
}

fn cmd_write(list: &mut Vec<mem_list>, arg: &str){
    let create = File::create(arg);
    match create{
        Ok(_) =>  {
            println!("Created [{}]",arg);
            ()},
        Err(_) => {
            println!("Cannot create file.");
            return;
        }
    }
    let mut file = BufWriter::new(create.unwrap());
    for l in list.iter() {
        let m = format!("{}",l.to_csv());
        file.write(m.as_bytes()).unwrap();
    }
    println!("Write complete.");
}

fn cmd_find(list: &mut Vec<mem_list>, arg: &str){
    println!("Find.");
    for f in list.iter() {
        if f.is_have(arg) {
            f.print_list();
        }
    }
    println!("Find finished.");
}

fn cmd_sort(list: &mut Vec<mem_list>, arg: &str){
    println!("Sort.");
    let arg_num = arg.parse();
    match arg_num{
        Ok(_) => (),
        Err(_) => {
            println!("Wrong argument");
            return;
        }
    }
    list_sort(list,arg_num.unwrap());
}

fn list_sort(list: &mut Vec<mem_list>, num: usize){
    match num{
    1 => list.sort_by(|a, b|a.id.cmp(&b.id)),
    2 => list.sort_by(|a, b|a.name.cmp(&b.name)),
    3 => list.sort_by(|a, b|(a.date.year,a.date.month,a.date.day).cmp(&(b.date.year,b.date.month,b.date.day)) ),
    4 => list.sort_by(|a, b|a.address.cmp(&b.address)),
    5 => list.sort_by(|a, b|a.comment.cmp(&b.comment)),
    _ => {println!("Invalid argument.");}
    }
}

fn cmd_exec(str: &str, list: &mut Vec<mem_list>){
    let v: Vec<char> = str.chars().collect();
    match &v[0] {
        'Q' => cmd_quit(),
        'C' => cmd_check(list),
        'P' => cmd_print(list, &str[1..].trim()),
        'R' => cmd_read(list, &str[1..].trim()),
        'W' => cmd_write(list, &str[1..].trim()),
        'F' => cmd_find(list, &str[1..].trim()),
        'S' => cmd_sort(list, &str[1..].trim()),
        _ => println!("{} is a not command.",v[0]),
    }
}

fn insert_data(str: &str, list: &mut Vec<mem_list>){
    let v: Vec<&str> = str.splitn(5, ",").collect();
    if v.len() < 5 {
        println!("Not enough element.");
        return;
    }
    let ymd: Vec<&str> = v[2].splitn(3,"-").collect();
    if ymd.len() < 3 {
        println!("Wrong date.");
        return;
    }

    let y_result = ymd[0].parse();
    match y_result{
        Ok(_) => (),
        Err(_) => {
            println!("Wrong year.");
            return;
        }
    }

    let m_result = ymd[1].parse();
    match y_result{
        Ok(_) => (),
        Err(_) => {
            println!("Wrong month.");
            return;
        }
    }

    let d_result = ymd[2].parse();
    match y_result{
        Ok(_) => (),
        Err(_) => {
            println!("Wrong day.");
            return;
        }
    }

    let bd = birthday{
        year: y_result.unwrap(),
        month: m_result.unwrap(),
        day: d_result.unwrap(),
    };

    let id_result = v[0].parse();
    match id_result{
        Ok(_) => (),
        Err(_) => {
            println!("Invalid ID.");
            return;
        }
    }
    let data = mem_list{
        id: id_result.unwrap(),
        name: v[1].to_string(),
        date: bd,
        address: v[3].to_string(),
        comment: v[4].to_string(),
    };


    list.push(data);
    println!("Data was saved.");

}

 fn parse_line(str: &str, list: &mut Vec<mem_list>){
     if str.starts_with("%") {cmd_exec(&str[1..],list);}
     else {insert_data(str, list);}
 }
