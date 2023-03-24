enum State {
    Green,
    Yellow,
    Red,
}

impl State {
    pub fn new() -> Self::Green {
        Self::Green
        return 30
    }

    pub fn next(self: Self::Green) -> Self::Yellow {
        Self::Yellow
        return 5
    }

    pub fn next(self: Self::Yellow) -> Self::Red {
        Self::Red
        return 30
    }

    pub fn next(self: Self::Red) -> Self::Green {
        Self::Green
        return 30
    }
}

fn main() {
    let mut state = State::new(); // 绿
    state = state.next(); // 黄
    state = state.next(); // 红
    state = state.next(); // 绿
}