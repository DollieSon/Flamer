use std::env;
const ALPHABET:usize = 26;
const BASE:usize = 'a' as usize;

macro_rules! str_to_arr {
    ($vec:ident,$str:ident ) => {
        for ch in $str.chars(){
            if ch == ' ' {
                continue;
            }
            let ind = (ch.to_ascii_lowercase() as usize) - BASE;
            $vec[ind] +=1
        }
    };
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let mut iter = args.iter();
    iter.next().unwrap();
    let name1 = iter.next().expect("Name 1 not provided");
    let name2 = iter.next().expect("Name 2 not provided");
    println!("Names : {} and {}",name1,name2);
    let meanings:Vec<String> = vec![
        "Friends".to_string(),
        "Lovers".to_string(),
        "Affectionate".to_string(),
        "Marriage".to_string(),
        "Enemies".to_string(),
        "Siblings".to_string(),
    ];
    let res = flame(name1, name2) as usize % meanings.len();
    println!("Flame Result : {res}");
    println!("{} and {} are {}",name1,name2,meanings[res]);
}

pub fn flame(name_1:&String,name_2:&String)-> i32{
    let mut name_vec1  = [0;ALPHABET];
    let mut name_vec2 = [0;ALPHABET];

    str_to_arr!(name_vec1,name_1);
    str_to_arr!(name_vec2,name_2);

    let mut total = 0;
    for ind in 0..name_vec1.len(){
        let val1 = name_vec1[ind];
        let val2 = name_vec2[ind];
        if val1 == val2{
            total += val1 + val2;
        }
    }

    return total;
}