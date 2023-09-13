use std::borrow::Borrow;
use visitor::dynamic_visitor::{Class, Database, Developer, Junior, ProjectElement, Senior, Test};

fn main() {
    let mut class = Class::default();
    let mut db = Database::default();
    let mut test = Test::default();

    let junior: &dyn Developer = Junior.borrow();
    let senior: &dyn Developer = Senior.borrow();

    println!("Task has been assigned to junior");
    println!("================================");
    class.be_written(junior);
    db.be_written(junior);
    test.be_written(junior);

    println!("\nTask has been assigned to senior");
    println!("================================");
    class.be_written(senior);
    db.be_written(senior);
    test.be_written(senior);
}