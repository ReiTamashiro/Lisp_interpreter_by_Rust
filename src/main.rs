#[derive(Clone)]
enum LCons {
    Nil,
    Atom{car: String, cdr: Box<LCons>},
    List{car: Box<LCons>, cdr: Box<LCons>}
}

impl LCons {
    // add code here
    fn state_print(&self) {
        match self {
            LCons::Nil => println!("Nil"),
            LCons::Atom{car, cdr} => println!("Atom"),
            LCons::List{car, cdr} => {
                match **car {
                    LCons::Nil => println!("Nil"),
                    _ => println!("other")
                };
                match **cdr {
                    LCons::Nil => println!("Nil"),
                    _ => println!("other")
                };
            }
        };
    }
}

fn eval(exp :&LCons, env:&LEnv) -> LCons{
    match exp {
        LCons::Nil => exp.clone(),
        LCons::Atom{car, cdr} => {
            if *car == String::from(""){

            };
            LCons::Nil
        },
        LCons::List{car, cdr} =>{
            let result = LCons::List{
                car: Box::new(eval(&car, &env)),
                cdr: Box::new(eval(&cdr, &env))
            };
            result
        }
    }
}

fn eval_atom(_exp :&LCons, _env:&LEnv) -> LCons{

    return LCons::Nil
}

struct LVal {
    name: String,
    val : Box<LCons>
}

struct LEnv {
    list: Vec<Box<LVal>>
}

fn main() {
    let nil = LCons::Nil;
    let envi = LEnv{
        list: vec![]
    };

    let cop = eval(&nil, &envi);

    eval(&nil, &envi).state_print();
    nil.state_print();
    cop.state_print();
}
