#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String },
    Audiobook {title: String},
    Podcast (u32),
    Placeholder
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(& mut self, media:Media) {
        self.items.push(media);
    }

    // Used custom enum MightHaveAValue
    /*fn get_by_index(&self, index:usize) -> MightHaveAValue {
        if self.items.len() > index {
            MightHaveAValue::ThereIsaValue(&self.items[index])
        } else {
            MightHaveAValue::NoValueAvailable
        }
    }*/

    fn get_by_index(&self, index:usize) -> Option < &Media > {
        if self.items.len() > index {
           Some(&self.items[index])
        } else {
            None
        }
    }
}

//To demonstrate enum Option
/*enum MightHaveAValue<'a> {
    ThereIsaValue(&'a Media),
    NoValueAvailable
}*/

fn print_media(media: Media) {
    println!("{:?}", media);
}

impl Media {
    fn description(&self) -> String {
        /*if let Media::Book {title,author } = self {
            format!("Book: {} {}", title,author)
        } else if let Media::Movie {title,director} = self {
            format!("Movie: {} {}", title,director)
        } else if let Media::Audiobook {title} = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Not a book, audiobook or movie")
        }*/
        match self {
            Media::Book {title, author } => {
                format!("Book: {} {}", title,author)
            },
            Media::Audiobook {title} => {
                format!("Audiobook: {}", title)
            },
            Media::Movie {title, director} => {
                format!("Movie: {} {}", title, director)
            },
            Media::Podcast (value) => {
                format!("Podcast: {}", value)
            },
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Media book"),
    };

    let movie = Media::Movie {
        title:String::from("Jurassic Park"),
        director:String::from("Stephen Spielberg"),
    };

    let book = Media::Book {
        title:String::from("Little Women"),
        author:String::from("John Smith"),
    };

    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    //println!("{}", audiobook.description());
    //println!("{}", movie.description());
    //println!("{}", book.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);
    //print_media(audiobook);
    //print_media(movie);
    //print_media(book);
    //println!("{:#?}", catalog.items.get(100));
    match catalog.items.get(100) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("No Item");
        }
    }

    let item = catalog.get_by_index(0);
    let placeholder = Media::Placeholder;
    println!("{:#?}",item.unwrap_or(&placeholder));
    /*println!("Item: {:#?}", item);
    match catalog.get_by_index(0) {
        Some(value) => {
            println!("ThereIsaValue: {:#?}", value);
        }
        None => {
            println!("NoValueAvailable");
        }
    }

    if let Some(value) = catalog.get_by_index(1) {
        println!("ThereIsaValue: {:#?}", value);
    } else {
        println!("NoValueAvailable");
    }*/

}
