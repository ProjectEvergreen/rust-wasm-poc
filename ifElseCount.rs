fn main() {
    //https://doc.rust-lang.org/1.0.0/book/mutability.html
    let mut count = 0u32;
    let mut count_desending = 29u32;
    //`<number_to_start_counter>u32`In human terms, this pretty much means 0 then u32.
    // mut helps call it in counter store below.
    println!("Start the age counter:");
    loop {
        //store it here
        count += 1;
        count_desending -= 1;
        if count_desending == 28 {
            //this is the start of count_desending;
            println!("This is my age: {0}", count_desending);
            continue;
        }
        // which is pretty much:  let  count_desending = 28u32 minus decrement the count;
        if count == 22 {
            //reached a point of reference in count;
            println!("You reached a point of reference in {0} counts!", count);
            continue;
        }
        //get count.
        println!("{0}", count);
        if count == 29 {
            println!("This almost my age: {0}", count);
            //end loop of age
            break;
        }
       'nameThatLoopHere: loop {
        //get count_desending
        println!("---> {0}", count_desending);
        //end the namedThatLoopHere;
        break 'nameThatLoopHere;
      }
    }
}
