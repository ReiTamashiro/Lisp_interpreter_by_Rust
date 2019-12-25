#[derive(Clone)]
pub enum LCons {
    Nil,
    Atom(String),
    Error(String),
    List(Vec<Box<LCons>>)
}

impl LCons {
    fn state(&self) -> String{
        match self {
            LCons::Nil => String::from("nil"),
            LCons::Atom(_atom) => String::from("atom"),
            LCons::List(_list) => String::from("list"),
            LCons::Error(_err) => String::from(_err)
        }
    }

    fn atom_string(&self) -> String{
        match self {
            LCons::Atom(_atom) => String::from(_atom),
            LCons::Error(_err) => String::from(_err),
            _ => String::from("Not atom") 
        }
    }

    fn car (&self) -> LCons{
        match self {
            LCons::Nil => LCons::Nil,
            LCons::Atom(_atom) => LCons::Nil,
            LCons::List(_list) =>{
                if _list.len() == 0 {return LCons::Nil};
                *(_list[0].clone())
            },
            LCons::Error(_err) => {
                if *_err != String::from(""){
                    println!("{}", _err)
                }
                LCons::Error(String::from(""))
            }
        }
    }

    fn cdr (&self) -> LCons{
        match self {
            LCons::Nil => LCons::Nil,
            LCons::Atom(_atom) => LCons::Nil,
            LCons::List(_list) =>{
                if _list.len() == 0 {return LCons::Nil};
                let mut res = _list.clone();
                res.remove(0);
                if res.len() != 0{
                    return LCons::List(res);
                } else {
                    return LCons::Nil;
                };
            },
            LCons::Error(_err) => {
                if *_err != String::from(""){
                    println!("{}", _err)
                }
                LCons::Error(String::from(""))
            }
        }
    }
}

pub fn eval(_exp :&LCons, _env :&LEnv) -> LCons{
    match _exp {
        LCons::Nil => LCons::Nil,
        LCons::Atom(_atom) => _env.search(String::from(_atom)),
        LCons::List(_list) =>{
            if _list.len() == 0 {return LCons::Nil};
            let result = _list.clone();
            match *result[0]{
                LCons::Nil =>{},
                _ =>{}
            };
            let result = LCons::List(vec![]);
            result
        },
        LCons::Error(_err) => {
            if *_err != String::from(""){
                println!("{}", _err)
            }
            LCons::Error(String::from(""))
        }
    }
}

pub struct LVal {
    name: String,
    val : Box<LCons>
}

pub struct LEnv(Vec<Box<LVal>>);

impl LEnv {
    fn search (&self, name: String) -> LCons{
        for item in &*self.0{
            if item.name == name {
                return *item.val.clone()
            }
        }
        LCons::Error(String::from("Not Defined"))
    }
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
    assert_eq!(test_list.car().car().state(), String::from("atom"));
    assert_eq!(test_list.car().cdr().car().state(), String::from("atom"));

    assert_eq!(test_list.car().car().atom_string(), String::from("Alice"));
    assert_eq!(test_list.car().cdr().car().atom_string(), String::from("Bell"));

    assert_eq!(test_list.car().cdr().cdr().state(), String::from("nil"));
    assert_eq!(empty_list.state(), String::from("list"));
    assert_eq!(empty_list.car().state(), String::from("nil"));
    assert_eq!(empty_list.cdr().state(), String::from("nil"));
}

#[test]
fn eval_atom(){
    let atom = LCons::Atom(String::from("Wonderland"));
    let result = atom.clone();
    let input = LCons::Atom(String::from("Alice"));

    let env = LEnv(
        vec![Box::new(LVal{
            name: String::from("Alice"),
            val: Box::new(atom)
        })]
    );
    
    assert_eq!(eval(&input, &env).atom_string(), result.atom_string());
}