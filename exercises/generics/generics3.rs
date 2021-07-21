pub trait Grade {
    fn print(&self) -> String;
}

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl Grade for ReportCard<f32> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}
impl Grade for ReportCard<&str> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

/* //This is better because you can allow any data type for grade that implements 'Display' trait, pay attention as here
use std::fmt::Display;


pub struct ReportCard<T: Display> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl <T: Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

fn main() {
    let report_card = ReportCard {
        grade: 2.1,
        student_name: "Tom Wriggle".to_string(),
        student_age: 12,
    };
    println!(
        "{} == {}",
        report_card.print(),
        "Tom Wriggle (12) - achieved a grade of 2.1"
    );
    let report_card = ReportCard {
        grade: "A+",
        student_name: "Gary Plotter".to_string(),
        student_age: 11,
    };
    println!(
        "{} == {}",
        report_card.print(),
        "Gary Plotter (11) - achieved a grade of A+"
    );
}
 */

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
