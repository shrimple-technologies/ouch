use adw::prelude::*;
use webkit6::{prelude::*, NetworkError, WebView};

pub fn init(app: &adw::Application) {
	let builder = gtk::Builder::from_string(
		include_str!("ui/window.ui")
	);
	let window = builder
		.object::<adw::ApplicationWindow>("window")
		.expect("Couldn't get window");
	let web_view_frame = builder
		.object::<gtk::Frame>("frame")
		.expect("Couldn't get web view frame");
	let url_dialog = builder
		.object::<adw::Dialog>("url_dialog")
		.expect("Couldn't get url dialog");
	let url_bar = builder
		.object::<gtk::Button>("url_bar")
		.expect("Couldn't get url bar");
	let _progress = builder
		.object::<gtk::ProgressBar>("progress")
		.expect("Couldn't get web view progress");

	let web_view = WebView::new();
	web_view.connect_load_failed(|web_view, _, fail_url, error| {
        if !error.matches(NetworkError::Cancelled) {
            let content = error_page(error.message());
            web_view.load_alternate_html(&content, fail_url, None);
        }
        false
    });

	web_view.load_uri("https://wikiwand.com/en/%22Hello%2C_World!%22_program");
	web_view_frame.set_child(Some(&web_view));

	window.set_application(Some(app));
	window.present();

	url_bar.connect_clicked(move |_| {
		url_dialog.present(Some(&window));
	});
}

fn error_page(msg: &str) -> String {
	format!(r#"
		<!doctype html>
			<html>
		    	<head>
    				<style>
        				* {{ margin: 0; }}

       					body {{
            				font-family: sans-serif;
           					margin-left: 20vh;
            				margin-right: 20vh;
            				position: absolute;
            				top: 40%;
            				transform: translateY(-40%);
						}}

        				svg {{ margin-bottom: 0.5rem; }}
    				</style>
				</head>
    			<body>
        			<svg height="64px" viewBox="0 0 16 16" width="64px" xmlns="http://www.w3.org/2000/svg">
            			<path d="m 3 0 c -1.660156 0 -3 1.339844 -3 3 v 7 c 0 1.660156 1.339844 3 3 3 h 10 c 1.660156 0 3 -1.339844 3 -3 v -7 c 0 -1.660156 -1.339844 -3 -3 -3 z m 0 2 h 10 c 0.554688 0 1 0.445312 1 1 v 7 c 0 0.554688 -0.445312 1 -1 1 h -10 c -0.554688 0 -1 -0.445312 -1 -1 v -7 c 0 -0.554688 0.445312 -1 1 -1 z m 3 2 c -0.550781 0 -1 0.449219 -1 1 s 0.449219 1 1 1 s 1 -0.449219 1 -1 s -0.449219 -1 -1 -1 z m 4 0 c -0.550781 0 -1 0.449219 -1 1 s 0.449219 1 1 1 s 1 -0.449219 1 -1 s -0.449219 -1 -1 -1 z m -2 3 c -1.429688 0 -2.75 0.761719 -3.464844 2 c -0.136718 0.238281 -0.054687 0.546875 0.183594 0.683594 c 0.238281 0.136718 0.546875 0.054687 0.683594 -0.183594 c 0.535156 -0.929688 1.523437 -1.5 2.597656 -1.5 s 2.0625 0.570312 2.597656 1.5 c 0.136719 0.238281 0.445313 0.320312 0.683594 0.183594 c 0.238281 -0.136719 0.320312 -0.445313 0.183594 -0.683594 c -0.714844 -1.238281 -2.035156 -2 -3.464844 -2 z m -3 7 c -1.105469 0 -2 0.894531 -2 2 h 10 c 0 -1.105469 -0.894531 -2 -2 -2 z m 0 0" fill="\#2e3436"/>
        			</svg>
        			<h3>There was an error loading this website</h3>
        			<small>{msg}</small>
    			</body>
			</html>"#)
}
