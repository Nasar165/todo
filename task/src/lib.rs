use core::fmt;
use std::ops::Deref;

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
}

impl From<&str> for Task {
    fn from(title: &str) -> Self {
        if title.is_empty() {
            Default::default()
        }

        Task {
            title: title.to_string(),
            done: false,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.title))
    }
}

impl Deref for Task {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.title
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
    fn default() {
        assert_eq!(*Task::default(), NO_TITLE)
    }

    #[test]
    fn from() {
        assert_eq!(Task::from("Test").title, "Test")
    }

    #[test]
    fn is_done() {
        let mut t = Task::from("sample");
        t.done();
        assert!(t.is_done());
    }

    #[test]
    fn to_string() {
        assert_eq!(Task::from("test").to_string(), "test");
    }

    #[test]
    fn are_equal() {
        let t = Task::from("Test me");
        let t2 = Task::from("test Me");
        assert!(t == t2);
    }
}
