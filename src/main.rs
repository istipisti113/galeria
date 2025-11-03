use warp::{filters::path::param, reply::{Reply, Response}, Filter};
use std::{collections::HashMap, fs, string};

#[tokio::main]
async fn main() {
    let port = 4040;
    println!("port is {}", port);
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let home2 = warp::path("index").map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let galeria = warp::path!("galeria").map(|| warp::reply::html(fs::read_to_string("galeria.html").unwrap()));
    //let sidebar = warp::path("sidebar.html").and(warp::fs::file("sidebar.html"));
    let style = warp::path("style.css").and(warp::fs::file("style.css"));
    let script = warp::path("script.js").and(warp::fs::file("script.js"));
    let bootstrapmincss = warp::path("bootstrap.min.css").and(warp::fs::file("bootstrap.min.css"));
    let bootstrapminjs = warp::path("bootstrap.min.js").and(warp::fs::file("bootstrap.min.js"));
    let bootstrapcss = warp::path("bootstrap.css").and(warp::fs::file("bootstrap.css"));
    let bootstrapjs = warp::path("bootstrap.js").and(warp::fs::file("bootstrap.js"));
    let asd = warp::path("kep")
    .and(warp::path::param()).and(warp::path::param())
    .and(warp::path::end())
    .map(|festo : String, festmeny : String|{
        let mut html = fs::read_to_string("kep.html").unwrap(); // ures html forma a kepnek
        let data  = fs::read_to_string(format!("festok/{}/{}/dat.txt",festo, festmeny)).unwrap(); 
        let adat : Vec<&str> = data.split("\n").collect(); // 0 a nev 1 a mu cime
        html = html.replace("painter", &festo);
        html = html.replace("painting", &festmeny);
        html = html.replace("festmeny", adat[1]);
        html = html.replace("muvesz", adat[0]);
        warp::reply::html(html)
        }
    );
    
    let form = warp::path!("form").map(||{warp::reply::html(fs::read_to_string("feltoltes.html").unwrap())});
    let feltoltes = warp::post().and(warp::path!("feltoltes")).and(warp::body::form())
    .map(|body: HashMap<String, String>|{

    });

    let alkotas = warp::path!("alkotas"/String/ String).map(|alkoto: String , alkotas: String|{
        let page = fs::read_to_string("alkotas.html").unwrap();
        let raw  = fs::read_to_string(format!("festok/{}/{}/dat.txt",alkoto, alkotas)).unwrap(); 
        let adat : Vec<&str> = raw.split("\n").collect(); // 0 a nev 1 a mu cime
        warp::reply::html(page
            .replace("painter", &alkoto)
            .replace("painting", &alkotas)
            .replace("alkotas", &adat[1])
            .replace("alkoto", &adat[0])
        )
    });

    let icons = warp::path("icons")
    .and(warp::fs::dir("menu_pictures/Icons"));

    let sorting = warp::path("sorting")
    .and(warp::fs::dir("menu_pictures/Sorting"));

    let festmenyek = warp::path("festok")
        .and(warp::fs::dir("./festok"));

    let articles = warp::path("articles")
    .and(warp::fs::dir("menu_pictures/Related_Articles"));

    let routes = home.or(home2).or(style).or(script).or(festmenyek).or(galeria)
    .or(alkotas).or(form).or(icons).or(sorting).or(bootstrapcss).or(bootstrapjs).or(bootstrapmincss).or(bootstrapminjs).or(articles);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}
