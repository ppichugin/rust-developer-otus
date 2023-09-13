use std::borrow::Borrow;
use visitor::dynamicv::{Database, Developer, Object, ProjectElement, Test};
use visitor::{Junior, Senior};

fn main() {
    let mut class = Object::default();
    let mut db = Database::default();
    let mut test = Test::default();

    let junior: &dyn Developer = Junior.borrow();
    let senior: &dyn Developer = Senior.borrow();

    println!("-------------------");
    println!("Junior is working:");
    class.be_written(junior);
    db.be_written(junior);
    test.be_written(junior);

    println!("\n-------------------");
    println!("Senior is working:");
    class.be_written(senior);
    db.be_written(senior);
    test.be_written(senior);
}
