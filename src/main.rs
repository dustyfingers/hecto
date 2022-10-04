mod editor;

use editor::Editor;

fn main() {
    // add stuff in new env
    let editor = Editor::default();
    editor.run();
}
