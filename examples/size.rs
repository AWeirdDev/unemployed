use unemployment::objects::{ EmObject };

fn main() {
    let a = vec![100];
    println!("{}", size_of_val(&a));

    let obj = EmObject::new(a);
    println!("{}", size_of_val(&obj));

    let a = obj.into_any();
    let x = unsafe { a.cast::<Vec<i32>>() };

    let item = unsafe { x.as_ref() };
    println!("{item:#?}");
    println!("{}", size_of_val(&item));
}
