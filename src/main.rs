use warp::{filters::path::param, Filter};
use std::fs;

#[tokio::main]
async fn main() {
    let port = 4040;
    println!("port is {}", port);
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let home2 = warp::path("index").map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    //let sidebar = warp::path("sidebar.html").and(warp::fs::file("sidebar.html"));
    let style = warp::path("style.css").and(warp::fs::file("style.css"));
    let script = warp::path("script.js").and(warp::fs::file("script.js"));
    let asd = warp::path("asdf")
    .and(warp::path::param()).and(warp::path::param())
    .and(warp::path::end())
    .map(|festo : String, festmeny : String|{
        let mut html = fs::read_to_string("kep.html").unwrap();
        html = html.replace("painter", &festo);
        html = html.replace("painting",&festmeny);
        warp::reply::html(html)
        });
    let kep = warp::path!("kep").map(||{
        let mut html = fs::read_to_string("kep.html").unwrap();
        html = html.replace("painter", "teszt");
        html = html.replace("painting", "festmeny.jpg");
        warp::reply::html(html)
    });

    let festmenyek = warp::path("festok")
        .and(warp::fs::dir("./festok"));

    let routes = home.or(home2).or(style).or(script).or(kep).or(festmenyek)
    .or(asd);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}
