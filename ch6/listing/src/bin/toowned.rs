use std::borrow::Borrow;

#[derive(Debug)]
struct Any<'a, T> {
    content: &'a T,
}

impl<'a, T> ToOwned for Any<'a, T> {
    type Owned = One<'a, T>;
    fn to_owned(&self) -> Self::Owned {
        One {
            any: Any {
                content: self.content.clone(),
            },
        }
    }
}

#[derive(Debug)]
struct One<'a, T> {
    any: Any<'a, T>,
}

impl<'a, T> Borrow<Any<'a, T>> for One<'a, T> {
    fn borrow(&self) -> &Any<'a, T> {
        &self.any
    }
}

fn main() {
    let content = "hello".to_owned();
    let o = One {
        any: Any { content: &content },
    };
    let b: &Any<String> = o.borrow();
    println!("{:?}", b);

    let a = Any { content: &content };
    let b = a.to_owned();
    println!("{:?}", b);

    let s: Vec<u8> = Vec::from("hello");
    println!("{:?}", s);
}
