fn main() {
     

     let n1 = "Electrical".to_string();
     let  _n2 = "Electronic".to_string();
     let  _n3 = "Engineering".to_string();


     // About Electrical/Electronic
     println!("\nThe {} is informed by the aspiration to train electrical/Electronic engineering professionals in the areas of design, building and maintenance of electrical control systems,", n1);


     let w1 = "Computer".to_string();
     let w2 = "Science".to_string();
     let w3 = w1 + &w2; //w2 reference is passed
     println!();
     println!("{} is aimed at developing competent, creative,innovative, enterpenurial and eithically-minded persons,capable of creating value in the diverse fields of Computer Science",w3);
}
