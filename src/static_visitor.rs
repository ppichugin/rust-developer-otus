#[derive(Default)]
pub struct Class {
    info: &'static str,
}

#[derive(Default)]
pub struct Database {
    info: &'static str,
}

#[derive(Default)]
pub struct Test {
    info: &'static str,
}

pub trait ProjectElement {
    fn be_written(&mut self, v: &impl Developer);
    fn get_info(&self);
}

impl ProjectElement for Class {
    fn be_written(&mut self, v: &impl Developer) {
        v.create_class(self);
    }

    fn get_info(&self) {
        println!("{}", self.info);
    }
}
impl ProjectElement for Database {
    fn be_written(&mut self, v: &impl Developer) {
        v.create_db(self);
    }

    fn get_info(&self) {
        println!("{}", self.info);
    }
}

impl ProjectElement for Test {
    fn be_written(&mut self, v: &impl Developer) {
        v.create_test(self);
    }

    fn get_info(&self) {
        println!("{}", self.info);
    }
}

pub trait Developer {
    fn create_class(&self, element: &mut Class);
    fn create_db(&self, element: &mut Database);
    fn create_test(&self, element: &mut Test);
}

pub struct Junior;

impl Developer for Junior {
    fn create_class(&self, element: &mut Class) {
        element.info = "Writing poor class...";
        element.get_info();
    }

    fn create_db(&self, element: &mut Database) {
        element.info = "Drop database...";
        element.get_info();
    }

    fn create_test(&self, element: &mut Test) {
        element.info = "Creating not reliable test...";
        element.get_info();
    }
}

pub struct Senior;

impl Developer for Senior {
    fn create_class(&self, element: &mut Class) {
        element.info = "Rewriting class after junior...";
        element.get_info();
    }

    fn create_db(&self, element: &mut Database) {
        element.info = "Fixing database...";
        element.get_info();
    }

    fn create_test(&self, element: &mut Test) {
        element.info = "Creating reliable test...";
        element.get_info();
    }
}
