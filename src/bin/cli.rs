use askama::Template;
use unorder::orders;




#[tokio::main]
async fn main() {
    let templ = Template::render(&orders::Views::New).unwrap();
    println!("{templ}");
}

