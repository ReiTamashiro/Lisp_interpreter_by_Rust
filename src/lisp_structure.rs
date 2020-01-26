#[derive(Clone)]
pub struct LVal {
    name: String,
    val : Box<LCons>
}

#[derive(Clone)]
pub struct LEnv(Vec<Box<LVal>>);

impl LEnv {
    fn search (&self, name: String) -> Result<LCons, ()>{
        for item in &*self.0{
            if item.name == name {
                return Ok(*item.val.clone())
            }
        }
        Err(())
    }

    fn add(&mut self, value: LVal){
        let LEnv(envi) = self;
        envi.push(Box::new(value.clone()));
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

    fn atom_string(&self) -> Result<String, String>{
        match self {
            LCons::Nil => Ok(String::from("Nil")),
            LCons::Atom(_atom) => Ok(String::from(_atom)),
            LCons::List(_list) => Err(String::from("List")),
            LCons::Error(e) => Err(String::from(e))
        }
    }

    fn is_list(&self) -> Result<Vec<Box<LCons>>, LCons>{
        match &*self {
            LCons::List(_list) => Ok((*_list).to_vec()),
            LCons::Nil => Ok(vec![]),
            _ => Err(self.clone())
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

pub fn eval(_exp :&LCons, _env :&mut LEnv) -> LCons{
    match _exp {
        LCons::Nil => LCons::Nil,
        LCons::Atom(_atom) => match _env.search(String::from(_atom)) {
            Ok(_ret) => _ret,
            Err(()) => _exp.clone()
        },
        LCons::List(_list) =>{
            let tmp_env = &mut _env.clone();
            if _list.len() == 0 {return LCons::Nil};
            let exp = _list.clone();
            match &*exp[0]{
                LCons::Nil =>{return _exp.cdr()},
                LCons::Atom(_atom) => {
                    if *_atom == String::from("quote"){
                        //need argments count check
                        return *exp[1].clone();
                    }
                    else if *_atom == String::from("if"){
                        if eval(&_exp.cdr().car(), tmp_env).state() != String::from("nil") {
                            return eval(&_exp.cdr().cdr().car(), tmp_env)
                        }
                        else {
                            return eval(&_exp.cdr().cdr().cdr().car(), tmp_env)
                        }
                    }
                    else if *_atom == String::from("lambda") {
                            let _args = match exp[1].is_list() {
                                Ok(_list) => _list,
                                Err(e) => {return e}
                            };
                            let mut env = LEnv(vec![]);
                            for (line, arg) in _args.iter().enumerate() {
                                env.add(LVal{
                                    name: arg.atom_string().unwrap(),
                                    val: Box::new(*exp[line + 3].clone())
                                });
                            }
                            let env = &mut env;
                            return eval(&*exp[2].clone(), env)

                    }
                    else if *_atom == String::from("+") {
                        return LCons::Atom((eval(&*exp[1], tmp_env).atom_string().unwrap().parse::<i32>().unwrap() + (eval(&*exp[2], tmp_env).atom_string().unwrap().parse::<i32>().unwrap())).to_string())
                    }
                    else {
                        let mut input = exp.clone();
                        input.remove(0);
                        let mut after_conversion = match _env.search(String::from(_atom)) {
                            Ok(_ret) => vec![Box::new(_ret)],
                            Err(()) => vec![]
                        };

                        for args in input {
                            after_conversion.push(Box::new(eval(&*args, tmp_env)))
                        }

                        if exp.len() == after_conversion.len(){
                            let after_conversion = LCons::List(after_conversion);
                            return eval(&after_conversion, tmp_env)
                        }

                        let after_conversion = LCons::List(after_conversion);
                        return after_conversion

                        
                    }
                },
                LCons::List(__list) =>{
                    if __list.len() == 0 {return LCons::Nil};
                    match &*__list[0]{
                        LCons::Atom(_atom) =>{
                            if *_atom == String::from("lambda"){
                                let mut input = _list.clone();
                                input.remove(0);
                                let mut define = __list.clone();
                                let mut res = vec![Box::new(*define.remove(0)), Box::new(*define.remove(0)), Box::new(*define.remove(0))];
                                
                                for args in input {
                                    res.push(args)
                                }

                                let res = res;

                                let res = LCons::List(res);

                                return eval(&res, tmp_env);
                            }
                            return LCons::Atom(String::from(_atom))
                        },
                        _ =>{return eval(&_list[0], tmp_env)}
                    }
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

    assert_eq!(test_list.car().car().atom_string().unwrap(), String::from("Alice"));
    assert_eq!(test_list.car().cdr().car().atom_string().unwrap(), String::from("Bell"));

    assert_eq!(test_list.car().cdr().cdr().state(), String::from("nil"));
    assert_eq!(empty_list.state(), String::from("list"));
    assert_eq!(empty_list.car().state(), String::from("nil"));
    assert_eq!(empty_list.cdr().state(), String::from("nil"));
}

#[test]
fn eval_atom(){
    let atom = LCons::Atom(String::from("value"));
    let result = atom.clone();
    let input = LCons::Atom(String::from("name"));

    let env = &mut LEnv(
        vec![Box::new(LVal{
            name: input.atom_string().unwrap(),
            val: Box::new(atom)
        })]
    );

    let quote = LCons::List(vec![
        Box::new(LCons::Atom(String::from("quote"))),
        Box::new(input.clone())
    ]);
    
    //(name)
    assert_eq!(eval(&input, env).atom_string(), result.atom_string());
    //(checktype (car name))
    assert_eq!(eval(&input, env).car().state(), result.car().state());
    //(quote name)
    assert_eq!(eval(&quote, env).atom_string(), input.atom_string());
    //(eval (quote name))
    assert_eq!(eval(&eval(&quote, env), env).atom_string(), result.atom_string());
}

#[test]
fn plus_fun(){
    let dummy = LCons::List(vec![
        Box::new(LCons::Atom(String::from("+"))),
        Box::new(LCons::Atom(String::from("1"))),
        Box::new(LCons::Atom(String::from("2")))
    ]);

    let env = &mut LEnv(vec![]);

    assert_eq!(eval(&dummy, env).atom_string().unwrap(), String::from("3"));
}

#[test]
fn deproyment_lambda(){
    let dummy_args = LCons::List(vec![
        Box::new(LCons::Atom(String::from("x"))),
        Box::new(LCons::Atom(String::from("y")))
    ]);
    let dummy_fn = LCons::List(vec![
        Box::new(LCons::Atom(String::from("+"))),
        Box::new(LCons::Atom(String::from("x"))),
        Box::new(LCons::Atom(String::from("y")))
    ]);
    let lam = LCons::Atom(String::from("lambda"));
    let lam = LCons::List(vec![
        Box::new(lam),
        Box::new(dummy_args),
        Box::new(dummy_fn)
    ]);
    let dummy = LCons::List(vec![
        Box::new(LCons::Atom(String::from("Fn"))),
        Box::new(LCons::Atom(String::from("1"))),
        Box::new(LCons::Atom(String::from("2")))
    ]);

    let env = &mut LEnv(
        vec![Box::new(LVal{
            name: String::from("Fn"),
            val: Box::new(lam.clone())
        })]
    );

    //(define Fn (quote (lambda (x y) (+ x y))))
    //(Fn 1 2)
    assert_eq!(eval(&dummy, env).atom_string().unwrap(), String::from("3"));
}