fn main() {
   // v owns the object in heap
   let v = vec![10,20,30];

   //ownership is moved to v2. (v is now invalid)

   let v2 = v;

   // pass a referemce(&v2) to display the function. Ownership is NOt moved. display (&v2);

   //v2 is still valid because we only borrowed it.
   println!("In main{:?}", v2 );
}
   //The function now takes a reference (&Vec<i32>)
   fn display (v: &Vec<i32>) {
      println!("inside display {:?}", v );
      // The reference v is droopped, but v2 in main is still the owner
   }

