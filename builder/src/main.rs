#[derive(Debug, PartialEq)]
struct SomeStructWithManyParameters {
    required_parameter_1: String,
    required_parameter_2: String,
    required_parameter_3: String,
    optional_parameter_1: Option<String>,
    optional_parameter_2: Option<String>,
    optional_parameter_3: Option<String>,
}

#[derive(Debug, PartialEq)]
struct SomeStructWithManyParametersBuilder {
    required_parameter_1: String,
    required_parameter_2: String,
    required_parameter_3: String,
    optional_parameter_1: Option<String>,
    optional_parameter_2: Option<String>,
    optional_parameter_3: Option<String>,
}

#[allow(dead_code)]
impl SomeStructWithManyParametersBuilder {
    fn new(
        required_parameter_1: String,
        required_parameter_2: String,
        required_parameter_3: String,
    ) -> SomeStructWithManyParametersBuilder {
        SomeStructWithManyParametersBuilder {
            required_parameter_1,
            required_parameter_2,
            required_parameter_3,
            optional_parameter_1: None,
            optional_parameter_2: None,
            optional_parameter_3: None,
        }
    }

    fn with_optional_parameter_1(mut self, value: String) -> Self {
        self.optional_parameter_1 = Some(value);
        self
    }

    fn with_optional_parameter_2(mut self, value: String) -> Self {
        self.optional_parameter_2 = Some(value);
        self
    }

    fn with_optional_parameter_3(mut self, value: String) -> Self {
        self.optional_parameter_3 = Some(value);
        self
    }

    fn build(self) -> SomeStructWithManyParameters {
        SomeStructWithManyParameters {
            required_parameter_1: self.required_parameter_1,
            required_parameter_2: self.required_parameter_2,
            required_parameter_3: self.required_parameter_3,
            optional_parameter_1: self.optional_parameter_1,
            optional_parameter_2: self.optional_parameter_2,
            optional_parameter_3: self.optional_parameter_3,
        }
    }
}

fn main() {
    let foo_from_scratch = SomeStructWithManyParameters {
        required_parameter_1: String::from("1"),
        required_parameter_2: String::from("2"),
        required_parameter_3: String::from("3"),
        optional_parameter_1: None,
        optional_parameter_2: Some(String::from("Value2")),
        optional_parameter_3: None,
    };

    let req_param_1 = String::from("1");
    let req_param_2 = String::from("2");
    let req_param_3 = String::from("3");
    let opt_param_2 = String::from("Value2");

    let foo_from_builder: SomeStructWithManyParameters =
        SomeStructWithManyParametersBuilder::new(req_param_1, req_param_2, req_param_3)
            .with_optional_parameter_2(opt_param_2)
            .build();

    assert_eq!(foo_from_scratch, foo_from_builder);
}
