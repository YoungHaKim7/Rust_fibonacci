fn main() {
    let mut i = 0;
    let mut a = 0;
    let mut c = 0;
    let mut b = 1;
    while i < 20 {
        if i > 1 {
            c = b; //1//1/2//3
            b = b + a; //1+0=1/1+1=2////3
            a = c; //1//1//2
            println!(" {}", b);
            i = i + 1;
        } else {
            println!(" {}", i);
            i = i + 1;
        }
    }
}
