use crate::{
    Junior, Senior, JUNIOR_CLASS, JUNIOR_DB, JUNIOR_TEST, SENIOR_CLASS, SENIOR_DB, SENIOR_TEST,
};

#[derive(Default)]
pub struct Object {
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
    fn be_written(&mut self, v: &dyn Developer);
    fn get_info(&self);
}

impl ProjectElement for Object {
    fn be_written(&mut self, v: &dyn Developer) {
        v.create_class(self);
    }

    fn get_info(&self) {
        println!("{}", self.info);
    }
}

impl ProjectElement for Database {
    fn be_written(&mut self, v: &dyn Developer) {
        v.create_db(self);
    }

    fn get_info(&self) {
        println!("{}", self.info);
    }
}

impl ProjectElement for Test {
    fn be_written(&mut self, v: &dyn Developer) {
        v.create_test(self);
    }

    fn get_info(&self) {
        println!("{}", self.info);
    }
}

pub trait Developer {
    fn create_class(&self, element: &mut Object);
    fn create_db(&self, element: &mut Database);
    fn create_test(&self, element: &mut Test);
}

impl Developer for Junior {
    fn create_class(&self, element: &mut Object) {
        element.info = JUNIOR_CLASS;
        element.get_info();
    }

    fn create_db(&self, element: &mut Database) {
        element.info = JUNIOR_DB;
        element.get_info();
    }

    fn create_test(&self, element: &mut Test) {
        element.info = JUNIOR_TEST;
        element.get_info();
    }
}

impl Developer for Senior {
    fn create_class(&self, element: &mut Object) {
        element.info = SENIOR_CLASS;
        element.get_info();
    }

    fn create_db(&self, element: &mut Database) {
        element.info = SENIOR_DB;
        element.get_info();
    }

    fn create_test(&self, element: &mut Test) {
        element.info = SENIOR_TEST;
        element.get_info();
    }
}
