fn main() {
    let v = vec![20, 40, 60, 80];
    //vector v owns the object in the heap

    //ownership is moved from v to v2. (v is now invalid)
    let v2 = v;


    //ownership is moved from v2 into display
    //The display function returns the ownership back
    //which is then assigned to v2_return.
    let v2_return = display(v2);

    //The previously used variable 'v' is invalid.
    //We must use the variable that now holds the onwership : v2_ return.
    println!("In main {:?}", v2_return);
}
     fn display (v:Vec<i32>)->Vec<i32> {
        // v owns the object inside this function
        println!("iniside display {:?}",v);

        //ownership is moved back to the caller 
        return v;

    }

    
