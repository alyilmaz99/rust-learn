fn main() {
    println!("Hello, world!");

    let mut _variable = 25;
    let mut _variable2 = 40;
    _variable = 30;

    println!("{}",_variable);
    println!("{0}-{1}",_variable,_variable2);
    println!("{sayi}",sayi=_variable2);
        // asdklzxcj
        /*

        asdasd
        */

    let _number:i32 = 10; // i32 signed unsigned
    let _number2:i64 =10;
    let _float:f64 = 10.23;
    let _bool:bool = true;

    let _numberu:u64 = 10;

    let _strasd = String::from("asdzxc");
    let _str = "asdzzc";

   



    if _variable == 29 {
        println!("1.");
    }else if _variable == 20 {
        println!("2.");
    }else{
        println!("3.");
    }

    loop {
      if _variable > 15{
          _variable -= 1;
          println!("{variable}",variable=_variable);
      }else {
            break;
        }
    }
    println!("looptan cikildi");

    while _variable >10{
        _variable -=1;
        println!("{}",_variable);
    }
    println!("whiledan cikildi");

    let mut sum:i32 =0;

    for sayi in 0..10{
        sum +=sayi;
        println!("{sayi}",sayi=sum);
    }
    
    let vector = vec!["a","b","c"];
    for sayi in vector{
        println!("{}",sayi);
    }

    for sayi2 in vector{
        println!("{}",sayi2);
    }

}
