use std::{
    clone::Clone,
    default::Default,
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
    time::Duration,
};

use super::sign_table::SignTableHandle;
/// 技能trait
/// 提供在进场`duration`时间后心情消耗与生产力改变和仓库容量改变
trait CharSkill<'a,S, T>
where
    S: SignTableHandle<T>,
    T: Hash
        + Default
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Clone,
    Self: Sized,
{
    /// # TODO 技能启用后的效果
    /// `duration`: 干员进场后经过`duration`时间后对生产单元的影响
    /// `signs`: 符号表，用于全局熟悉影响
    fn get_effect(&mut self, duration: Duration, signs: &'a mut S)->(T,T,u32);
}
