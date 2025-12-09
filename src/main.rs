use std::io;

fn main() {
    println!("Hello, user!");



    let mut name: Vec<String> = Vec::new();
    let mut age: Vec<usize> = Vec::new();
    let mut flag = true;

while flag == true {
    let mut input1 = String::new();
    println!("\nDo you want to collect student data (YES / NO ) ? ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let e  = input1.trim().to_string();

    if e == "NO" {
        println!("\nExiting........");
        flag = false;
        

        for stu in 0..name.len() {
            println!("\nSTUDENT {} details are: ",stu+1);
            println!("Student's name: {} ",name[stu]);
            println!("Student's age: {} ",age[stu]);
        }



    } else if e == "YES" {


        let mut input2 = String::new();
        let mut input3 = String::new();

        println!("\nEnter Student name: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let student_name = input2.trim();
        name.push(student_name.to_string());

        println!("\nEnter Student Age: ");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let student_age:usize = input3.trim().parse().expect("Not a valid Number");
        age.push(student_age);    
    }





 }


}
