use rand::{thread_rng, Rng};
use std::io::stdin;

fn print_random_ascii(n: usize) {
    let mut tmp = 0;
    while tmp != n {
        print!("{}", thread_rng().gen_range(33u8, 127) as char);
        tmp += 1;
    }
    print!("\n");
}

trait KtStd {
    fn let_owned<R>(self, block: fn(Self) -> R) -> R where Self: Sized {
        block(self)
    }

    fn also_mut(&mut self, block: fn(&mut Self)) -> &mut Self {
        block(self);
        self
    }
}

impl<T> KtStd for T {}

fn main() {
    loop {
        println!("Please input the size you want to generate:");
        String::new()
            .also_mut(|s| { stdin().read_line(s).unwrap(); })
            .trim_end().parse::<usize>().unwrap()
            .let_owned(|x| print_random_ascii(x));
        print!("\n");
    }
}
