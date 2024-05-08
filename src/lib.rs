pub struct Arguments {
    gotargs: Vec<String>,
    allargs: Vec<String>,
    arguments: Vec<String>,
    errors: Vec<String>,
    data: Vec<ArgumentDescription>,
    used: Vec<String>,
}

pub struct ArgumentDescription {
    name: String,
    description: String,
    shorter_counterpart: String,
}

impl Arguments {
    pub fn new(recvd_args: Vec<String>) -> Self {
        Arguments {
            gotargs: recvd_args,
            allargs: Vec::new(),
            arguments: Vec::new(),
            errors: Vec::new(),
            data: Vec::new(),
            used: Vec::new(),
        }
    }

    pub fn add(&mut self, arg: &str, description: ArgumentDescription) {
        self.allargs.push(arg.to_string());
        self.data.push(description);
    }

    pub fn get_arg_by_number(&self, number: usize) -> String {
        self.gotargs[number - 1].clone()
    }

    pub fn get_arg_by_index(&self, index: usize) -> String {
        self.gotargs[index].clone()
    }

    pub fn get_arg_description(&self, arg: &str) -> ArgData {
        let mut description: Vec<String> = vec![];
        // for all values in self.allargs, if the arg matches, then append all its contents in the description
        for x in 0..=self.allargs.len() - 1 {
            if self.allargs[x] == arg.to_string()
                || self.data[x].shorter_counterpart == arg.to_string()
            {
                description.push(self.data[x].name.clone());
                description.push(self.data[x].description.clone());
                description.push(self.data[x].shorter_counterpart.clone());
                break;
            }
        }

        ArgData { data: description }
    }

    pub fn remove(&mut self, arg: &str) {
        let mut new = Arguments {
            gotargs: self.gotargs.clone(),
            allargs: Vec::new(),
            arguments: Vec::new(),
            errors: Vec::new(),
            data: Vec::new(),
            used: self.used.clone(),
        };

        for x in 0..=self.allargs.len() - 1 {
            if self.allargs[x] == arg.to_string()
                || self.data[x].shorter_counterpart == arg.to_string()
            {
                continue;
            } else {
                new.allargs.push(self.allargs[x].clone());
                new.data.push(self.data[x].clone());
            }
        }

        self.allargs = new.allargs;
        self.data = new.data;
    }

    pub fn analyse(&mut self) {
        // check if arguments are already there, if there, clear them and star anew
        if self.arguments.len() != 0 {
            self.arguments.clear();
            self.errors.clear();
        }

        // start
        for x in 0..=self.allargs.len() - 1 {
            if self.gotargs.contains(&self.allargs[x])
                || self.gotargs.contains(&self.data[x].shorter_counterpart)
            {
                self.arguments.push(self.allargs[x].clone());
            }
        }
    }

    pub fn ifarg(&mut self, arg: &str) -> bool {
        if self.arguments.len() == 0 {
            return false;
        }

        let mut flag = false;

        //basic check
        if self.arguments.contains(&arg.to_string()) {
            flag = true;
            self.used.push(arg.to_string());
        }

        // if not found still
        if !flag {
            for x in 0..=self.allargs.len() - 1 {
                if (self.arguments.contains(&self.data[x].shorter_counterpart)
                    || self.arguments.contains(&self.allargs[x]))
                    && (self.allargs[x] == arg.to_string()
                        || self.data[x].shorter_counterpart == arg.to_string())
                {
                    flag = true;
                    self.used.push(self.allargs[x].clone());
                    break;
                }
            }
        }

        flag
    }

    pub fn ifarg_force(&mut self, arg: &str) -> bool {
        if self.arguments.len() == 0 {
            return false;
        }

        let mut flag = false;

        //basic check
        if self.arguments.contains(&arg.to_string()) {
            flag = true;
        }

        // if not found still
        if !flag {
            for x in 0..=self.allargs.len() - 1 {
                if (self.arguments.contains(&self.data[x].shorter_counterpart)
                    || self.arguments.contains(&self.allargs[x]))
                    && (self.allargs[x] == arg.to_string()
                        || self.data[x].shorter_counterpart == arg.to_string())
                {
                    flag = true;
                    break;
                }
            }
        }

        flag
    }

