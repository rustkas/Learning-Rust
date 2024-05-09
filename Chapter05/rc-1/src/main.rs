use std::rc::Rc;

fn main()
{
    let memblock: Rc<i64> = Rc::new(256); // allocate space on the heap and assign
    second_method(memblock.clone()); // clone a new reference counted pointer and pass it on to the method
    println!("{}", memblock); // output the value
} // free memory here

fn second_method(memblock: Rc<i64>)
{
	println!("In secondMethod and memblock is {}", memblock);
     let _sec_memblock: Rc<i64> = memblock.clone(); // yet another reference counted pointer to memblock
} // secMemblock goes out of scope, but the memory is not deallocated

