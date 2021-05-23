const P: u64 = 5; // ここを適宜書き換える

pub mod identities;
pub mod inverse;
pub mod modint;
pub mod polynomial;
pub mod solution_set;

use crate::identities::Zero;
use crate::modint::ModInt;
use crate::polynomial::Polynomial;
use crate::solution_set::SolutionSet;

use std::collections::HashSet;

fn main() {
    println!("mod {} での f(x) = g(y) の形の方程式の解を求めます。", P);

    if !is_prime(P) {
        println!("注：{}は素数ではありません。", P);
    }

    println!("fの次数を入力");

    // 解く方程式の左辺の次数
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    // usize型に変換
    let n: usize = n.trim().parse().ok().unwrap();

    let mut v: Vec<ModInt<P>> = vec![ModInt::<P>::zero(); n + 1];

    for i in 0..=n {
        println!("{}次の係数を入力", i);
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).ok();
        // u64型に変換
        let a: u64 = a.trim().parse().ok().unwrap();
        v[i] = ModInt::<P>::new(a);
    }

    let f: Polynomial<ModInt<P>> = Polynomial::new(&v);

    println!("gの次数を入力");

    // 解く方程式の左辺の次数
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    // usize型に変換
    let n: usize = n.trim().parse().ok().unwrap();

    let mut w: Vec<ModInt<P>> = vec![ModInt::<P>::zero(); n + 1];

    for i in 0..=n {
        println!("{}次の係数を入力", i);
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).ok();
        // u64型に変換
        let a: u64 = a.trim().parse().ok().unwrap();
        w[i] = ModInt::<P>::new(a);
    }

    let g: Polynomial<ModInt<P>> = Polynomial::new(&w);

    let s: SolutionSet<(ModInt<P>, ModInt<P>)> = solve_equation(&f, &g);

    println!(
        "方程式 {} = {} の解の集合は",
        f.print_f_of_x(),
        g.print_f_of_y()
    );
    print_solutions(&s);
    println!("です。");
}

/// 素数判定
fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    if n == 0 || n == 1 {
        return false;
    }
    for i in 0..n {
        if n != 3 + 2 * i && n % (3 + 2 * i) == 0 {
            return false;
        }
        if (3 + 2 * i) * (3 + 2 * i) >= n {
            break;
        }
    }
    true
}

/// 方程式の解を全探索
fn solve_equation(
    f: &Polynomial<ModInt<P>>,
    g: &Polynomial<ModInt<P>>,
) -> SolutionSet<(ModInt<P>, ModInt<P>)> {
    let mut s: HashSet<(ModInt<P>, ModInt<P>)> = HashSet::new();
    for i in 0..P {
        for j in 0..P {
            if Polynomial::evaluate(&f, ModInt::<P>::new(i))
                == Polynomial::evaluate(&g, ModInt::<P>::new(j))
            {
                s.insert((ModInt::<P>::new(i), ModInt::<P>::new(j)));
            }
        }
    }
    SolutionSet::new(s)
}

fn print_solutions(ss: &SolutionSet<(ModInt<P>, ModInt<P>)>) {
    let mut s: String = String::new();
    if ss.size() == 0 {
        s.push_str(&"{ }");
    } else {
        s.push_str(&"{");
        for (x, y) in &ss.unwrap() {
            s.push_str(&"(");
            s.push_str(&format!("{}", x).to_string());
            s.push_str(&", ");
            s.push_str(&format!("{}", y).to_string());
            s.push_str(&")");
            s.push_str(&", ");
        }
        s.pop();
        s.pop();
        s.push_str(&"}");
    }
    println!("{}", s);
}
