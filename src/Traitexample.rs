pub fn starting_point(){

  
    example();
 
 
 }


trait Display {
    fn format(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

struct Announcement {
    title: String,
    date: String,
}
impl Display for Article {
    fn format(&self) -> String {
        format!("Article: {}\nContent: {}", self.title, self.content)
    }
}

impl Display for Announcement {
    fn format(&self) -> String {
        format!("Announcement: {}\nDate: {}", self.title, self.date)
    }
}


fn show(item: &impl Display) {
    println!("{}", item.format());
}

fn example() {
    let article = Article {
        title: "Rust Traits".to_string(),
        content: "Traits in Rust allow for polymorphism...".to_string(),
    };

    let announcement = Announcement {
        title: "Maintenance Window".to_string(),
        date: "2024-03-10".to_string(),
    };

    show(&article);
    show(&announcement);
}
