extern crate sonar_qg;

fn main() {
    println!("-- Rating + Coverage/Density");
    plop("AXHBx8zuhJIgoSnA8Nfq");
    println!("-- Rating + Coverage/Density");
    plop("AXHBYMjbhJIgoSnA5MjW");
    println!("-- Nb + Coverage");
    plop("AXEbnWAohJIgoSnALzRU");
    println!("-- Nb + new Coverage");
    plop("AW-kZyhitad4bcXnikLH")
}

fn plop(id: &str) {
    match sonar_qg::yolo(id) {
        Ok(result) => {
            for x in &result.project_status.conditions {
                println!("{}", x.display());
            }
        }
        Err(e) => println!("MY Error: {:#?}", e),
    };
}
