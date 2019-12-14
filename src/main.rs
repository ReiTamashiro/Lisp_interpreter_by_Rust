#[derive(Clone)]
enum LCons {
    Nil,
    Atom(String),
    List(Vec<Box<LCons>>)
}

impl LCons {
    fn state(&self){
        match self {
            LCons::Nil => println!("nil"),
            LCons::Atom(_atom) => println!("atom"),
            LCons::List(_list) => println!("list")
        }
    }

    fn print_state(&self){
        match self {
            LCons::Nil => println!("nil"),
            LCons::Atom(_atom) => println!("{}", _atom),
            LCons::List(_list) => println!("list")
        }
    }

    fn car (&self) -> LCons{
        match self {
            LCons::Nil => self.clone(),
            LCons::Atom(_atom) => self.clone(),
            LCons::List(_list) =>{
                *(_list[0].clone())
            }
        }
    }

    fn cdr (&self) -> LCons{
        match self {
            LCons::Nil => self.clone(),
            LCons::Atom(_atom) => self.clone(),
            LCons::List(_list) =>{
                let mut res = _list.clone();
                res.remove(0);
                let res = LCons::List(res);
                res
            }
        }
    }
}

fn eval(exp :&LCons, env :&LEnv) -> LCons{
    match exp {
        LCons::Nil => exp.clone(),
        LCons::Atom(_atom) => exp.clone(),
        LCons::List(_list) =>{
            let result = LCons::List(vec![]);
            result
        }
    }
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
    let test_list = LCons::List(vec![
        Box::new(LCons::Atom(String::from("Alice"))),
        Box::new(LCons::Atom(String::from("Bell"))),
        Box::new(LCons::Nil)
    ]);
    let test_list = LCons::List(vec![
        Box::new(test_list),
        Box::new(LCons::Nil)
    ]);
    let mut envi = LEnv{
        list: vec![Box::new(LVal{
            name: String::from("NIIIIL"),
            val: Box::new(LCons::Nil)
        })]
    };

    test_list.car().state();
    test_list.car().car().print_state();
    test_list.car().cdr().car().print_state();
}
