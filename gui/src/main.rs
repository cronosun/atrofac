#![windows_subsystem = "windows"]

use atrofac_libgui::engine::Engine;
use atrofac_libgui::system::{
    new_system_interface, MenuItem, MenuItemIdx, MenuItemState, StringMenuItem, SystemEvent,
    SystemInterface,
};
use atrofac_library::AfErr;
use log::info;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::time::Duration;

const MENU_ITEM_RELOAD_CONFIG_OFFSET: usize = 1;
const MENU_ITEM_EDIT_CONFIG_OFFSET: usize = 2;
const MENU_ITEM_EDIT_EXIT_OFFSET: usize = 3;

fn apply(engine: &mut Engine, system: &mut impl SystemInterface) -> Result<(), AfErr> {
    engine.apply()?;

    // set the timer
    if let Some(active_plan) = engine.active_plan() {
        if let Some(refresh_interval_sec) = active_plan.refresh_interval_sec {
            system.set_timer(Duration::from_secs(refresh_interval_sec as u64))?;
        } else {
            system.remove_timer()?;
        }
    }
    Ok(())
}

fn on_apm_resume_automatic(engine: &mut Engine) -> Result<(), AfErr> {
    // Check whether we need to do something (stored in the configuration)
    if let Some(active_plan) = engine.active_plan() {
        // default value is "true"
        let refresh_on_resume = active_plan.refresh_on_apm_resume_automatic.unwrap_or(true);
        if refresh_on_resume {
            info!("Going to re-apply the plan due to wakeup (PBT_APMRESUMEAUTOMATIC).");
            engine.apply()?;
        }
    }
    Ok(())
}

fn load_tray(engine: &Engine, system: &mut impl SystemInterface) -> Result<(), AfErr> {
    system.tray_clear()?;

    let active_plan = engine.active_plan();
    for (_, plan_name) in engine.available_plans() {
        let active = if let Some(active_plan) = active_plan {
            active_plan.name == plan_name
        } else {
            false
        };
        system.tray_add(MenuItem::String(StringMenuItem {
            text: Cow::Borrowed(plan_name.as_str()),
            state: if active {
                MenuItemState::Checked
            } else {
                MenuItemState::Default
            },
        }))?;
    }
    system.tray_add(MenuItem::Separator)?;
    system.tray_add(MenuItem::String(StringMenuItem {
        text: "Reload configuration".into(),
        state: MenuItemState::Default,
    }))?;
    system.tray_add(MenuItem::String(StringMenuItem {
        text: "Edit configuration".into(),
        state: MenuItemState::Default,
    }))?;
    system.tray_add(MenuItem::String(StringMenuItem {
        text: "Quit application".into(),
        state: MenuItemState::Default,
    }))?;
    Ok(())
}

fn on_tray(
    menu_item_id: MenuItemIdx,
    engine: &mut Engine,
    system: &mut impl SystemInterface,
) -> Result<(), AfErr> {
    let index_usize = usize::try_from(menu_item_id.id())?;
    let number_of_plans = engine.number_of_plans();
    if index_usize >= number_of_plans {
        // not a plan
        let offset = index_usize - number_of_plans;
        match offset {
            MENU_ITEM_RELOAD_CONFIG_OFFSET => {
                engine.load_configuration()?;
                load_tray(engine, system)?;
                apply(engine, system)?;
                info!("Plan applied due to configuration file reload.");
                Ok(())
            }
            MENU_ITEM_EDIT_CONFIG_OFFSET => {
                let config_file = engine.config_file();
                system.edit(config_file)?;
                Ok(())
            }
            MENU_ITEM_EDIT_EXIT_OFFSET => {
                info!("Quitting atrofac (user selected quit from menu).");
                system.quit()?;
                Ok(())
            }
            _ => Err(AfErr::from(format!("Unknown menu item offset {}.", offset))),
        }
    } else {
        // it's a plan
        // first re-load the configuration (in case user has edited the file; prevents overwriting the file).
        engine.load_configuration()?;
        if let Some(plan_name) = engine.plan_by_index(menu_item_id.id() as usize).cloned() {
            info!("User selected plan {} in system tray.", plan_name.as_str());
            engine.set_active_plan(plan_name);
            // when the plan has been changed, save the configuration
            engine.save_configuration()?;
            apply(engine, system)?;
            // reload tray
            load_tray(engine, system)?;
            Ok(())
        } else {
            Err(AfErr::from(format!(
                "Plan #{} not found.",
                menu_item_id.id()
            )))
        }
    }
}

fn run_main_with_error(
    engine: &mut Engine,
    system: &mut impl SystemInterface,
) -> Result<(), AfErr> {
    engine.load_configuration()?;
    apply(engine, system)?;
    info!("Plan initially applied while application startup.");
    load_tray(engine, system)?;
    system.tray_tooltip("Control fan curve and power profile for Asus Zephyrus ROG G14.")?;
    system.tray_icon(include_bytes!("../resources/icon.ico"), 64, 64)?;

    // loop
    loop {
        let event = system.receive_event()?;
        if let Some(event) = event {
            match event {
                SystemEvent::OnTimer => {
                    apply(engine, system)?;
                    info!(
                        "Plan successfully re-applied due to timer event ('refresh_interval_sec')."
                    );
                }
                SystemEvent::OnTray(menu_item_id) => {
                    on_tray(menu_item_id, engine, system)?;
                }
                SystemEvent::OnApmResume => {
                    on_apm_resume_automatic(engine)?;
                }
            }
        } else {
            // finish
            return Ok(());
        }
    }
}

fn run_main(engine: &mut Engine, system: &mut impl SystemInterface) {
    if let Err(err) = run_main_with_error(engine, system) {
        system
            .show_err_message("Error", &format!("{}", err))
            .expect("Unable to display error message");
    }
}

fn main() {
    let mut system = new_system_interface().expect("Unable to create system interface");
    let mut engine = Engine::new().expect("Unable to create engine.");
    run_main(&mut engine, &mut system);
}
