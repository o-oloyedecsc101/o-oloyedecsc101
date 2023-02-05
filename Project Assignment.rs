use std::io::Write;	

fn main() {


let mut file = std::fs::File::create("data.txt").expect("create failed");
file.write_all("Lager   stout   Non-Alchoholic\n"
	.as_bytes()).expect("write failed");
file.write_all("33 export   Turbo King   Amstel Malta\n"
	.as_bytes()).expect("write failed");
file.write_all("Goldberg   willams   malta Gold\n"
     .as_bytes()).expect("write failed");
file.write_all("Gulder    {}   Fayrouz\n"
     .as_bytes()).expect("write failed");
file.write_all("Heineken   {}   {}\n"
     .as_bytes()).expect("write failed");
file.write_all("Desperados   legend   maltina\n"
     .as_bytes()).expect("write failed");
file.write_all("Desperados   {}   {}".as_bytes()).expect("write failed");
      
println!("\nData written to file.");

}
