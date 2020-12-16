sixtyfps::sixtyfps! {
    import {
        Button, CheckBox, SpinBox, Slider,
        GroupBox, LineEdit, StandardListView, ComboBox
    } from "sixtyfps_widgets.60";

    HelloWorld := Window {
        signal doThing;
        property<string> header_text <=> input.text;
        GridLayout {

            Rectangle {
                Text {
                    text: header_text;
                    color: blue;
                    width: 50%;
                }
                color: red;
                height: 20%;
            }
            width: 800px;
            height: 500px;
            input := TextInput {
                text: "thisisaninput";
            }
            Button {
                text: "Regular Button";
                enabled: true;
                clicked => { root.doThing() }
            }
        }
    }
}

// sixtyfps::include_modules!();

fn main() {
    let app = HelloWorld::new();
    app.on_doThing(|| {
        println!("button clicked!");
    });
    app.run();
}
