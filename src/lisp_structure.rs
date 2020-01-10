#[derive(Clone)]
pub struct LVal {
    name: String,
    val : Box<LCons>
}

#[derive(Clone)]
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
            match &*result[0]{
                LCons::Nil =>{return _exp.cdr()},
                LCons::Atom(_atom) => {
                    if *_atom == String::from("quote"){
                        //need argments count check
                        return *result[1].clone();
                    }
                    else if *_atom == String::from("if"){
                        if eval(&_exp.cdr().car(), &_env).state() != String::from("nil") {
                            return eval(&_exp.cdr().cdr().car(), &_env)
                        }
                        else {
                            return eval(&_exp.cdr().cdr().cdr().car(), &_env)
                        }
                    }
                    else if *_atom == String::from("lambda") {

                    }
                },
                LCons::List(_list) =>{
                    if _list.len() == 0 {return LCons::Nil};
                    match &*_list[0]{
                        LCons::Atom(_atom) =>{
                            if *_atom == String::from("lambda"){
                                let mut input = _list.clone();
                                let mut res = vec![Box::new(*input.remove(0)), Box::new(*input.remove(0))];
                                res.append(&mut input);
                                let res = LCons::List(res);
                                return eval(&res, &_env);
                            }
                        },
                        _ =>{}
                    }
                    return LCons::Nil
                },
                _ =>{}
            };
            LCons::Nil
        },
        LCons::Error(_err) => {
            if *_err != String::from(""){
                println!("{}", _err)
            }
            LCons::Error(String::from(""))
        }
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
    let atom = LCons::Atom(String::from("Alice"));
    let result = atom.clone();
    let input = LCons::Atom(String::from("Wonderland"));

    let env = LEnv(
        vec![Box::new(LVal{
            name: input.atom_string(),
            val: Box::new(atom)
        })]
    );

    let quote = LCons::List(vec![
        Box::new(LCons::Atom(String::from("quote"))),
        Box::new(input.clone())
    ]);
    
    //(Wonderland)
    assert_eq!(eval(&input, &env).atom_string(), result.atom_string());
    //(checktype (car Wonderland))
    assert_eq!(eval(&input, &env).car().state(), result.car().state());
    //(quote Wonderland)
    assert_eq!(eval(&quote, &env).atom_string(), input.atom_string());
    //(eval (quote Wonderland))
    assert_eq!(eval(&eval(&quote, &env), &env).atom_string(), result.atom_string());
}

#[test]
fn deproyment_lambda(){
    let dummy_args = LCons::List(vec![
        Box::new(LCons::Atom(String::from("x"))),
        Box::new(LCons::Atom(String::from("y"))),
        Box::new(LCons::Atom(String::from("z")))
    ]);
    let lam = LCons::Atom(String::from("lambda"));
    let lam = LCons::List(vec![
        Box::new(lam),
        Box::new(dummy_args)
    ]);
    let dummy = LCons::List(vec![
        Box::new(lam.clone())
    ]);
    let dummy_fn = LCons::Atom(String::from("Fn"));

    let env = LEnv(
        vec![Box::new(LVal{
            name: String::from("Fn"),
            val: Box::new(lam.clone())
        })]
    );

    assert_eq!(eval(&dummy, &env).state(), lam.state());
    assert_eq!(eval(&dummy, &env).car().atom_string(), lam.car().atom_string());
    assert_eq!(eval(&dummy_fn, &env).car().atom_string(), lam.car().atom_string());
}