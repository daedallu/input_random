extern crate rand;
use rand::seq::SliceRandom;
//use rand::{seq::IteratorRandom, thread_rng};
use rand::Rng;
use itertools::Itertools;
fn main(){
    let mut vecuno: Vec<String> = Vec::new();
	let mut vecuno_two: Vec<String> = Vec::new();
    let mut nm = String::new();
    let mut snm = String::new();
    let mut chd = String::new();
    let mut bd = String::new();

    println!("NAME= ");
    std::io::stdin().read_line(&mut nm).expect("failed to read the input.");
    println!("SURNAME= ");
    std::io::stdin().read_line(&mut snm).expect("failed to read the input.");
    println!("CHILD= ");
    std::io::stdin().read_line(&mut chd).expect("failed to read the input.");
    println!("BIRTHDAY= ");
    std::io::stdin().read_line(&mut bd).expect("failed to read the input.");
    
    let target = build_target(nm, snm, chd, bd);
    let nm = target.name;
    let snm = target.surname;
    let chd = target.child;
    let bd = target.birthdate;

    
    let nmcl = nm.replace("\n","");
    let snmcl = snm.replace("\n","");
    let chdcl = chd.replace("\n","");
    let bdcl = bd.replace("\n","");
   
    
    vecuno.push(nmcl);
    vecuno.push(snmcl);
    vecuno.push(chdcl);
    vecuno.push(bdcl);
    
//Vecuno have the struct inputed data
	for i in vecuno.clone(){
		let rep = replacement_one(i);
		vecuno_two.push(rep);
	}
	vecuno.append(&mut vecuno_two);
	let veclen: usize = vecuno.len();
	let loop_tries: usize = fac(veclen);
    let mut finalist: Vec<String> = Vec::new();
    let mut n = 0;
    //println!("{:#?}", vecuno);
    while n < loop_tries{
        let res = take_random(vecuno.clone());
        n = n +1;
        finalist.push(res);
    }
    let finalist: Vec<_> = finalist.into_iter().unique().collect();
    println!("There'll be {} strings!", finalist.len());
    for i in finalist{
        let lows = i.to_lowercase();
        let cutws = lows.replace(" ", ""); 
        println!("{cutws}");
        
    }
}
//Struct that will format the data entries

struct Target{
	name: String,
	surname: String,
	child: String,
	birthdate: String,
}

// a function to build de target's data set
fn build_target(name: String, surname: String, child: String, birthdate: String) -> Target{
	Target{
		name, 
		surname,
		child,
		birthdate,
	}
}
// a function to make random string with Vecuno's content
fn take_random(veca: Vec<String>) -> String{
	let mut stringus: Vec<String> = Vec::new();
	let mut rng = rand::thread_rng();
	let mut n = 0;
	while n < rng.gen_range(1..8){
		let happy_choice = veca.choose(&mut rand::thread_rng());
		let tostr = happy_choice
			.expect("Oops! NOT to string.")
			.to_string();
			n = n + 1;
			stringus.push(tostr);

};
	let joint = stringus.join("");
	return joint
}

// a func to replace some letters with special chars

fn replacement_one(s: String) -> String{
    let cut_wspace = s.replace(" ","");
    let repla = cut_wspace.replace("a", "@");
    let reple = repla.replace("e", "3");
    let repli = reple.replace("i", "1");
    let replo = repli.replace("o", "0");
    let replh = replo.replace("h", "#");
    let repls = replh.replace("s", "5");
    let replg = repls.replace("g", "9");
    let replz = replg.replace("z", "2");
    return replz

}

// a simple function to factor and return some likely amount of strings

fn fac (number: usize) -> usize{
    if number <=1{
        return 1;
    }
    return number * fac(number-1);
}
