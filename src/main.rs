#[macro_use]
extern crate penrose;

use std::convert::TryFrom;

use penrose::{
    core::{
        bindings::KeyEventHandler,
        config::Config,
        helpers::index_selectors,
        manager::WindowManager, 
        Layout,
        layout::{
            LayoutConf,
            side_stack,
        }, hooks::{
            Hooks,
            Hook,
        },
        xconnection::XConn,
    },
    contrib::layouts::dwindle,
    logging_error_handler,
    xcb::{new_xcb_backed_window_manager, XcbDraw},
    Backward, Forward, Less, More, Selector, draw::{dwm_bar, TextStyle}, __test_helpers::Color, XcbConnection, Result
};

use simplelog::{LevelFilter, SimpleLogger};

// Replace these with your preferred terminal and program launcher
const TERMINAL: &str = "kitty";
const LAUNCHER: &str = "dmenu_run";

const HEIGHT: usize = 18;

const PROFONT: &str = "JetBrainsMono Nerd Font";

const BLACK: u32 = 0x282828ff;
const GREY: u32 = 0x3c3836ff;
const WHITE: u32 = 0xebdbb2ff;
const PURPLE: u32 = 0xb16286ff;
const BLUE: u32 = 0x458588ff;
const RED: u32 = 0xcc241dff;

fn main() -> penrose::Result<()> {

    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    let side_stack_layout = Layout::new("[<>=]", LayoutConf::default(), side_stack, 1, 0.6);

    let config = Config::default()
        .builder()
        .show_bar(true)
        .top_bar(true)
        .layouts(vec![side_stack_layout])
        .build()
        .unwrap();

    let style = TextStyle {
        font: PROFONT.to_string(),
        point_size: 11,
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2.0, 2.0),
    };

    let highlight = BLUE;
    let empty_ws = GREY;
    let draw = XcbDraw::new()?;
    let bar = dwm_bar(
        draw,
        HEIGHT,
        &style,
        highlight,
        empty_ws,
        config.workspaces().clone(),
    )?;

    let hooks: Hooks<XcbConnection> = vec![
        Box::new(bar),
        Box::new(StartupScript::new("/home/chris/.MYWM")),
    ];

    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-p" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        // Exit Penrose (important to remember this one!)
        "M-A-C-Escape" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-c" => run_internal!(kill_client);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-period" => run_internal!(cycle_workspace, Forward);
        "M-A-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-l" => run_internal!(update_main_ratio, More);
        "M-h" => run_internal!(update_main_ratio, Less);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
             "M-{}" => focus_workspace (REF);
             "M-S-{}" => client_to_workspace (REF);
         };
    };

    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})
}


pub struct StartupScript {
    path: String,
}

impl StartupScript {
    pub fn new(s: impl Into<String>) -> Self {
        Self { path: s.into() }
    }
}

impl<X: XConn> Hook<X> for StartupScript {
    fn startup(&mut self, _: &mut WindowManager<X>) -> Result<()> {
        spawn!(&self.path)
    }
}
