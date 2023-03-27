use std::any::type_name;

fn test_type<T>(_: T) {
    println!("{:?}", { type_name::<T>() });
}

fn hello_my() -> String {
    let mut str_a = String::from("hello");
    str_a.push_str(" -> Mike");

    str_a
}

fn hello_you(str_a: &mut String) -> &String {
    let str_b = str_a;

    str_b.push_str("--aiiii");

    str_b
}

fn run_hello() {
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

fn get_ch_str() {
    let s: String = String::from("我是Mike");
    let len = s.len();
    let some_s = &s[0..len];

    println!("some_s is {}", some_s)
}
// 字符串

fn get_tum() {
    let s: (i32, &str) = (12, "eeee");
    let s1 = s.0;

    println!("s1 is {}", s1)
}

struct User {
    name: String,
    age: i32,
}

fn test_struct() {
    let martin = User {
        name: String::from("Martin"),
        age: 18,
    };

    let jack = User { ..martin };

    println!("Martin age is {}", martin.age);
    println!("Jack age is {}", jack.name);
}

fn test_enum() {
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

fn test_array() {
    let arr_1: [String; 8] = core::array::from_fn(|i| String::from("uiuiiu"));
    let slice_1: &[String] = &arr_1[1..3];

    println!("arr is {:?}", arr_1);
    println!("slice is {:?}", slice_1[0]);
}

fn test_condition() {
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

fn test_match() {
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
        Hometown(x, y),
    }
}

fn main() {
    run_hello();
    get_ch_str();
    get_tum();
    test_struct();
    test_enum();
    test_array();
    test_condition();
    test_match();
}
