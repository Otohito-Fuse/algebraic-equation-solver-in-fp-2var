/// 零元を与える
pub trait Zero {
    fn zero() -> Self;
}

/// 単位元を与える
pub trait Identity {
    fn identity() -> Self;
}
