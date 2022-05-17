#[macro_use]
extern crate penrose;

mod hooks;
mod styles;
mod config;

use simplelog::{ LevelFilter, SimpleLogger };
use std::convert::TryFrom;
use clap::Parser;
use penrose::{
    core::{
        config::Config,
        helpers::index_selectors,
        layout::{
            LayoutConf,
            side_stack,
        }, 
        hooks::Hooks,
        Layout,
    },
    xcb::{
        new_xcb_backed_window_manager, 
        XcbDraw,
    },
    draw::{
        dwm_bar, 
        TextStyle,
        Color,
    }, 
    logging_error_handler,
    Backward, 
    Forward, 
    Less, 
    More, 
    Selector, 
    XcbConnection,
};
use styles::{
    colors,
    dimensions,
    PROFONT,
};
use config::Config as application_config;

fn main() -> penrose::Result<()> {

    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    let application_config = application_config::parse();

    let side_stack_layout = Layout::new("[[]=]", LayoutConf::default(), side_stack, 1, 0.6);

    let floating_classes = vec![
        "polybar",
    ];

    let config = Config::default()
        .builder()
        .show_bar(true)
        .top_bar(true)
        .layouts(vec![side_stack_layout])
        .floating_classes(floating_classes)
        .focused_border(colors::GOKU)?
        .build()
        .expect("Unable to build configuration");

    let style = TextStyle {
        font: PROFONT.to_string(),
        point_size: 11,
        fg: Color::try_from(colors::WHITE)?,
        bg: Some(Color::try_from(colors::BLACK)?),
        padding: (2.0, 2.0),
    };

    let empty_ws = Color::try_from(colors::GREY)?;
    let draw = XcbDraw::new()?;

    let bar = dwm_bar(
        draw,
        dimensions::HEIGHT,
        &style,
        Color::try_from(colors::GOKU)?,
        empty_ws,
        config.workspaces().clone(),
    )?;

    let hooks: Hooks<XcbConnection> = vec![
        Box::new(bar),
        Box::new(hooks::StartupScript::new(&application_config.mywm_start_script)),
    ];

    #[allow(unused_braces)]
    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-p" => run_external!({&application_config.mywm_launcher});
        "M-Return" => run_external!({&application_config.mywm_terminal});

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
