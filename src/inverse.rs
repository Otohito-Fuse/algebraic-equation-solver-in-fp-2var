/// 逆元を与える。可逆元の場合は```Some(逆元)```を返し、そうでない場合は```None```を返す
pub trait Inverse
where
    Self: std::marker::Sized,
{
    fn inverse(self) -> Option<Self>;
}
