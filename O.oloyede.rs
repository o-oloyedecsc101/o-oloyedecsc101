
fn main(){

    let dept_arr:[&str]  =  ["Stategy Consulting","Corporate and growth strategy","Transaction and strategy execution","restructuring and turnaround strategy","Industry stategy","Digital business building","Commercial strategy"];
        println!("array is {:?}", dept_arr);

    for stategy by Ey Parthenon in dept_arr.iter(){
        println!("stategy by Ey Parthenon has :|{}|departments",stategy by Ey Parthenon);
    }

    let  Consulting_arr:[&str]  =  ["Analytics Consulting services","Coostumer services","cybrsecurity stategy risk complimeance and resilence","Digitial transformsation","Risk Consulting services","Supply chain and Operation","Technology transformation"];
        println!("array is {:?}",Consulting_arr);

    for Constulting in Consulting_arr.iter(){
        println!("Consulting has :|{}|departments",Consulting_arr);
    }
    let People and Workforce_arr:[&str]  =  ["Change managment and expericence","HR transformation","Intergrated workforce mobility","Learning and developnment consulting","WORKFORCE Analytics","Recognition and reward advisory","People and workforce"];
        println!("array is {:?}", people and workforce_arr);

    for People and workforce  in people and workforce_arr.iter(){
        println!("workforce :|{}|departments",People in work);
    }
    let Tax_arr:[&str]  =  ["Tax planning","Tax function operatin","Tax policy and controversy","Global trade","Tax accounting","Tax compliance","Transcstin tax"];
        println!("array is {:?}", Tax_arr);

    for Tax in Tax_arr.iter(){
        println!("Tax has :|{}|departments",Tax);

    }
    let Transcation and corporate finance_arr:[&str]  =  ["Cooperate finance","Divestments and carve outs ","ESG services","Advisory"," INTEGRATION","Technology and tools","Advanced analytics"];
        println!("array is {:?}", Transcation and corporate finance_arr);

    for Transcation and corporate finance in Transcation and corporate finance_arr.iter(){
        println!("Transcation and corporate finance has :|{}|departments",Transcation and corporate finance);


    }


    use::std::io::Write;

    let mut file = std::fs::File::create("Staff.txt").expect("create failed");
file.write_all("Name | department | Qualification |code\n"
    .as_bytes()).expect("write failed");
file.write_all("Aigbona juilet | Consulting | B.SC |7\n"
    .as_bytes()).expect("write failed");
file.write_all("Ehis Ero | strategy | M.sc. |9\n"
     .as_bytes()).expect("write failed");
file.write_all("Adamu Sagamu | Tax | B.Sc. |8\n"
     .as_bytes()).expect("write failed");
file.write_all("Akpevwe illoka | Assurance | HND |7\n"
     .as_bytes()).expect("write failed");
file.write_all("Maria Akinsola | Corporate finance | M.sc |9\n"
     .as_bytes()).expect("write failed");
file.write_all("Gbenga Daniels | Workforce | HND |8\n".as_bytes()).expect("write failed");
      
println!("\nData written to file.");

}



    















    

}

