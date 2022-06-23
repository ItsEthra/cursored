use cursored::Cursored;

fn main() {
    let mut c = Cursored::new(vec![-62i8 as u8, 0, 5, 4]);
    c.advance(2);
    dbg!(c.lasting());
}