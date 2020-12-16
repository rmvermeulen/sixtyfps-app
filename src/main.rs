sixtyfps::sixtyfps! {
    import {
        Button, CheckBox, SpinBox, Slider,
        GroupBox, LineEdit, StandardListView, ComboBox
    } from "sixtyfps_widgets.60";

    MyApp := Window {
        signal doThing;
        property<string> header_text <=> input.text;
        property<[string]> files: [];
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
        StandardListView {
        }
    }
}

// sixtyfps::include_modules!();

fn main() {
    do_a_glob();
    run_app();
}
fn run_app() {
    let app = MyApp::new();
    app.on_doThing(|| {
        println!("button clicked!");
    });
    app.run();
}
fn do_a_glob() {
    use glob::glob;
    for entry in glob("D:/assets/*.png").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
            }
            Err(e) => println!("{:?}", e),
        }
    }
    println!("glob done!");
}
