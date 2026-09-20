  // Excersice 02
  let mut number:i32=1;
   if number%2==0{
    println!("{} is even",number);
   }else{
    println!("{} is odd",number);
   }
   number +=7;
   
    if number%2==0{
    println!("{} is even",number);
   }else{
    println!("{} is odd",number);
   }
   
   for i in 0..=100{
    if i%2==0{
        println!("{} is even",i);
    }else{
          println!("{} is odd",i);
    }
   }

  // Excersice 03
  let mut score:i32=80;
   if score>75{
    println!("Grade A");
   }else if 65<=score &&score<=74{
    println!("Grade B");
   }else if 55<=score &&score<=64{
    println!("Grade C");
   }else if 35<score &&score<54{
    println!("Grade B");
   }else{
    println!("Fail");
   }

   // Excersice 04
 let has_id_card:bool=true;
 let knows_password:bool=true;
 let is_account_locked:bool=false;

 if has_id_card && knows_password && !is_account_locked{
    println!("Welcome To System");
 }else{
    println!("Access Deined");
 }