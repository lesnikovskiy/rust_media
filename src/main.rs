mod content;

fn main() {
    let audiobook = content::media::Media::Audiobook { title: String::from("An Audiobook") };

    let good_movie = content::media::Media::Movie {
        title: String::from("Good Moview"),
        director: String::from("Director"),
    };

    let bad_book = content::media::Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    let podcast = content::media::Media::Podcast(135);

    let placeholder = content::media::Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = content::catalog::Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let placeholder = content::media::Media::Placeholder;
    let item = catalog.get_by_index(10).unwrap_or(&placeholder);
    println!("Item: {:#?}", item);
}
