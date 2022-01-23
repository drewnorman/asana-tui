/// Oversees management of view data.
///
#[derive(Debug, PartialEq, Eq)]
pub struct View {
    title: String,
}

impl View {
    /// Returns the view title.
    ///
    pub fn title(&self) -> &str {
        &self.title
    }
}

/// Returns the default welcome view.
///
pub fn welcome() -> View {
    View {
        title: String::from("Welcome"),
    }
}

/// Returns the default my-tasks view.
///
pub fn my_tasks() -> View {
    View {
        title: String::from("My Tasks"),
    }
}

/// Returns the default due-soon view.
///
pub fn due_soon() -> View {
    View {
        title: String::from("Due Soon"),
    }
}

/// Returns the default past-due view.
///
pub fn past_due() -> View {
    View {
        title: String::from("Past Due"),
    }
}

/// Returns the default recently-created view.
///
pub fn recently_created() -> View {
    View {
        title: String::from("Recently Created"),
    }
}

/// Returns the default recently-edited view.
///
pub fn recently_edited() -> View {
    View {
        title: String::from("Recently Edited"),
    }
}

/// Returns the default recently-completed view.
///
pub fn recently_completed() -> View {
    View {
        title: String::from("Recently Completed"),
    }
}
