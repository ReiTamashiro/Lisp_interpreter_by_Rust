#[derive(Clone)]
enum LCons {
    Nil,
    Atom(String),
    List(Vec<Box<LCons>>)
}

impl LCons {
    fn state(&self) -> String{
        match self {
            LCons::Nil => String::from("nil"),
            LCons::Atom(_atom) => String::from(_atom),
            LCons::List(_list) => String::from("list")
        }
    }

    fn car (&self) -> LCons{
        match self {
            LCons::Nil => self.clone(),
            LCons::Atom(_atom) => self.clone(),
            LCons::List(_list) =>{
                if _list.len() == 0 {return LCons::Nil};
                *(_list[0].clone())
            }
        }
    }

    fn cdr (&self) -> LCons{
        match self {
            LCons::Nil => self.clone(),
            LCons::Atom(_atom) => self.clone(),
            LCons::List(_list) =>{
                if _list.len() == 0 {return LCons::Nil};
                let mut res = _list.clone();
                res.remove(0);
                if res.len() != 0{
                    return LCons::List(res);
                } else {
                    return LCons::Nil;
                };
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
}

#[test]
fn car_cdr_test(){
    let test_list = LCons::List(vec![
        Box::new(LCons::Atom(String::from("Alice"))),
        Box::new(LCons::Atom(String::from("Bell")))
    ]);
    let test_list = LCons::List(vec![
        Box::new(test_list)
    ]);
    let empty_list = LCons::List(vec![]);

    assert_eq!(test_list.car().state(), String::from("list"));
    assert_eq!(test_list.car().car().state(), String::from("Alice"));
    assert_eq!(test_list.car().cdr().car().state(), String::from("Bell"));
    assert_eq!(test_list.car().cdr().cdr().state(), String::from("nil"));
    assert_eq!(empty_list.state(), String::from("list"));
    assert_eq!(empty_list.car().state(), String::from("nil"));
    assert_eq!(empty_list.cdr().state(), String::from("nil"));
}