    pub fn fetch(&mut self, arg: &str, fetch: FetchTypes) -> ArgData {
        let mut values: Vec<String> = vec![];

        // at first check if the arg exists
        if self.ifarg_force(arg) {
            // if exist
            let expected = match fetch {
                FetchTypes::Fetch(n) => n,
                FetchTypes::TillLast => 100,
                FetchTypes::TillNext => 200,
            };

            // get the index of the argument.
            let mut index: usize = 100;
            for x in 0..=self.gotargs.len() - 1 {
                if self.gotargs[x] == arg.to_string() {
                    index = x;
                    break;
                }
            }

            // if still unchanged, check smaller counterpart
            if index == 100 {
                for x in 0..=self.allargs.len() - 1 {
                    if self.data[x].shorter_counterpart == arg.to_string() || self.allargs[x] == arg.to_string() {
                        for y in 0..=self.gotargs.len()-1 {
                            if self.gotargs[y] == self.data[x].shorter_counterpart || self.gotargs[y] == self.allargs[x] {
                                index = y;
                                break;
                            }
                        }
                    }
                }
            }

            // for index to index+expected (if expected is not 100)
            if expected!=100 && expected!=200 {
                for x in index+1..=index+expected {
                    values.push(self.gotargs[x].clone());
                }
            } else if expected == 100 {
                for x in index+1..=self.gotargs.len()-1 {
                    values.push(self.gotargs[x].clone());
                }
            } else if expected == 200 {
                // TillNext
                let index2; // index of first arg
                let firstarg = self.gotargs[index].clone(); // first arg
                // println!("{}", firstarg);
                // find counterpart too
                let mut counterpart = String::from("");
                for x in 0..=self.allargs.len()-1 {
                    if self.allargs[x] == firstarg {
                        counterpart = self.data[x].shorter_counterpart.clone();
                        break;
                    } else if self.data[x].shorter_counterpart == firstarg {
                        counterpart = self.allargs[x].clone();
                        break;
                    }
                }

                if let Some(ind) = self.arguments.iter().position(|s| s == &firstarg || s==&counterpart) {
                    if ind < self.arguments.len()-1 {index2 = ind;}
                    else {
                        return self.fetch(arg, FetchTypes::TillLast);
                    }
                } else {
                    index2 = 200;
                }
                
                let secondarg: String;
                if index2!=200 {
                    secondarg = self.arguments[index2+1].clone();
                } else {
                    secondarg = "".to_string();
                }
                if secondarg!="".to_string() {
                    for x in index..=self.gotargs.len()-1 {
                        if self.gotargs[x] == firstarg {
                            for y in x+1..=self.gotargs.len()-1 {
                                if self.gotargs[y]!=secondarg {
                                    values.push(self.gotargs[y].clone());
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    // if no arg is present
                    values = self.fetch(arg, FetchTypes::TillLast).get();
                }
                
            }
        } else {}

        ArgData { data: values }
    }
}

impl ArgumentDescription {
    // create a new instance
    pub fn new() -> Self {
        ArgumentDescription {
            name: String::new(),
            description: String::new(),
            shorter_counterpart: String::new(),
        }
    }
    // add name
    pub fn name(&mut self, name: &str) -> Self {
        ArgumentDescription {
            name: name.to_string(),
            description: self.description.clone(),
            shorter_counterpart: self.shorter_counterpart.clone(),
        }
    }
    // add description
    pub fn description(&mut self, desc: &str) -> Self {
        ArgumentDescription{
            name:self.name.clone(),
            description:desc.to_string(),
            shorter_counterpart:self.shorter_counterpart.clone(),
        }
    }
    // add shorter_counterpart
    pub fn short(&mut self, shortarg: &str) -> Self {
        ArgumentDescription{
            name:self.name.clone(),
            description:self.description.clone(),
            shorter_counterpart:shortarg.to_string(),
        }
    }

    pub fn clone(&self) -> Self {
        ArgumentDescription {
            name: self.name.clone(),
            description: self.description.clone(),
            shorter_counterpart: self.shorter_counterpart.clone(),
        }
    }
}

pub struct ArgData {
    data: Vec<String>,
}

impl ArgData {
    pub fn get(&self) -> Vec<String> {
        self.data.clone()
    }
}

pub enum FetchTypes {
    Fetch(usize),
    TillLast,
    TillNext,
}


#[cfg(test)]
mod all_tests {
    use super::*;

    #[test]
    fn add_new() {
        let args = Arguments::new(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(args.gotargs.len(), 2);
    }

    #[test]
    fn add() {
        let mut args = Arguments::new(vec!["-a".to_string(), "--b".to_string()]);
        let desc_a = ArgumentDescription::new()
            .name("a")
            .description("hehe")
            .short("-a");

        args.add("--a", desc_a);

        assert_eq!(args.data[0].name, "a".to_string());
        assert_eq!(args.data[0].description, "hehe".to_string());
        assert_eq!(args.data[0].shorter_counterpart, "-a".to_string());
    }

    #[test]
    fn get_arg_by_number() {
        let args = Arguments::new(vec!["-a".to_string(), "--b".to_string()]);
        assert_eq!(args.get_arg_by_number(1), "-a".to_string());
        assert_eq!(args.get_arg_by_number(2), "--b".to_string());
    }

    #[test]
    fn get_arg_by_index() {
        let args = Arguments::new(vec!["-a".to_string(), "--b".to_string()]);
        assert_eq!(args.get_arg_by_index(0), "-a".to_string());
        assert_eq!(args.get_arg_by_index(1), "--b".to_string());
    }

    #[test]
    fn get_arg_description() {
        let mut args = Arguments::new(vec!["-a".to_string(), "--b".to_string()]);
        let desc_a = ArgumentDescription::new()
            .name("a")
            .description("hehe")
            .short("-a");

        args.add("--a", desc_a);

        assert_eq!(args.get_arg_description("-a").get(), args.get_arg_description("--a").get());
    }

    #[test]
    fn remove() {
        {
            let mut args = Arguments::new(vec!["-a".to_string(), "--b".to_string()]);
            let desc_a = ArgumentDescription::new()
                .name("a")
                .description("hehe")
                .short("-a");

            args.add("--a", desc_a);

            args.remove("-a");

            assert!(args.allargs.len()==0);
        }

        {
            let mut args = Arguments::new(vec!["-a".to_string(), "--b".to_string()]);
            let desc_a = ArgumentDescription::new()
                .name("a")
                .description("hehe")
                .short("-a");

            args.add("--a", desc_a);

            args.remove("--a");

            assert!(args.allargs.len()==0);
        }

    }

    #[test]
    fn analyse() {
        let mut args = Arguments::new(vec!["-a".to_string(), "--b".to_string()]);
        let desc_a = ArgumentDescription::new()
            .name("a")
            .description("hehe")
            .short("-a");

        args.add("--a", desc_a);

        args.analyse();

        assert!(args.arguments.len()==1);
    }

    #[test]
    fn ifarg(){
        let mut args = Arguments::new(vec!["-a".to_string(), "--b".to_string()]);
        let desc_a = ArgumentDescription::new()
            .name("a")
            .description("hehe")
            .short("-a");

        args.add("--a", desc_a);

        args.analyse();

        assert!(args.ifarg("-a"));
        assert!(args.used.len()==1);
    }

    #[test]
    fn fetch() {
        let mut args = Arguments::new(vec!["-a".to_string(), "hh".to_string(), "hehe".to_string(), "--b".to_string(), "hehe".to_string()]);
        let desc_a = ArgumentDescription::new()
            .name("a")
            .description("hehe")
            .short("-a");

        args.add("--a", desc_a);

        let desc_b = ArgumentDescription::new()
            .name("b")
            .description("hehe")
            .short("-b");
        args.add("--b", desc_b);

        args.analyse();

        assert_eq!(args.fetch("-a", FetchTypes::Fetch(1)).get(), vec!["hh".to_string()]);
        assert_eq!(args.fetch("--a", FetchTypes::TillNext).get(), vec!["hh".to_string(), "hehe".to_string()]);
        assert_eq!(args.fetch("-a", FetchTypes::TillNext).get(), vec!["hh".to_string(), "hehe".to_string()]);

        assert_eq!(args.fetch("-b", FetchTypes::TillNext).get(), vec!["hehe".to_string()]);

        assert_eq!(args.fetch("--b", FetchTypes::TillLast).get(), vec!["hehe".to_string()]);
    }
} 