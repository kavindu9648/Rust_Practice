// Quection 01
 const pi:f64=3.14159;
 let radius:f64=7.0;

 let area:f64=pi*radius*radius;
 println!("Radius:{},Area:{}",radius,area);

// Quection 02
 let age:i32=22;
 let has_driving_license:bool=true;
 let has_medical_fitness:bool=false;

 if age>=18 && has_driving_license &&! has_medical_fitness{
    println!("Application Accepted");
 }else{
     println!("Application Rejected");
 }

// Quection 03
let day_number:i32=7;
 match day_number{
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6|7 => println!("Weekend"),
    _=>println!("Invalide")
 }

 // Quection 04
 let mut count:i32=10;
while count>=1{
    if count%2==0{
        println!("Count Is Even");
    }else{
    println!("Count Is Odd");
    }
    count-=1;
}
println!("Countdown finished!");

// Quection 05
for i in 1..=30{
    if i%3==0 && i%5==0{
    println!("FizzBuzz");

    }else if i%3==0{
         println!("Fizz");

    }else if i%5==0{
         println!("Bizz");
         
    }else{
         println!("No");
    }
}