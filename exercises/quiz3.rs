// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

//为泛型 G 添加 std::fmt::Display 特征约束 —— 
//因为 format! 宏需要用到 Display 特征来将类型转为字符串，确保数字（f32）和字符串（&str/String）都能被正确格式化。
use std::fmt::Display;
//将 grade 字段的固定类型 f32 改为泛型类型 G，让结构体能存储任意类型的成绩（只要该类型满足 “可格式化输出” 的约束）。
pub struct ReportCard<G:Display> {
    pub grade: G,
    pub student_name: String,
    pub student_age: u8,
}

//泛型参数必须先声明后使用
//泛型结构体实现方法时的强制语法，
//核心原因是要让编译器明确：“我们正在为带有泛型参数 G（且 G 满足 Display 约束）的 ReportCard 结构体实现方法”
impl<G:Display> ReportCard<G> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
