use std::borrow::Cow;

fn main() {
    {
        fn abs_all(input: &mut Cow<[i32]>) {
            for i in 0..input.len() {
                let v = input[i];
                if v < 0 {
                    // Clones into a vector if not already owned.
                    input.to_mut()[i] = -v;
                }
            }
        }

        // No clone occurs beacase `input` doesn't need to be mutated.
        let slice = [0, 1, 3];
        let mut input = Cow::from(&slice[..]);
        abs_all(&mut input);
        println!("{:?}", input);
        match input {
            Cow::Borrowed(_) => println!("slice is borrowed"),
            Cow::Owned(_) => println!("slice is owned"),
        }

        // Clone occurs beacase `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        abs_all(&mut input);
        println!("{:?}", input);
        match input {
            Cow::Borrowed(_) => println!("slice is borrowed"),
            Cow::Owned(_) => println!("slice is owned"),
        }

        // No clone occur because `input` is already owned.
        let mut input = Cow::from(vec![-1, 0, 1]);
        abs_all(&mut input);
        match input {
            Cow::Borrowed(_) => println!("slice is borrowed"),
            Cow::Owned(_) => println!("slice is owned"),
        }
    }

    {
        struct Items<'a, X: 'a>
        where
            [X]: ToOwned<Owned = Vec<X>>,
        {
            values: Cow<'a, [X]>,
        }

        impl<'a, X: Clone + 'a> Items<'a, X>
        where
            [X]: ToOwned<Owned = Vec<X>>,
        {
            fn new(v: Cow<'a, [X]>) -> Self {
                Items { values: v }
            }
        }

        // Creates a container from borrowed values of a slice
        let readonly = [1, 2];
        let borrowed = Items::new((&readonly[..]).into());
        match borrowed {
            Items {
                values: Cow::Borrowed(b),
            } => println!("borrowed {b:?}"),
            _ => panic!("expect borrowed value"),
        }

        let mut clone_on_write = borrowed;
        // Mutates the data from slice into owned vec and pushes a new value on top
        clone_on_write.values.to_mut().push(3);
        println!("clone_on_write = {:?}", clone_on_write.values);

        // The data was mutated. Let's check it out
        match clone_on_write {
            Items {
                values: Cow::Owned(_),
            } => println!("clone_on_write contains owned data"),
            _ => panic!("expect owned data"),
        }
    }
}
