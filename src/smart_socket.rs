pub struct ImitationSmartSocket {
    pub(crate) response: String,
    is_on: bool,
}

impl ImitationSmartSocket {
    pub fn new(response: &str) -> Self {
        ImitationSmartSocket {
            response: response.to_string(),
            is_on: false, // Изначально умная розетка выключена
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
        self.response = self.get_info();
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        self.response = self.get_info();
    }

    pub fn get_info(&self) -> String {
        format!(
            "Power: {}. Consumption: {}W.\n",
            if self.is_on { "On" } else { "Off" },
            if self.is_on { 100 } else { 0 } // Имитируем потребление мощности
        )
    }
}
