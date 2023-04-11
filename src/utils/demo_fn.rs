use std::collections::HashMap;

pub fn hello_my() -> String {
    let mut str_a = String::from("hello");
    str_a.push_str(" -> Mike");

    str_a
}

pub fn hello_you(str_a: &mut String) -> &String {
    let str_b = str_a;

    str_b.push_str("--aiiii");

    str_b
}

pub fn run_hello() {
    let str_b = hello_my();
    let str_c = str_b.clone();

    println!("{} -> {}", str_b, str_c);

    let mut str_d = String::from("Jack");
    let str_e = hello_you(&mut str_d);
    println!("str_e -> {}", str_e);

    let str_f = &str_d;
    println!("{} ---->", str_f)
}

// 所有权相关↑

pub fn get_ch_str() {
    let s: String = String::from("我是Mike");
    let len = s.len();
    let some_s = &s[0..len];

    println!("some_s is {}", some_s)
}
// 字符串

pub fn get_tum() {
    let s: (i32, &str) = (12, "eeee");
    let s1 = s.0;

    println!("s1 is {}", s1)
}

struct User {
    name: String,
    age: i32,
}

pub fn test_struct() {
    let martin = User {
        name: String::from("Martin"),
        age: 18,
    };

    let jack = User { ..martin };

    println!("Martin age is {}", martin.age);
    println!("Jack age is {}", jack.name);
}

pub fn test_enum() {
    #[derive(Debug)]
    enum CardProperty {
        One,
        Two,
        Three,
    }

    println!(
        "EveryProperty is {:?}，{:?}，{:?}",
        CardProperty::One,
        CardProperty::Two,
        CardProperty::Three
    );

    #[derive(Debug)]
    enum DetailCard {
        One(u8),
        Two(u8),
    }

    let detail_one = DetailCard::One(2);
    let detail_two = DetailCard::Two(2);
    println!("detail is {:?},{:?}", detail_one, detail_two)
}

pub fn test_array() {
    let arr_1: [String; 8] = core::array::from_fn(|i| String::from("uiuiiu"));
    let slice_1: &[String] = &arr_1[1..3];

    println!("arr is {:?}", arr_1);
    println!("slice is {:?}", slice_1[0]);
}

pub fn test_condition() {
    let res = if 2 + 2 > 5 { "yes" } else { "no" };

    println!("res is {}", res);

    let arr_1 = [String::from("aaaa"), String::from("aaaabbb")];

    for i in &arr_1 {
        println!("i is {}", i);
    }

    for (i, v) in arr_1.iter().enumerate() {
        println!("i is {} v is {}", i, v)
    }

    println!("value is {:?}", arr_1);
}

pub fn test_match() {
    #[derive(Debug)]
    enum DetailData {
        Jack,
        Martin,
        Mike,
    }

    println!(
        "Data is {:?},{:?},{:?}",
        &DetailData::Jack,
        &DetailData::Martin,
        &DetailData::Mike
    );

    println!("eq is {:?}", DetailData::Jack);

    let name = DetailData::Martin;
    let res = match name {
        DetailData::Jack | DetailData::Martin => {
            println!("is Jack Or Martin!!!");
            1
        }

        _ => {
            print!("Other!!!");
            0
        }
    };

    println!("res is {}", res);

    enum PeopleData {
        Name(String),
        Age(String),
        HometownPos(u16, u16),
    }

    let infos = [PeopleData::Name(String::from("Lusty")), PeopleData::Age(String::from("十八")), PeopleData::HometownPos(112, 18)];
    // 穷举
    for info in infos {
        match info {
            PeopleData::Name(name) => {
                println!("name is {}", name)
            },
            PeopleData::Age(age) => {
                println!("age is {}", age)
            },
            PeopleData::HometownPos(x, y, ) => {
                println!("x is {} y is {}", x, y)
            }
        }   
    }

    // if let用法
    let tom = PeopleData::Age(String::from("TOM"));

    match tom {
        PeopleData::Name(name) => {
            println!("name is {}", name)
        },
        _ => {
            println!("no match")
        }
    }

    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }

    let x = 5;

    match x {
        1..=5 => {
            println!("is 5")
        },
        _ => {
            println!("else...")
        }
    }

    struct Point{
        x: i32,
        y: i32
    }
    let point = Point {x: 1, y: 2};
    let Point { x: a, y: b } = point;
    println!("x is {} y is {}", a, b);

}

pub fn test_method () {
    // 先声明结构体
    #[derive(Debug)]
    struct People {
        name: String,
        age: i32
    }
    // 继承结构体
    impl People {
        pub fn new (name: String, age: i32) -> Self {
            People {
                name: name,
                age: age
            }
        }

        pub fn show_info (&self) {
            println!("name is {} age is {}", self.name, self.age)
        }

        pub fn show_info2 (&self) {
            println!("name is {} age is {}", self.name, self.age)
        }

        pub fn name (&self) -> &String {
            &self.name
        }
    }

    let jack =  People {name: "Jack".to_string(), age: 20};
    jack.show_info();
    jack.show_info2();
    println!("value is {}", jack.name());
    println!("value is {:?}", People::new("Martin".to_string(), 99));
}

pub fn test_vector () {
    let v = vec![1,2,3,4];

    let first = v.get(1);
    // let first = v.get(10);

    match first {
        Some(x) => {
            println!("x is {x}")
        },

        _ => {
            println!("nothing!!!")
        }
    }
}

pub fn test_hash () {
    let arr_data = vec![
        ("aaaa".to_string(), 8888),
        ("bbbb".to_string(), 7777),
        ("cccc".to_string(), 6666),
    ];

    let hash_obj: HashMap<_,_> = arr_data.into_iter().collect();

    println!("{:?}", hash_obj);

    let key = "aaaa".to_string();
    println!("{:?}", hash_obj.get(&key))
}

pub fn add_str () {
    let s1 = "hello".to_string();
    let s2 = " world".to_string();
    let s3 = s1 + &s2;
    println!("s3 is {s3}")
}