use gtk::gio;

fn main() {
    gio::compile_resources(
        "resources",
        "resources/resources.gresource.xml",
        "test.gresource",
    );
}