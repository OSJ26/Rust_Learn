mod Rusty;
use Rusty::is_odd_even;
fn main() {
    let number = 42;
    let check_number = is_odd_even(number);

    if check_number{
        print!("{} is Even",number);
    }
    else {
        print!("{} is Odd",number);
    }
}
