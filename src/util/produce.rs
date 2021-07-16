use std::hash::Hash;

///# 标准生产单元trait
pub trait ProduceUnit<S>: Sized
where
    S: Hash + Sized + Default + Clone,
{
    /// * 在无任何提升上基建单元基础生产速率
    fn base_speed(lv: u8) -> S;
    ///* 在无任何提升上基建单元仓库容量
    fn base_capacity(lv: u8) -> u32;

    /// * 提供一个闭包对生产速率操作
    /// * 闭包格式：
    /// ```rust
    /// FS: FnMut(&mut S)->Option<()>
    /// ```
    fn operate_speed<FS>(&mut self, op: FS) -> Option<()>
    where
        FS: FnMut(&mut S) -> Option<()>;
    /// * 提供一个闭包对生产速率操作
    /// * 闭包格式：
    /// ```rust
    /// FC: FnMut(&mut u32)->Option<()>
    /// ```
    fn operate_capacity<FC>(&mut self, op: FC) -> Option<()>
    where
        FC: FnMut(&mut u32) -> Option<()>;
}
