use std::io::Write;

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams","","",""];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", "",""];


    let print = vec!("\nLager: {}\nStout: {}\nNon-alcoholic: {}", lager[0],stout[0],non_alcoholic[0]);
    


    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Lager           Stout           Non-alcoholic\n"
        .as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    println!("\nData written to file." );


}