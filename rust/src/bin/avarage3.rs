use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let grades: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Error parsing grade"))
        .collect();

    let media: f64 = (grades[0] * 2.0 + grades[1] * 3.0 + grades[2] * 4.0 + grades[3] * 1.0) / 10.0;

    println!("Media: {:.1}", media);

    if media >= 7.0 {
        println!("Aluno aprovado.");
        return;
    } else if media < 5.0 {
        println!("Aluno reprovado.");
        return;
    } else {
        println!("Aluno em exame.");

        let mut exam_grade = String::new();

        io::stdin()
            .read_line(&mut exam_grade)
            .expect("Failed to read exam grade");

        let exam_grade: f64 = exam_grade.trim().parse().expect("Error parsing exam grade");
        println!("Nota do exame: {:.1}", exam_grade);

        let final_grade = (media + exam_grade) / 2.0;

        if final_grade >= 5.0 {
            println!("Aluno aprovado.");
            println!("Media final: {:.1}", final_grade);
            return;
        } else {
            println!("Aluno reprovado.");
            println!("Media final: {:.1}", final_grade);
            return;
        }

        

    }
}
