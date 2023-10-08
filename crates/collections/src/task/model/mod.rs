use std::collections::HashMap;

///
/// 公司
///
#[derive(Debug)]
pub struct Company {
    ///
    /// 部门
    ///
    org: HashMap<String, Vec<String>>,
}

impl Company {
    ///
    /// 实例化
    ///
    pub fn new() -> Company {
        Company {
            org: HashMap::new(),
        }
    }

    ///
    /// 添加员工
    ///
    pub fn add_employee(&mut self, dep: &str, emp: &str) -> bool {
        let emps = self.org.entry(String::from(dep)).or_insert(Vec::new());

        let emp = String::from(emp);
        if emps.contains(&emp) {
            println!("[Inf] 该部门[{}]已存在员工: {}", dep, emp);
            return false;
        }

        emps.push(emp);
        println!("[Inf] 添加成功, {:?}", self.org);
        true
    }

    ///
    /// 获取部门列表
    ///
    pub fn deps(&self) -> Vec<&String> {
        self.org.keys().collect()
    }

    ///
    /// 获取指定部门员工列表
    ///
    pub fn employees(&self, dep: &String) -> Vec<&String> {
        match self.org.get(dep) {
            Some(emps) => emps.iter().collect(),
            None => Vec::new(),
        }
    }

    ///
    /// 所有部门所有员工
    /// - 升序排序
    /// 
    pub fn show_all(&self) {
        let mut deps = self.deps();
        deps.sort();
        for dep in deps {
            let mut emps = self.employees(dep);
            emps.sort();
            println!("{}: {:?}", dep, emps);
        }

        // println!("{:?}", self);
    }

}
