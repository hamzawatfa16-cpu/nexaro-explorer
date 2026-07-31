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
use std::env;
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

fn home_path() -> PathBuf {
    dirs::home_dir().unwrap()
}

fn documents_path() -> PathBuf {
    dirs::document_dir().unwrap()
}

fn downloads_path() -> PathBuf {
    dirs::download_dir().unwrap()
}

fn pictures_path() -> PathBuf {
    dirs::picture_dir().unwrap()
}

fn music_path() -> PathBuf {
    dirs::audio_dir().unwrap()
}

fn videos_path() -> PathBuf {
    dirs::video_dir().unwrap()
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

    let controller_clone = controller.clone();
    let window_weak = window.as_weak();

    window.on_open_this_pc(move || {
        let window = window_weak.unwrap();

        let mut controller = controller_clone.borrow_mut();

        controller.open_this_pc();

        load_files(&controller, &window);
    });

    let controller_clone = controller.clone();
    let window_weak = window.as_weak();

    window.on_open_folder(move |path| {
        let window = window_weak.unwrap();

        let mut controller = controller_clone.borrow_mut();

        controller.open_folder(PathBuf::from(path.as_str()));

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

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

    let controller_clone = controller.clone();
    let window_weak = window.as_weak();

    window.on_go_back(move || {
        let window = window_weak.unwrap();

        let mut controller = controller_clone.borrow_mut();

        controller.back();

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_clone = controller.clone();
    let window_weak = window.as_weak();

    window.on_go_forward(move || {
        let window = window_weak.unwrap();

        let mut controller = controller_clone.borrow_mut();

        controller.forward();

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_clone = controller.clone();
    let window_weak = window.as_weak();

    window.on_go_up(move || {
        let window = window_weak.unwrap();

        let mut controller = controller_clone.borrow_mut();

        controller.up();

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_clone = controller.clone();
    let window_weak = window.as_weak();

    window.on_new_folder(move || {
        let window = window_weak.unwrap();

        let mut controller = controller_clone.borrow_mut();

        let current = match &controller.state().location {
            crate::models::location::ExplorerLocation::Folder(path) => path.clone(),
            _ => {
                return;
            }
        };

        let mut folder = current.join("New Folder");

        let mut count = 2;

        while folder.exists() {
            folder = current.join(format!("New Folder ({})", count));

            count += 1;
        }

        if let Err(e) = controller.create_folder(folder) {
            eprintln!("Create folder failed: {:?}", e);
            return;
        }

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_select = controller.clone();

    window.on_select_file(move |path| {
        let mut controller = controller_select.borrow_mut();

        controller.select_file(PathBuf::from(path.as_str()));
    });

    let controller_rename = controller.clone();
    let window_weak_rename = window.as_weak();

    window.on_rename_requested(move || {
        let window = window_weak_rename.unwrap();

        let controller = controller_rename.borrow();

        let selected = controller.selected_files();

        if selected.is_empty() {
            return;
        }

        let old_path = &selected[0];

        if let Some(name) = old_path.file_name() {
            window.set_rename_text(name.to_string_lossy().to_string().into());

            window.set_rename_visible(true);
        }
    });

    let controller_rename_ok = controller.clone();
    let window_weak_rename_ok = window.as_weak();

    window.on_rename_confirmed(move |new_name| {
        let window = window_weak_rename_ok.unwrap();

        let mut controller = controller_rename_ok.borrow_mut();

        let selected = controller.selected_files();

        if selected.is_empty() {
            window.set_rename_visible(false);
            return;
        }

        let old_path = selected[0].clone();

        let parent = match old_path.parent() {
            Some(p) => p.to_path_buf(),
            None => {
                window.set_rename_visible(false);
                return;
            }
        };

        let new_path = parent.join(new_name.as_str());

        if let Err(e) = controller.rename(old_path, new_path) {
            eprintln!("Rename failed: {:?}", e);
            return;
        }

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);

        window.set_rename_visible(false);
    });

    let window_weak_cancel = window.as_weak();

    window.on_rename_cancelled(move || {
        let window = window_weak_cancel.unwrap();
        window.set_rename_visible(false);
    });

    let controller_delete = controller.clone();
    let window_weak_delete = window.as_weak();

    window.on_delete_selected(move || {
        let window = window_weak_delete.unwrap();

        let controller = controller_delete.borrow();

        let selected = controller.selected_files();

        if selected.is_empty() {
            return;
        }

        let name = selected[0]
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        window.set_delete_name(name.into());

        window.set_delete_visible(true);
    });

    let controller_delete_confirm = controller.clone();
    let window_weak_delete_confirm = window.as_weak();

    window.on_delete_confirmed(move || {
        let window = window_weak_delete_confirm.unwrap();

        let mut controller = controller_delete_confirm.borrow_mut();

        if let Err(e) = controller.delete_selected() {
            eprintln!("Delete failed: {:?}", e);
            return;
        }

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);

        window.set_delete_visible(false);
    });

    let window_weak_delete_cancel = window.as_weak();

    window.on_delete_cancelled(move || {
        let window = window_weak_delete_cancel.unwrap();

        window.set_delete_visible(false);
    });

    let controller_search = controller.clone();
    let window_weak = window.as_weak();

    window.on_search_changed(move |query| {
        let window = window_weak.unwrap();

        let mut controller = controller_search.borrow_mut();

        if query.is_empty() {
            if let Err(e) = controller.refresh() {
                eprintln!("Refresh failed: {:?}", e);
                return;
            }
        } else {
            if let Err(e) = controller.search(query.as_str()) {
                eprintln!("Search failed: {:?}", e);
                return;
            }
        }

        load_files(&controller, &window);
    });

    let controller_sidebar = controller.clone();
    let window_weak_sidebar = window.as_weak();

    window.on_navigate_home(move || {
        let window = window_weak_sidebar.unwrap();

        let mut controller = controller_sidebar.borrow_mut();

        controller.open_folder(home_path());

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_sidebar = controller.clone();
    let window_weak_sidebar = window.as_weak();

    window.on_navigate_documents(move || {
        let window = window_weak_sidebar.unwrap();

        let mut controller = controller_sidebar.borrow_mut();

        controller.open_folder(documents_path());

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_sidebar = controller.clone();
    let window_weak_sidebar = window.as_weak();

    window.on_navigate_downloads(move || {
        let window = window_weak_sidebar.unwrap();

        let mut controller = controller_sidebar.borrow_mut();

        controller.open_folder(downloads_path());

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_sidebar = controller.clone();
    let window_weak_sidebar = window.as_weak();

    window.on_navigate_pictures(move || {
        let window = window_weak_sidebar.unwrap();

        let mut controller = controller_sidebar.borrow_mut();

        controller.open_folder(pictures_path());

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_sidebar = controller.clone();
    let window_weak_sidebar = window.as_weak();

    window.on_navigate_music(move || {
        let window = window_weak_sidebar.unwrap();

        let mut controller = controller_sidebar.borrow_mut();

        controller.open_folder(music_path());

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    let controller_sidebar = controller.clone();
    let window_weak_sidebar = window.as_weak();

    window.on_navigate_videos(move || {
        let window = window_weak_sidebar.unwrap();

        let mut controller = controller_sidebar.borrow_mut();

        controller.open_folder(videos_path());

        if let Err(e) = controller.refresh() {
            eprintln!("Refresh failed: {:?}", e);
            return;
        }

        load_files(&controller, &window);
    });

    window.run().unwrap();
}