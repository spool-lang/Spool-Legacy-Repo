use std::error::Error;
use std::fs;
use std::env;
use std::process;

mod nodes {
    pub fn load_class(path : &String) {

    }

    //Represents a fully loaded class.
    pub struct Script {
        pub imports : Vec<Script>,
        pub script_type : ScriptType,
        pub identifier : String
    }

    enum ScriptType {
        Class,
        Singleton,
        Enum,
        Interface,
        Annotation,
        Unknown
    }

    impl Script {

        pub fn new(path : String) -> Script {

            let file_path : String = path.to_owned().push(".silicon");
            let contents : String = fs::read_to_string(&file_path).unwrap_or_else( |err|{
                println!("File {} is not a valid script file!", file_path);
                process::exit(2)
            });

            let mut class = Script {
                imports : Vec::new(),
                script_type : Unknown,
                identifier : ""
            };

            return class
        }

        fn parse() {

        }
    }

    fn remove_file_from_path(path : String) -> String {
        let package_path : String;
        let mut split: Vec<&str> = path.copy.split("/").collect();
        split.pop();

        for slice in &split  {
            package_path.push_str(slice);
            package_path.push_str("/")
        }

        return package_path
    }
}
