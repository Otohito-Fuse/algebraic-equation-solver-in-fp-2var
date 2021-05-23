use crate::identities::{Identity, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// （1変数の）多項式
///
/// coefficientsは係数を並べた配列。i番目がi次の項に対応。
/// 最高次係数がnon-zeroになるようにする（0だけは別）
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Polynomial<T> {
    coefficients: Vec<T>,
    degree: usize,
}

impl<T> Polynomial<T> {
    /// 多項式の次数を返す。簡単のため0の次数も0にしてしまう。
    pub fn deg(&self) -> usize {
        self.degree
    }

    /// 係数環の元から定数（0次多項式）を作る。
    pub fn new_constant(t: T) -> Self {
        Self {
            coefficients: vec![t],
            degree: 0,
        }
    }
}

impl<T: Zero + Eq> Polynomial<T> {
    /// 多項式の次数を返す。こっちは0の次数はNoneにする。
    pub fn strict_deg(&self) -> Option<usize> {
        if self.coefficients.len() == 1 && self.coefficients[0] == T::zero() {
            None
        } else {
            Some(self.degree)
        }
    }
}

impl<T: Zero + Eq + Copy> Polynomial<T> {
    /// ```T```型のデータからなるベクトルを受け取り、それを係数としてもつ多項式を作る。
    /// 最高次係数がnon-zeroになるようにするのでTには```Zero```や```Eq```を要求。
    pub fn new(v: &Vec<T>) -> Self {
        let mut f: Vec<T> = Vec::new();
        if v.len() == 0 {
            f.push(T::zero());
            Self {
                coefficients: f,
                degree: 0,
            }
        } else {
            f.push(v[0]);
            for &t in &v[1..] {
                f.push(t.clone());
            }
            while f.len() > 1 {
                if let Some(&t) = f.last() {
                    if t == T::zero() {
                        f.pop();
                    } else {
                        break;
                    }
                }
            }
            let d = f.len() - 1;
            Self {
                coefficients: f,
                degree: d,
            }
        }
    }
}

impl<T: Zero + Identity + Mul<Output = T> + AddAssign + Copy + Eq> Polynomial<T> {
    /// 微分（derivative）を求める関数。
    pub fn derivative(f: &Self) -> Self {
        let mut integer = T::identity(); // 1,2,3,...に相当する元を作るために用意
        let mut v = Vec::<T>::new();
        for &c in &f.coefficients[1..] {
            v.push(c * integer);
            integer += T::identity();
        }
        Polynomial::new(&v)
    }
}

impl<T: Zero + Identity + Mul<Output = T> + MulAssign + AddAssign + Copy + Eq> Polynomial<T> {
    /// 多項式に代入する。
    pub fn evaluate(f: &Self, t: T) -> T {
        let mut t_pow = T::identity();
        let mut ans = T::zero();
        for &c in &f.coefficients {
            ans += c * t_pow;
            t_pow *= t;
        }
        ans
    }
}

/// ```println!```などで見やすく表示させるため、```Display```トレイトを実装。
impl<T: fmt::Display + Zero + Identity + Eq> fmt::Display for Polynomial<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::new();
        let mut flag = false;
        if self.degree == 0 || self.coefficients[0] != T::zero() {
            s.push_str(&self.coefficients[0].to_string())
        } else {
            flag = true;
        }
        if self.degree > 0 && self.coefficients[1] != T::zero() {
            if !flag {
                s.push_str(&" + ");
            } else {
                flag = false;
            }
            if self.coefficients[1] != T::identity() {
                s.push_str(&self.coefficients[1].to_string());
            }
            s.push_str(&"x");
        }
        if self.degree > 1 {
            for i in 2..=self.degree {
                if self.coefficients[i] == T::zero() {
                    continue;
                }
                if !flag {
                    s.push_str(&" + ");
                } else {
                    flag = false;
                }
                if self.coefficients[i] != T::identity() {
                    s.push_str(&self.coefficients[i].to_string());
                }
                s.push_str(&"x^");
                s.push_str(&i.to_string());
            }
        }
        write!(f, "{}", s)
    }
}

impl<T: fmt::Display + Zero + Identity + Eq> Polynomial<T> {
    /// xの多項式としての表示
    pub fn print_f_of_x(&self) -> String {
        let mut s: String = String::new();
        let mut flag = false;
        if self.degree == 0 || self.coefficients[0] != T::zero() {
            s.push_str(&self.coefficients[0].to_string())
        } else {
            flag = true;
        }
        if self.degree > 0 && self.coefficients[1] != T::zero() {
            if !flag {
                s.push_str(&" + ");
            } else {
                flag = false;
            }
            if self.coefficients[1] != T::identity() {
                s.push_str(&self.coefficients[1].to_string());
            }
            s.push_str(&"x");
        }
        if self.degree > 1 {
            for i in 2..=self.degree {
                if self.coefficients[i] == T::zero() {
                    continue;
                }
                if !flag {
                    s.push_str(&" + ");
                } else {
                    flag = false;
                }
                if self.coefficients[i] != T::identity() {
                    s.push_str(&self.coefficients[i].to_string());
                }
                s.push_str(&"x^");
                s.push_str(&i.to_string());
            }
        }
        s
    }

