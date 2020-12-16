sixtyfps::sixtyfps! {
    HelloWorld := Rectangle {
        Text {
            text:"Hi there (from macro)";
            color:blue;
        }
    }
}

// sixtyfps::include_modules!();

fn main() {
    HelloWorld::new().run();
}
