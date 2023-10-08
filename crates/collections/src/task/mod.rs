use std::io;

use crate::task::model::Company;

///
/// 使用哈希 map 和 vector，
/// 创建一个文本接口来允许用户向公司的部门中增加员工的名字。
/// 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
/// 接着让用户获取一个部门的所有员工的列表,
/// 或者公司每个部门的所有员工按照字母顺排序的列表
mod model;

fn err(msg: &str) {
    println!("[Err] {} \n\n", msg);
}

pub fn test() {
    let mut company = Company::new();

    // UI交互
    loop {
        // 面板选项
        println!("\n请选择:");
        let opts = vec![
            "1. 添加员工",
            "2. 查询指定部门员工列表",
            "3. 查看所有部门所有员工",
            "0. 退出",
        ];
        for opt in opts {
            println!("{}", opt);
        }

        // 获取标准输入
        let mut opt = String::new();
        if let Err(_) = io::stdin().read_line(&mut opt) {
            err("输入有误, 请重新输入!!");
            continue;
        }

        // 输入转换为数值选项
        let opt: i32 = match opt.trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                err("输入项无法转换为数字, 请重新输入!!");
                continue;
            }
        };

        // 处理选项
        match opt {
            0 => break,
            1 => add_employee(&mut company),
            2 => show_employees_with_dep(&company),
            3 => show_all(&company),
            other => err(&format!("无效选项: {}, 请重新输入!!", other)),
        }
    }

    println!("Bye!");
}

///
/// 显示所有部门, 所有员工
///
fn show_all(company: &Company) {
    println!("\n所有员工:");
    company.show_all();
}

///
/// 显示指定部门员工列表
///
fn show_employees_with_dep(company: &Company) {
    loop {
        println!("\n部门列表(q退出):");
        let deps = company.deps();
        let mut i = 0;
        for dep in &deps {
            i += 1;
            println!("{}. {}", i, *dep);
        }

        // 获取输入
        let mut ipt = String::new();
        io::stdin().read_line(&mut ipt).unwrap();
        if "q" == ipt.trim() {
            break;
        }

        // 转换为索引
        let ipt = match ipt.trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                err("输入项无效, 请重新输入!!");
                continue;
            }
        };
        if 0 > ipt || (deps.len() as i32) < ipt {
            err("输入项无效, 请重新输入!!");
            continue;
        }

        // 列出员工
        let dep = deps[ipt as usize - 1];
        let mut emps = company.employees(dep);
        emps.sort();
        println!("员工列表: {:?}", emps);
    }
}

///
/// 添加员工
///
fn add_employee(company: &mut Company) {
    loop {
        println!("\n请输入(员工,部门), (q)退出:");

        let mut ipt = String::new();
        io::stdin().read_line(&mut ipt).unwrap();
        let ipt = ipt.trim();
        if "q" == ipt {
            break;
        }

        // 切分输入项
        let sp: Vec<_> = ipt.split(",").into_iter().map(|e| e.trim()).collect();
        if 2 != sp.len() {
            err("格式错误, 期望格式: Employ,Department");
            continue;
        }

        // 添加员工
        if company.add_employee(sp[1], sp[0]) {
            break;
        }
    }
}
