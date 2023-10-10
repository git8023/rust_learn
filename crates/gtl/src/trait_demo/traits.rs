///
/// 可摘要的
///
pub trait Summarizable {
    ///
    /// 获取摘要
    ///
    fn summary(&self) -> String;

    ///
    /// 默认摘要
    /// 
    fn default_summary(&self) -> String {
        format!("(Read more...)")
    }
}
