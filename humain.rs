struct Human {
    _name: String,
    _age: u8,
}

impl Human {

    fn getTralalaWith(&self, target: &Human) {
        if (self._age >= 18 && target._age >= 18) {
            target.whine();
        }
        else {
            if (self._age < 18) {
                println!("{} is not 18' right now. Stop trying to have tralala before 18.", self._name);
            }
            else {
                println!("{} is not 18' right now. Don't touch.", target._name);
            }
        }
    }

    fn say(&self, what: &str) {
        println!("{}: {}", self._name, what);
    }

    fn whine(&self) {
        self.say("Ohhh yeahh...");
    }
}

fn main() {
    let mut h1 = Human {_name: "Trololo".to_string(), _age: 20};
    let mut h2 = Human {_name: "Ponyporn".to_string(), _age: 20};
    let mut h3 = Human {_name: "Little Ponyporn".to_string(), _age: 12};
    h1.getTralalaWith(&h2);
    h1.getTralalaWith(&h3);
    h3.getTralalaWith(&h2);
}
