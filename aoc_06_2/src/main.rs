use std::{env, fs};

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();
    let line = input.first().unwrap();

    let iter1 = line.chars();
    let iter2 = line.chars().skip(1);
    let iter3 = line.chars().skip(2);
    let iter4 = line.chars().skip(3);
    let iter5 = line.chars().skip(4);
    let iter6 = line.chars().skip(5);
    let iter7 = line.chars().skip(6);
    let iter8 = line.chars().skip(7);
    let iter9 = line.chars().skip(8);
    let iter10 = line.chars().skip(9);
    let iter11 = line.chars().skip(10);
    let iter12 = line.chars().skip(11);
    let iter13 = line.chars().skip(12);
    let iter14 = line.chars().skip(13);

    let (pos, _) = iter1.zip(iter2)
                            .zip(iter3)
                            .zip(iter4)
                            .zip(iter5)
                            .zip(iter6)
                            .zip(iter7)
                            .zip(iter8)
                            .zip(iter9)
                            .zip(iter10)
                            .zip(iter11)
                            .zip(iter12)
                            .zip(iter13)
                            .zip(iter14)
                            .map(|(((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n)| (a, b, c, d, e, f, g, h, i, j, k, l, m, n))
                            .enumerate()
                            .find(|(_, (a,b,c,d, e, f,g,h,i,j,k,l,m,n))| {
                                a != b && a != c && a != d && a != e && a != f && a != g && a != h && a != i && a != j && a != k && a != l && a != m && a != n &&
                                b != c && b != d && b != e && b != f && b != g && b != h && b != i && b != j && b != k && b != l && b != m && b != n &&
                                c != d && c != e && c != f && c != g && c != h && c != i && c != j && c != k && c != l && c != m && c != n &&
                                d != e && d != f && d != g && d != h && d != i && d != j && d != k && d != l && d != m && d != n &&
                                e != f && e != g && e != h && e != i && e != j && e != k && e != l && e != m && e != n &&
                                f != g && f != h && f != i && f != j && f != k && f != l && f != m && f != n &&
                                g != h && g != i && g != j && g != k && g != l && g != m && g != n &&
                                h != i && h != j && h != k && h != l && h != m && h != n &&
                                i != j && i != k && i != l && i != m && i != n &&
                                j != k && j != l && j != m && j != n &&
                                k != l && k != m && k != n &&
                                l != m && l != n &&
                                m != n
                            })
                            .unwrap();
    println!("position: {}", pos+14);
}
