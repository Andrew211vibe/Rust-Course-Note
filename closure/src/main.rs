// struct Cache<T, E>
// where
//     T: Fn(E) -> E,
//     E: Copy
// {
//     query: T,
//     value: Option<E>,
// }

// impl<T, E> Cache<T, E> 
// where
//     T: Fn(E) -> E,
//     E: Copy
// {
//     fn new(query: T) -> Cache<T, E> {
//         Cache {
//             query,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: E) -> E {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// fn main() {
//     println!("Hello, world!");
// }

// #[test]
// fn call_with_different_values() {
//     let mut c = Cache::new(|a| a);

//     let v1 = c.value(1);
//     let v2 = c.value(2);

//     assert_eq!(v2, 1);
// }

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}