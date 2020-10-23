use imgui::{im_str, Condition, Ui};
use imnodes::*;

pub fn show_basic_node(ui: &Ui) {
    begin_editor();

    begin_node(0);
    ui.text(im_str!("first node in rust"));

    begin_output(1);
    end_output();

    begin_input(2);
    end_input();

    end_node();

    end_editor();
}
