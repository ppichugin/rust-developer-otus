use visitor::staticv::{Class, Database, Junior, ProjectElement, Senior, Test};

fn main() {
    let mut class = Class::default();
    let mut db = Database::default();
    let mut test = Test::default();

    println!("Task has been assigned to junior");
    println!("================================");
    class.be_written(&Junior);
    db.be_written(&Junior);
    test.be_written(&Junior);

    println!("\nTask has been assigned to senior");
    println!("================================");
    class.be_written(&Senior);
    db.be_written(&Senior);
    test.be_written(&Senior);
}
