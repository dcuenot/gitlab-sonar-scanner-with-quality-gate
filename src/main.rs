extern crate display_in_gitlab_merge_request;

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
    match display_in_gitlab_merge_request::yolo(id) {
        Ok(result) => {
            for x in &result.project_status.conditions {
                println!("{}", x.display());
            }
        }
        Err(e) => println!("MY Error: {:#?}", e),
    };
}
