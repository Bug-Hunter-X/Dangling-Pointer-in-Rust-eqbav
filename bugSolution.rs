fn main() {
    let mut v = vec![1, 2, 3];
    let mut ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 42; //This is safe

        // Ensure that the pointer remains valid until it is no longer used.
        //One way is to keep v in scope.
    }
}