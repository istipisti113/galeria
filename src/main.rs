use warp::{Filter};
use std::fs;

#[tokio::main]
async fn main() {
    let port = 4040;
    println!("port is {}", port);
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let home2 = warp::path("index").map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let sidebar = warp::path("sidebar.html").map(|| warp::reply::html(fs::read_to_string("sidebar.html").unwrap()));
    let elerhetosegek = warp::path("elerhetosegek").map(|| warp::reply::html(fs::read_to_string("elerhetosegek.html").unwrap()));
    let gyermekvedelem = warp::path("gyermekvedelem").map(|| warp::reply::html(fs::read_to_string("gyermekvedelem.html").unwrap()));
    let munkatarsaink = warp::path("munkatarsaink").map(|| warp::reply::html(fs::read_to_string("munkatarsaink.html").unwrap()));
    //let sidebar = warp::path("sidebar.html").and(warp::fs::file("sidebar.html"));
    let style = warp::path("style.css").and(warp::fs::file("style.css"));
    let gyermekvedelemcss = warp::path("gyermekvedelem.css").and(warp::fs::file("gyermekvedelem.css"));
    let munkatarsainkcss = warp::path("munkatarsaink.css").and(warp::fs::file("munkatarsaink.css"));
    let script = warp::path("script.js").and(warp::fs::file("script.js"));

    let golyak = warp::path("golyak2024.jpg").and(warp::fs::file("golyak2024.jpg"));
    let logo = warp::path("logo.png").and(warp::fs::file("logo.png"));
    let logo150 = warp::path("logo150.png").and(warp::fs::file("logo150.png"));
    let kretalogo = warp::path("kreta_logo.png").and(warp::fs::file("kreta_logo.png"));
    let gepiras = warp::path("gepiras.jpeg").and(warp::fs::file("gepiras.jpeg"));
    let tanarok = warp::path("img")
        .and(warp::fs::dir("./img"));

    let routes = home.or(home2).or(style).or(script).or(sidebar).or(golyak).or(logo)
        .or(logo150).or(kretalogo).or(gepiras).or(elerhetosegek).or(gyermekvedelem).or(gyermekvedelemcss)
        .or(munkatarsaink).or(munkatarsainkcss).or(tanarok);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}
