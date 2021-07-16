use std::{
    hash::Hash,
    ops::Add,
};

///符号表
pub trait SignTableHandle {
    type Item: Hash + Sized + Clone;
    /// 检查某个符号是否存在
    fn sign_exist(&self, key: &str) -> bool;
    /// 获取某个符号
    fn get(&self, key: &str) -> Option<&Self::Item>;
    /// 以可以更改模式获取某个符号
    fn get_mut(&mut self, key: &str) -> Option<&mut Self::Item>;
    ///添加或者覆盖新符号
    fn insert(&mut self, key: &str, data: Self::Item) -> Option<()>;
    ///移除指定符号
    fn remove(&mut self, key: &str) -> Option<()>;
    ///对某个符号递增加，如果不存在，创建默认值再执行操作
    fn and_asign(&mut self, key: &str, data: Self::Item) -> Option<()>
    where
        Self::Item: Add<Output = Self::Item> + Default,
    {
        if self.sign_exist(key) {
            let mut temp = self.get(key).unwrap().clone();
            temp = data + temp;
            self.insert(key, temp)
        } else {
            let mut temp = Self::Item::default();
            temp = temp + data;
            self.insert(key, temp)
        }
    }
}

