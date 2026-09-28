fn main() {
/*we can define a variable using let kw that will make a variable which will be immutable by default */
let mut age:u32 = 19 ; 
println!("{age}");

// now by default it is immtable to make it mutablee we add mut kw
age = 90 ;
println!("{age}");

// but we can aslp create a variable with kw const that will basically make a variable constant meaninng that it is always immutable you may say that let is also immutable then what is the difference but the diiference is we can make the let as a mutable adding <<mut>> kw while in const we cant make it mut . also another differnce is let can only be made inside a certain scope while const can be declared in any scope including global scope .. also the final diffeernce is like in let we are free to not give its type annotations but in the case of const declared ariable we must explicitly declare its type annotation its must ... also inside const we can add a value profoming ceartain oepration but it must be independednt of any other variable made by let if it have to intearct or perfrom operation using variable that operand variable must be declared wth const.

// i.e const age:u32 = 1+7+age_const ; here age_const must be a const declared variable


// let random_value = 23 ;
// const PI:f32 = 3.14;
// PI=1.2; cant be done
// const PI:f32 = 3.14+random_value; error

const RANDOM_VALUE:f32 = 12.0 ;
const PI:f32 = 3.14 * RANDOM_VALUE;

println!("{PI}");


// shadowing shadowing is the concept where you can redeclare the same variabe and reuse its previous value...
let my_num  = 12 ;

let my_num= false;

// here my_nm initially have interger type but we can make redeclare and reaassign complete new evalue withs hadwoing similarly you can say that we can do it via mut too but in mut we can only reassign the value of the type that we gave whie declaring....
println!("{my_num}");
}
