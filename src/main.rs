mod app;
mod core;
mod models;
mod platform;
mod ui_bridge;
mod utils;

slint::include_modules!();

use crate::app::controller::ExplorerController;
use crate::ui_bridge::breadcrumb_mapper::build_breadcrumbs;
use crate::ui_bridge::mapper::map_files;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn load_files(controller: &ExplorerController, window: &MainWindow) {
    let files = map_files(controller.files());

    let ui_files: Vec<UiFileItem> = files
        .into_iter()
        .map(|f| UiFileItem {
            name: f.name.into(),
            path: f.path.into(),
            file_type: f.file_type.into(),
            size: f.size.into(),
            modified: f.modified.into(),
            icon: f.icon.into(),
        })
        .collect();

    window.set_files(ui_files.as_slice().into());
    window.set_selected_count(controller.selected_files().len() as i32);

    window.set_current_path(match &controller.state().location {
        crate::models::location::ExplorerLocation::Folder(path) => {
            path.to_string_lossy().to_string().into()
        }
        crate::models::location::ExplorerLocation::ThisPc => "This PC".into(),
        crate::models::location::ExplorerLocation::QuickAccess => "Quick Access".into(),
        crate::models::location::ExplorerLocation::Network => "Network".into(),
    });

    match &controller.state().location {
        crate::models::location::ExplorerLocation::Folder(path) => {
            let crumbs = build_breadcrumbs(path);

            let ui_crumbs: Vec<UiBreadcrumb> = crumbs
                .into_iter()
                .map(|c| UiBreadcrumb {
                    label: c.label.into(),
                    path: c.path.into(),
                })
                .collect();

            window.set_breadcrumbs(ui_crumbs.as_slice().into());
        }

        crate::models::location::ExplorerLocation::ThisPc => {
            window.set_breadcrumbs(
                vec![UiBreadcrumb {
                    label: "This PC".into(),
                    path: "This PC".into(),
                }]
                .as_slice()
                .into(),
            );
        }

        _ => {}
    }
}

fn open_special_folder(
    controller: &Rc<RefCell<ExplorerController>>,
    window: &MainWindow,
    path: PathBuf,
) {
    let mut controller = controller.borrow_mut();

    controller.open_folder(path);

    if let Err(e) = controller.refresh() {
        eprintln!("Failed to open folder: {:?}", e);
        return;
    }

    load_files(&controller, window);
}

fn main() {
    let controller = Rc::new(RefCell::new(ExplorerController::new(PathBuf::from("."))));

    if let Err(e) = controller.borrow_mut().refresh() {
        eprintln!("Refresh failed: {:?}", e);
        return;
    }

    let window = MainWindow::new().unwrap();

    load_files(&controller.borrow(), &window);

    {
        let window_weak = window.as_weak();
        window.on_toggle_sidebar(move || {
            if let Some(window) = window_weak.upgrade() {
                window.set_sidebar_compact(!window.get_sidebar_compact());
            }
        });
    }

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_select_file(move |path| {
            if let Some(window) = window_weak.upgrade() {
                let mut controller = controller.borrow_mut();
                controller.clear_selection();
                controller.select_file(PathBuf::from(path.as_str()));
                window.set_selected_count(controller.selected_files().len() as i32);
            }
        });
    }

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_breadcrumb_clicked(move |path| {
            if let Some(window) = window_weak.upgrade() {
                let mut controller = controller.borrow_mut();

                if path == "This PC" {
                    controller.open_this_pc();
                } else {
                    controller.open_folder(PathBuf::from(path.as_str()));

                    if let Err(e) = controller.refresh() {
                        eprintln!("Breadcrumb navigation failed: {:?}", e);
                        return;
                    }
                }

                load_files(&controller, &window);
            }
        });
    }

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_open_this_pc(move || {
            if let Some(window) = window_weak.upgrade() {
                let mut controller = controller.borrow_mut();
                controller.open_this_pc();
                load_files(&controller, &window);
            }
        });
    }

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_open_folder(move |path| {
            if let Some(window) = window_weak.upgrade() {
                let mut controller = controller.borrow_mut();
                controller.open_folder(PathBuf::from(path.as_str()));

                if let Err(e) = controller.refresh() {
                    eprintln!("Refresh failed: {:?}", e);
                    return;
                }

                load_files(&controller, &window);
            }
        });
    }

    window.on_open_file(move |path| {
        let _ = open::that(path.as_str());
    });

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_open_documents(move || {
            if let Some(window) = window_weak.upgrade() {
                if let Some(path) = dirs::document_dir() {
                    open_special_folder(&controller, &window, path);
                }
            }
        });
    }

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_open_downloads(move || {
            if let Some(window) = window_weak.upgrade() {
                if let Some(path) = dirs::download_dir() {
                    open_special_folder(&controller, &window, path);
                }
            }
        });
    }

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_open_pictures(move || {
            if let Some(window) = window_weak.upgrade() {
                if let Some(path) = dirs::picture_dir() {
                    open_special_folder(&controller, &window, path);
                }
            }
        });
    }

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_open_music(move || {
            if let Some(window) = window_weak.upgrade() {
                if let Some(path) = dirs::audio_dir() {
                    open_special_folder(&controller, &window, path);
                }
            }
        });
    }

    {
        let controller = controller.clone();
        let window_weak = window.as_weak();

        window.on_open_videos(move || {
            if let Some(window) = window_weak.upgrade() {
                if let Some(path) = dirs::video_dir() {
                    open_special_folder(&controller, &window, path);
                }
            }
        });
    }

    window.run().unwrap();
}
