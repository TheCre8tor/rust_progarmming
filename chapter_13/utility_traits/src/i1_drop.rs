use std::ops::Drop;

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);

        if !self.nicknames.is_empty() {
            {
                print!("(AKA {})", self.nicknames.join(", "));
            }
        }

        println!("")
    }
}

pub fn run() {
    let mut a = Appellation {
        name: "Zeus".to_owned(),
        nicknames: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };

    println!("before assingnment");
    a = Appellation {
        name: "Hera".to_string(),
        nicknames: vec![],
    };
    println!("at the end of block");
}
