use eframe::{App, CreationContext};

#[cfg(not(target_arch = "wasm32"))]
pub fn run_app<F, T>(app_creator: &F)
where
    F: Fn(&CreationContext) -> T + 'static,
    T: App + 'static,
{
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        env!("CARGO_BIN_NAME"),
        options,
        Box::new(|cc| Ok(Box::new(app_creator(cc)))),
    )
    .expect("Cannot run app");
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
pub fn run_app<F, T>(app_creator: &'static F)
where
    F: Fn(&CreationContext) -> T + 'static,
    T: App + 'static,
{
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    //eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id(env!("CARGO_BIN_NAME"))
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(app_creator(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
