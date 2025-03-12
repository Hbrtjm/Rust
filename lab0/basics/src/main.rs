fn is_prime(a: i32) -> bool
{
    if a < 2
    {
        return false;
    }
    let mut i: i32 = 2;
    loop{
        if i*i > a
        {
            break;
        }
        if a % i == 0
        {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    let numbers = [2, 1, 3, 7, 4, 2, 0, 6, 9];
    for num in numbers
    {
        if is_prime(num)
        {
            println!("{}", num);
        }
    }
}
