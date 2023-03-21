pub struct Process {}

impl Process {
    pub fn pid(&self) -> i32 { return 0; }

    pub fn user(&self) -> String { return String::from("user"); }

    pub fn cpu_utilization(&self) -> f32 { return 0.0; }

    pub fn ram(&self) -> String { return String::from("ram"); }

    pub fn up_time(&self) -> f64 { return 0.0; }

    pub fn command(&self) -> String { return String::from("command"); }
}

pub struct Format {}

impl Format {
    pub fn elapsed_time(seconds: f64) -> String {
        let minutes = seconds / 60.0;
        let hours = minutes / 60.0;
        return hours.to_string() + ":" + &(minutes % 60.0).to_string() + ":" +
               &(seconds % 60.0).to_string();
      }
}