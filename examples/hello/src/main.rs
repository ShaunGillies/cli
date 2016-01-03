extern crate clt;

use clt::Editor;

fn main() {
    let editor = Editor::new("vim");
    editor.edit_file("./test_path/test_file.txt");
}
