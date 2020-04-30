extern crate sonar_qg;

fn main() {
    println!();
    println!("-- Rating + Coverage/Density");
    plop("AXHBx8zuhJIgoSnA8Nfq");
    println!();

    println!("-- Rating + Coverage/Density");
    plop("AXHBYMjbhJIgoSnA5MjW");
    println!();

    plop("AXHBx8zuhJIgoSnA8Nfq");
    println!();
    plop("AXEbnWAohJIgoSnALzRU");
    println!();

    println!("-- Nb + new Coverage");
    plop("AW-kZyhitad4bcXnikLH")
}

fn plop(id: &str) {
    match sonar_qg::yolo(id) {
        Ok(result) => {
            // println!("{:#?}", &result);
            println!("{}", result.display());
        }
        Err(e) => println!("MY Error: {:#?}", e),
    };
}
