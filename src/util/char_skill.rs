use std::{clone::Clone, default::Default, hash::Hash, ops::Add, time::Duration};

use super::sign_table::SignTableHandle;
/// 技能trait
/// 提供在进场`duration`时间后心情消耗与生产力改变和仓库容量改变
pub trait CharSkill<S, T>
where
    S: SignTableHandle<Item = T>,
    T: Default + Clone + Hash,
    Self: Sized,
{
    /// # TODO 技能启用后的效果
    /// `duration`: 干员进场后经过`duration`时间后对生产单元的影响
    /// `signs`: 符号表，用于全局熟悉影响
    fn get_effect(&mut self, duration: Duration, signs: &mut S) -> SkillEffect<T>;
    
    fn get_during_effect(&mut self, duration: Duration, signs: &mut S) -> SkillEffect<T>
    where
        T: Add<Output = T>,
    {
        let range = 0..duration.as_secs();
        range
            .into_iter()
            .map(|f| self.get_effect(Duration::new(f, 0), signs))
            .reduce(|a, b| a + b)
            .or_else(|| Some(SkillEffect::default()))
            .unwrap()
    }
}
#[derive(Default)]
pub struct SkillEffect<T>
where
    T: Default + Clone,
{
    pub mood_effect: T,
    pub produce_effect: T,
    pub capacity_effect: u32,
}

impl<T> Add for SkillEffect<T>
where
    T: Default + Add<Output = T> + Clone,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SkillEffect {
            mood_effect: self.mood_effect + rhs.mood_effect,
            produce_effect: self.produce_effect + rhs.produce_effect,
            capacity_effect: rhs.capacity_effect,
        }
    }
}
