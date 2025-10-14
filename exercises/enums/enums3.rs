// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.


enum Message {
    // TODO: implement the message variant types based on their usage below
    // 这里 ChangeColor 携带(u8, u8, u8)作为参数，表示 RGB 颜色。
    // 注意：这是三个独立参数。//这是由于下面test  78 行的设置
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8), // 当前颜色，用元组表示 (R,G,B)
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        //这里参数 color 的类型是一个元组 (u8, u8, u8)，意味着调用者必须传入一个“打包好的三元组”：
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message
        // variants
        // Remember: When passing a tuple as a function argument, you'll need
        // extra parentheses: fn function((t, u, p, l, e))
        match message {
            // 注意这里的 (r,g,b) 外层括号的含义：
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)), // 由于 change_color 需要一个整体元组参数，所以要把 (r,g,b) 打包成一个元组传进去：
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));//
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
