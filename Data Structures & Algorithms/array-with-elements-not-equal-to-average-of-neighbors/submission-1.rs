macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    };
}

macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2
    }
    calculate! {
        eval (1 + 2) * (3 / 1)
    }

    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
    };
    println!("{:?}", scores);
}