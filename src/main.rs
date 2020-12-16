sixtyfps::sixtyfps! {
    import {
        Button, CheckBox, SpinBox, Slider,
        GroupBox, LineEdit, StandardListView, ComboBox
    } from "sixtyfps_widgets.60";

    MyApp := Window {
        signal doThing;
        property<string> header_text <=> input.text;
        property<[StandardListViewItem]>files <=> list.model;
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
            list := StandardListView {
                model: [
                    {text:"file1"},
                    {text:"file2"},
                    {text:"file3"}
                ];
            }
        }
    }
}
struct StandardListViewItem {
    text: sixtyfps::SharedString,
}
// sixtyfps::include_modules!();

fn main() {
    run_app();
    // do_a_glob();
}

fn run_app() {
    use sixtyfps::SharedString;
    let app = MyApp::new();
    app.on_doThing(|| {
        println!("button clicked!");
        use glob::glob;
        let files = glob("./src/*.rs")
            .unwrap()
            .filter_map(|entry| match entry {
                Ok(path) => Some(path),
                Err(e) => None,
            })
            .filter_map(|path| path.to_str())
            .map(|str| {
                Box::new(StandardListViewItem {
                    text: SharedString::from(str),
                })
            })
            .collect();
        app.set_files(sixtyfps::ModelHandle::new(files));
    });
    app.run();
}

fn do_a_glob() {
    use glob::glob;
    for entry in glob("./src/*.rs").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
            }
            Err(e) => println!("{:?}", e),
        }
    }
    println!("glob done!");
}
