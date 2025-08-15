use crate::amo::data_types::pipeline::Funnel;
use std::io::BufRead;

pub fn read_project(projects: &[&str]) -> String {
    let mut res = 0;

    while res < 1 {
        println!("Введите номер варианта для выбора проекта");
        for (idx, project) in projects.iter().enumerate() {
            println!("{}. {}", idx + 1, project);
        }

        let input = read_number();
        if input > 0 && input < projects.len() + 1 {
            res = input;
        }
    }

    let project = &projects[res - 1];

    println!("Вы выбрали проект: {project}");
    project.to_string()
}

pub fn read_funnel(funnels: Vec<Funnel>) -> Funnel {
    let mut res = 0;

    while res < 1 {
        println!("Введите номер варианта для выбора воронки");
        for (idx, f) in funnels.iter().enumerate() {
            println!("{}. {}({})", idx + 1, f.name, f.id);
        }
        let input = read_number();
        if input > 0 && input < funnels.len() + 1 {
            res = input;
        }
    }

    let funnel = funnels[res - 1].clone();

    println!("Вы выбрали воронку: {}", funnel.name);
    funnel
}

fn read_number() -> usize {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap_or(0)
}
