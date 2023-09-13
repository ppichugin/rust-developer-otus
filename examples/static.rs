use visitor::staticv::{Database, Object, ProjectElement, Test};
use visitor::{Junior, Senior};

fn main() {
    let mut class = Object::default();
    let mut db = Database::default();
    let mut test = Test::default();

    println!("-------------------");
    println!("Junior is working:");
    class.be_written(&Junior);
    db.be_written(&Junior);
    test.be_written(&Junior);

    println!("\n-------------------");
    println!("Senior is working:");
    class.be_written(&Senior);
    db.be_written(&Senior);
    test.be_written(&Senior);
}
