use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

#[derive(Default, Debug)]
pub struct ErrorList<T: Display + Debug> {
    errors: Vec<T>,
    initial: String,
    stop: &'static str,
}

impl<T: Display + Debug> ErrorList<T> {
    pub fn new(initial: impl Display, stop: &'static str) -> Self {
        Self {
            errors: Vec::new(),
            initial: initial.to_string(),
            stop,
        }
    }

    pub fn push(&mut self, err: T) {
        self.errors.push(err);
    }
}

fn indent(s: impl Display, first: impl Display, mid: impl Display, last: impl Display) -> String {
    let s = s.to_string();
    let lines = s.lines().collect::<Vec<_>>();
    lines
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let before = if i == 0 {
                first.to_string()
            } else if i < lines.len() - 1 {
                mid.to_string()
            } else {
                last.to_string()
            };
            before + s
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

impl Display for ErrorList<anyhow::Error> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.initial)?;

        let indent_n = |i, n: usize, s| {
            indent(
                s,
                format!("{}{i}: ", " ".repeat(n)),
                format!("{}│  ", " ".repeat(n)),
                format!("{}└  ", " ".repeat(n)),
            )
        };

        for (i, err) in self.errors.iter().enumerate() {
            let mut out = String::new();
            out += &format!("{err}\n");

            let chain = err.chain().collect::<Vec<_>>();
            if chain.len() > 1 {
                out += "\nCaused By:\n";
                for (i, err) in chain.into_iter().skip(1).enumerate() {
                    let err = err.to_string();
                    out += &indent_n(i, 3, err);
                }
            }

            out += "\nBacktrace:\n";
            let s = err.backtrace().to_string();
            let lines = s.lines().collect::<Vec<_>>();
            let backtrace = lines
                .as_slice()
                .windows(3)
                .skip_while(|l: &&[&str]| !l[1].contains(": amplitude"))
                .take_while(|l| !l[0].contains(self.stop))
                .map(|l| l[1])
                .collect::<Vec<_>>()
                .join("\n");
            out += &backtrace;

            write!(f, "\n{}", indent_n(i, 0, out))?;
        }

        Ok(())
    }
}

impl Error for ErrorList<anyhow::Error> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
