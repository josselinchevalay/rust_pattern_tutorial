trait Subject {
    fn attach(&mut self, observer: Box<dyn Observer>);
    fn detach(&mut self, observer: Box<dyn Observer>);
    fn notify(&mut self);
}

trait Observer {
    fn update(&mut self);
}

struct Logger {
    logs: Vec<String> 
}

impl Observer for Logger {
    fn update(&mut self) {
        self.logs.push("Notification received!".to_string());
    }
}

struct SubjectImpl {
    observers: Vec<Box<dyn Observer>>
}

impl Subject for SubjectImpl {
    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, observer: Box<dyn Observer>) {
        self.observers.retain(|x| *x != observer);
    }

    fn notify(&mut self) {
        for observer in self.observers.iter_mut() {
            observer.update();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut subject = SubjectImpl {};
        let logger = Box::new(Logger { logs: vec![] });
        ubject.attach(logger);
        subject.notify();
        assert_eq!(logger.logs, vec!["Notification received!".to_string()]);
    }
}
