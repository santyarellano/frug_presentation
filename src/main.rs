use frug;

fn main() {
    let (frug_instance, event_loop) = frug::new("My Window");

    let update_function = move |_instance: &mut frug::FrugInstance, _input: &frug::InputHelper| {
        // your frame per frame code will go here!
    };

    frug_instance.run(event_loop, update_function);
}
