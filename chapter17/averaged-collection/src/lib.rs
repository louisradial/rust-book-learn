pub struct AveragedCollection {
    list: Vec<i32>,
    average: Option<f64>,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: None,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> Option<f64> {
        self.average
    }

    fn update_average(&mut self) -> () {
        let length = self.list.len();
        if length == 0 {
            self.average = None;
        } else {
            let total: i32 = self.list.iter().sum();
            self.average = Some(total as f64 / self.list.len() as f64);
        }
    }
}
