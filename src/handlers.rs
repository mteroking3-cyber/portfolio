use actix_web::{get, web, HttpResponse};
use serde::{Serialize};
use tera::Tera;

// Remove the query! macro and use query instead
#[derive(Serialize)]
struct PortfolioData {
    name: String,
    title: String,
    about: String,
    skills: Vec<String>,
    projects: Vec<Project>,
    contact_email: String,
    contact_number: String,
    
}

#[derive(Serialize)]
struct Project {
    name: String,
    description: String,
    technologies: Vec<String>,
    github_url: Option<String>,
    live_url: Option<String>,
}


#[get("/")]
pub async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let portfolio_data = PortfolioData {
        name: "Keith Mutero".to_string(),
        title: "Full Stack Developer".to_string(),
        about: "I’m a Full Stack Developer who builds high-performance, production-grade systems. I specialize in Rust-based and Python-based backends, scalable APIs, and modern frontends, turning complex requirements into clean, efficient, and reliable software. I don’t just write code I engineer solutions that last.".to_string(),
        skills: vec![
            "Docker".to_string(),
            "React".to_string(),
            "REDIS".to_string(),
            "Splint".to_string(),
            "PostgreSQL".to_string(),
            "Actix Web".to_string(),
            "Django".to_string(),
            "Rust".to_string(),
            "Kivy".to_string(),
            "Python".to_string(),
            "KivyMD".to_string(),
            
        ],
        projects: vec![
            Project {
                name: "Futuristic Portfolio Web App".to_string(),
                description: "A futuristic web application built with Rust and modern frontend technologies.".to_string(),
                technologies: vec!["Rust".to_string(), "Tera".to_string(), "WASM".to_string(), "Actix Web".to_string(), "CSS".to_string(), "JS".to_string()],
                github_url: Some("https://github.com/mteroking3-cyber/portfolio".to_string()),
                live_url: None,
            },
            Project {
                name: "Kivy Lazy Loading Template".to_string(),
                description: "A modern kivymd template with all the Optimisation for beginner Android Application Developers utilising Python 13+, integrated with Cython for Optimisations.".to_string(),
                technologies: vec!["Python".to_string(), "Kivy".to_string(), "KivyMD".to_string(), "Cython".to_string()],
                github_url: Some("https://github.com/mteroking3-cyber/Kivy-Lazy-Loading-Template".to_string()),
                live_url: None,
            },
            Project {
                name: "Mobile School Enrolment application".to_string(),
                description: "A mobile application that digitizes the school enrolment process, enabling students and parents to register online while allowing administrators to manage records efficiently. Designed to reduce paperwork, improve accuracy, and streamline school administration.".to_string(),
                technologies: vec!["Python".to_string(), "KivyMD".to_string(),"Kivy".to_string(), "FastAPI".to_string(), "SqlAlchemy".to_string()],
                github_url: Some("https://github.com/mteroking3-cyber/enrollment".to_string()),
                live_url: None,
            },
        ],
        contact_email: "mteroking3@icloud.com".to_string(),
        contact_number: "+263717794958".to_string(),
    };

    let mut ctx = tera::Context::new();
    ctx.insert("portfolio", &portfolio_data);

    match tmpl.render("index.html.tera", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}