    /// yの多項式としての表示
    pub fn print_f_of_y(&self) -> String {
        let mut s: String = String::new();
        let mut flag = false;
        if self.degree == 0 || self.coefficients[0] != T::zero() {
            s.push_str(&self.coefficients[0].to_string())
        } else {
            flag = true;
        }
        if self.degree > 0 && self.coefficients[1] != T::zero() {
            if !flag {
                s.push_str(&" + ");
            } else {
                flag = false;
            }
            if self.coefficients[1] != T::identity() {
                s.push_str(&self.coefficients[1].to_string());
            }
            s.push_str(&"y");
        }
        if self.degree > 1 {
            for i in 2..=self.degree {
                if self.coefficients[i] == T::zero() {
                    continue;
                }
                if !flag {
                    s.push_str(&" + ");
                } else {
                    flag = false;
                }
                if self.coefficients[i] != T::identity() {
                    s.push_str(&self.coefficients[i].to_string());
                }
                s.push_str(&"y^");
                s.push_str(&i.to_string());
            }
        }
        s
    }
}

/// 足し算の実装
impl<T: Copy + Add<Output = T> + Zero + Eq> Add for Polynomial<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut v: Vec<T> = Vec::new();
        v.push(self.coefficients[0] + rhs.coefficients[0]);
        if self.degree < rhs.degree {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
            for i in (self.degree + 1)..=rhs.degree {
                v.push(rhs.coefficients[i]);
            }
        } else if self.degree > rhs.degree {
            for i in 1..=rhs.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
            for i in (rhs.degree + 1)..=self.degree {
                v.push(self.coefficients[i]);
            }
        } else {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
        }
        Polynomial::new(&v)
    }
}

impl<T: Copy + Add<Output = T> + Zero + Eq> AddAssign for Polynomial<T> {
    fn add_assign(&mut self, rhs: Self) {
        let mut v: Vec<T> = Vec::new();
        v.push(self.coefficients[0] + rhs.coefficients[0]);
        if self.degree < rhs.degree {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
            for i in (self.degree + 1)..=rhs.degree {
                v.push(rhs.coefficients[i]);
            }
        } else if self.degree > rhs.degree {
            for i in 1..=rhs.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
            for i in (rhs.degree + 1)..=self.degree {
                v.push(self.coefficients[i]);
            }
        } else {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
        }
        *self = Polynomial::new(&v)
    }
}

/// 引き算の実装
impl<T: Copy + Sub<Output = T> + Zero + Eq> Sub for Polynomial<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut v: Vec<T> = Vec::new();
        v.push(self.coefficients[0] - rhs.coefficients[0]);
        if self.degree < rhs.degree {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
            for i in (self.degree + 1)..=rhs.degree {
                v.push(T::zero() - rhs.coefficients[i]);
            }
        } else if self.degree > rhs.degree {
            for i in 1..=rhs.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
            for i in (rhs.degree + 1)..=self.degree {
                v.push(self.coefficients[i]);
            }
        } else {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
        }
        Polynomial::new(&v)
    }
}

impl<T: Copy + Sub<Output = T> + Zero + Eq> SubAssign for Polynomial<T> {
    fn sub_assign(&mut self, rhs: Self) {
        let mut v: Vec<T> = Vec::new();
        v.push(self.coefficients[0] - rhs.coefficients[0]);
        if self.degree < rhs.degree {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
            for i in (self.degree + 1)..=rhs.degree {
                v.push(T::zero() - rhs.coefficients[i]);
            }
        } else if self.degree > rhs.degree {
            for i in 1..=rhs.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
            for i in (rhs.degree + 1)..=self.degree {
                v.push(self.coefficients[i]);
            }
        } else {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
        }
        *self = Polynomial::new(&v)
    }
}

/// 掛け算の実装
impl<T: Copy + Add<Output = T> + AddAssign<T> + Mul<Output = T> + Zero + Eq> Mul for Polynomial<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut v: Vec<T> = vec![T::zero(); self.degree * rhs.degree + 1];
        for i in 0..=(self.degree * rhs.degree) {
            for j in 0..=i {
                if i - j <= rhs.degree && j <= self.degree {
                    v[i] += self.coefficients[j] * rhs.coefficients[i - j];
                }
            }
        }
        Polynomial::new(&v)
    }
}

impl<T: Copy + Add<Output = T> + AddAssign<T> + Mul<Output = T> + Zero + Eq> MulAssign
    for Polynomial<T>
{
    fn mul_assign(&mut self, rhs: Self) {
        let mut v: Vec<T> = vec![T::zero(); self.degree * rhs.degree + 1];
        for i in 0..=(self.degree * rhs.degree) {
            for j in 0..=i {
                if i - j <= rhs.degree && j <= self.degree {
                    v[i] += self.coefficients[j] * rhs.coefficients[i - j];
                }
            }
        }
        *self = Polynomial::new(&v)
    }
}

/// unary negation の実装
impl<T: Zero + Eq + Copy + Neg<Output = T>> Neg for Polynomial<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut v: Vec<T> = Vec::new();
        for c in self.coefficients.clone() {
            v.push(-c);
        }
        Polynomial::new(&v)
    }
}

/// ```Zero```の実装
impl<T: Zero + Copy> Zero for Polynomial<T> {
    fn zero() -> Self {
        Self {
            coefficients: vec![T::zero(); 1],
            degree: 0,
        }
    }
}

/// ```Identity```の実装
impl<T: Identity + Copy> Identity for Polynomial<T> {
    fn identity() -> Self {
        Self {
            coefficients: vec![T::identity(); 1],
            degree: 0,
        }
    }
}
