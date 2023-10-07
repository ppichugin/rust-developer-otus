trait Command {
    fn execute(&self);
}

struct MacroCommand {
    stack: Vec<Box<dyn Command>>,
}

impl MacroCommand {
    fn new() -> MacroCommand {
        MacroCommand { stack: Vec::new() }
    }

    fn append(&mut self, cmd: Box<dyn Command>) {
        self.stack.push(cmd);
    }

    fn undo(&mut self) {
        self.stack.pop();
    }

    fn clear(&mut self) {
        self.stack.clear();
    }
}

impl Command for MacroCommand {
    fn execute(&self) {
        for command in &self.stack {
            command.execute();
        }
    }
}

struct DrawCommand {
    drawable: Box<dyn Drawable>,
    x: u32,
    y: u32,
}

impl DrawCommand {
    fn new(drawable: Box<dyn Drawable>, x: u32, y: u32) -> DrawCommand {
        DrawCommand { drawable, x, y }
    }
}

impl Command for DrawCommand {
    fn execute(&self) {
        self.drawable.draw(self.x, self.y);
    }
}

trait Drawable {
    fn draw(&self, x: u32, y: u32);
}

#[derive(Clone)]
struct DrawCanvas {}

impl DrawCanvas {
    fn new() -> DrawCanvas {
        DrawCanvas {}
    }
}

impl Drawable for DrawCanvas {
    fn draw(&self, x: u32, y: u32) {
        println!("draw(x:{}, y:{})", x, y);
    }
}

fn main() {
    let mut history = MacroCommand::new();
    let canvas = Box::new(DrawCanvas::new());

    let cmd1 = Box::new(DrawCommand::new(canvas.clone(), 1, 1));
    let cmd2 = Box::new(DrawCommand::new(canvas.clone(), 2, 2));
    let cmd3 = Box::new(DrawCommand::new(canvas.clone(), 3, 3));

    history.append(cmd1);
    history.append(cmd2);
    history.append(cmd3);

    println!("----------");
    history.execute();
    println!();

    println!("---undo---");
    history.undo();
    history.execute();
    println!();

    println!("---clear---");
    history.clear();
    history.execute();
}
