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

//虚构的魔法学校编写了新的成绩单生成系统
//在 Rust 中！目前系统仅支持创建成绩单
//学生的成绩以数字表示（例如 1.0 -> 5.5）。但是，那
//学校还发布按字母顺序排列的成绩（A+ -> F-）并且需要能够
//打印两种类型的报告卡！
//
//在 struct ReportCard 和 impl 块中进行必要的代码更改
//支持按字母顺序排列的报告卡。将第二次测试的成绩更改为
//“A+”表示您的更改允许按字母顺序评分。

pub trait Grade {
    fn grade(&self) -> String;
}

impl Grade for f32 {
    fn grade(&self) -> String {
        self.floor().to_string()
    }
}

impl Grade for String {
    fn grade(&self) -> String {
        self.to_string()
    }
}

pub struct ReportCard<T: Grade> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: Grade + std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
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
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
