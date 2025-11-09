use warp::{filters::path::param, reply::{Reply, Response}, Filter};
use std::{collections::HashMap, fs, os::raw, string};

#[tokio::main]
async fn main() {
    let port = 4040;
    println!("port is {}", port);
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let home2 = warp::path("index").map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let galeria = warp::path!("galeria").map(|| warp::reply::html(creategalery()));
    let header = warp::path!("header").map(|| warp::reply::html(fs::read_to_string("header.html").unwrap()));
    let footer = warp::path!("footer").map(|| warp::reply::html(fs::read_to_string("footer.html").unwrap()));
    let shipping = warp::path!("shipping").map(|| warp::reply::html(fs::read_to_string("shipping.html").unwrap()));
    let shippingcss = warp::path("shipping.css").and(warp::fs::file("shipping.css"));
    //let sidebar = warp::path("sidebar.html").and(warp::fs::file("sidebar.html"));
    let style = warp::path("style.css").and(warp::fs::file("style.css"));
    let script = warp::path("script.js").and(warp::fs::file("script.js"));
    let bootstrapmincss = warp::path("bootstrap.min.css").and(warp::fs::file("bootstrap.min.css"));
    let bootstrapminjs = warp::path("bootstrap.min.js").and(warp::fs::file("bootstrap.min.js"));
    let scrolljs = warp::path("scroll-animations.js").and(warp::fs::file("scroll-animations.js"));
    let bootstrapcss = warp::path("bootstrap.css").and(warp::fs::file("bootstrap.css"));
    let bootstrapjs = warp::path("bootstrap.js").and(warp::fs::file("bootstrap.js"));

    let checkout = warp::path("checkout.html").map(|| warp::reply::html(fs::read_to_string("checkout.html").unwrap()));

    let galeria_elemek = warp::path("kep")
    .and(warp::path::param()).and(warp::path::param())
    .and(warp::path::end())
    .map(|festo : String, festmeny : String|{
        let mut html = fs::read_to_string("kep.html").unwrap(); // ures html forma a kepnek
        let data  = fs::read_to_string(format!("kepek/{}/{}/dat.txt",festo, festmeny)).unwrap_or("def\ndef".to_owned());
        let adat : Vec<&str> = data.split("\n").collect(); // 0 a nev 1 a mu cime
        html = html.replace("festmeny", adat[0]);
        //html = html.replace("muvesz", adat[1]);
        html = html.replace("painter", &festo);
        html = html.replace("painting", &festmeny);
        warp::reply::html(html)
        }
    );
    
    let form = warp::path!("form").map(||{warp::reply::html(fs::read_to_string("feltoltes.html").unwrap())});
    let feltoltes = warp::post().and(warp::path!("feltoltes")).and(warp::body::form())
    .map(|body: HashMap<String, String>|{

    });

    let alkotas = warp::path!("alkotas"/String/ String).map(|alkoto: String , alkotas: String|{
        let page = fs::read_to_string("alkotas.html").unwrap();
        let raw  = fs::read_to_string(format!("kepek/{}/{}/dat.txt",alkoto.trim(), alkotas.trim())).unwrap_or("def\ndef".to_owned()); 
        let adat : Vec<&str> = raw.split("\n").collect(); // 0 a nev 1 a mu cime
        warp::reply::html(page
            .replace("painter", &alkoto)
            .replace("painting", &alkotas)
            .replace("alkotas", &adat[0])
            //.replace("alkoto", &adat[1])
        )
    });

    let vetel = warp::path!("buy"/String).map(|alkotas: String|{
        
    });

    let favicon = warp::path("favicon.ico").and(warp::fs::file("favourite.ico"));

    let icons = warp::path("icons")
    .and(warp::fs::dir("menu_pictures/Icons"));

    let sorting = warp::path("sorting")
    .and(warp::fs::dir("menu_pictures/Sorting"));

    let festmenyek = warp::path("festok")
        .and(warp::fs::dir("./festok"));

    let kepek = warp::path("kepek")
        .and(warp::fs::dir("./kepek"));

    let articles = warp::path("articles")
    .and(warp::fs::dir("menu_pictures/Related_Articles"));
    let basket = warp::path("basketimg").and(warp::fs::dir("menu_pictures/basket"));

    let title_icon = warp::path("title-icon.png").and(warp::fs::file("menu_pictures/x-icon/Title-icon.png"));

    let routes = home.or(home2).or(style).or(script).or(festmenyek).or(galeria)
    .or(alkotas).or(form).or(icons).or(sorting).or(bootstrapcss).or(bootstrapjs).or(bootstrapmincss).or(bootstrapminjs).or(articles).or(favicon)
    .or(title_icon).or(galeria_elemek).or(kepek).or(checkout).or(basket).or(header).or(footer).or(scrolljs).or(shipping).or(shippingcss);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}

fn creategalery() -> String{
    let mut rawgaleryhtml = fs::read_to_string("galeria.html").unwrap();
    let mut dirs  = fs::read_dir("kepek/abstract").unwrap();
    let impressionist = fs::read_dir("kepek/impressionism").unwrap();
    //let n = fs::read_dir("kepek/teszt").unwrap().count();
    let mut items = String::new();
    let mut paintings : Vec<std::ffi::OsString> = vec![];
    //let _ = dirs.map(|painting|{paintings.push(painting.unwrap().file_name().to_os_string())});
    //let _ = impressionist.map(|painting|{paintings.push(painting.unwrap().file_name().to_os_string())});
    items += &style(dirs, "abstract");
    items += &style(impressionist, "impressionism");
    //println!("{}", items);
    rawgaleryhtml.replace("galeriaitem", &items)
}

fn style(a: fs::ReadDir, mappa: &str)->String{
    let mut returning = String::new();
    a.for_each(|painting|{
        returning += &format!("
            <div class='col-md-3'>
                <div id='{}'>
                    <script>loadPage('/kep/{}/{}', '{}')</script>
                </div>
            </div>",
            //painting.as_ref().unwrap().file_name().to_str().unwrap(),
            painting.as_ref().unwrap().file_name().to_str().unwrap(),
            mappa,
            painting.as_ref().unwrap().file_name().to_str().unwrap(),
            painting.as_ref().unwrap().file_name().to_str().unwrap());
        //items += &item;
    });
    returning
}
