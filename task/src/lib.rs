use core::fmt;

const NO_TITLE: &str = "no title";

pub struct Task {
    title: String,
    done: bool,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: String::from(NO_TITLE),
            done: false,
        }
    }
}

impl Task {
    pub fn done(&mut self) {
        self.done = true
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn from_string_to_list(list: &str) -> Vec<Self> {
        list.split("\n").map(Task::from).collect()
    }
}

impl From<&str> for Task {
    fn from(task: &str) -> Self {
        if task.is_empty() {
            Default::default()
        }

        let mut split = task.split("::");
        let Some(title) = split.next() else {
            return Default::default();
        };

        let mut task = Task {
            title: title.to_string(),
            done: false,
        };

        if let Some(done) = split.next()
            && done.to_lowercase() == "done"
        {
            task.done();
        }

        task
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_done = match self.is_done() {
            true => " (completed)",
            _ => "",
        };
        f.write_fmt(format_args!("{}{}", self.title, is_done))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.title.to_lowercase() == other.title.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        assert_eq!(Task::from("Test").title, "Test")
    }

    #[test]
    fn from_is_done() {
        assert!(Task::from("Test::done").is_done())
    }

    #[test]
    fn is_done() {
        let mut t = Task::from("sample");
        t.done();
        assert!(t.is_done());
    }

    #[test]
    fn to_string() {
        let mut t = Task::from("test");
        assert_eq!(t.to_string(), "test");
        t.done();
        assert_eq!(t.to_string(), "test (completed)")
    }

    #[test]
    fn are_equal() {
        let t = Task::from("Test me");
        let t2 = Task::from("test Me");
        assert!(t == t2);
    }
}
