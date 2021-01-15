use std::hash::Hash;

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Pos {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Pos {
    pub fn after(mut self, input: &str) -> Pos {
        for c in input.chars() {
            self.offset += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }            
        }
        self
    }
    pub fn advance_lines(mut self, lines: usize, chars: usize) -> Pos {
        self.offset += chars;
        self.line += lines;
        self.column = 0;
        self
    }
    // do not cross a newline boundary with me
    pub fn advance_columns(mut self, columns: usize) -> Pos {
        self.offset += columns;
        self.column = 0;
        self
    }
    pub fn span(self, to: Pos) -> Span {
        Span::new(self, to)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

impl Span {
    pub fn new(start: Pos, end: Pos) -> Span {
        Span { start, end }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Spanning<T> {
    pub inner: T,
    pub span: Span,
}

impl<T> Spanning<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Spanning { inner, span }
    }
}
