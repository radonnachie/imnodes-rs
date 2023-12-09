use imnodes::{editor, PinShape};

/// https://github.com/Nelarius/imnodes/blob/master/example/hello.cpp
pub fn show(ui: &imgui::Ui, context: &mut imnodes::EditorContext) {
    let mut id_generator = context.new_identifier_generator();

    editor(context, |mut editor| {
        editor.add_node(id_generator.next_node(), |mut node| {
            node.add_titlebar(|| {
                ui.text("simple node :)");
            });
            
            node.add_input(id_generator.next_input_pin(), PinShape::QuadFilled, ||{
                ui.text("parameter_sink")
            });
            node.add_output(id_generator.next_output_pin(), PinShape::QuadFilled, ||{
                ui.text("parameter_source")
            });

            node.add_input(
                id_generator.next_input_pin(),
                imnodes::PinShape::Circle,
                || ui.text("input"),
            );

            node.add_output(
                id_generator.next_output_pin(),
                imnodes::PinShape::QuadFilled,
                || ui.text("output"),
            );
        });
    });
}
