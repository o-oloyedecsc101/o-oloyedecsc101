fn main() {
   let name1 ="Oluwalonimi oloyede";
   println!("My name is {}",name1);

   //find and replace
   let name2 = name1.replace("Oluwalonimi","Nimitimi");
   println!("You can also call me {}",name2);
   let faculty = "faculty of Science and technology";

   //find and replace
   let school = faculty.replace("Faculty", "School");
   println!("I am a student of the {}", school );
}
 