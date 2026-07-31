use crate::ui_bridge::breadcrumb::Breadcrumb;

use std::path::{Path, PathBuf};


pub fn build_breadcrumbs(
    path: &Path,
) -> Vec<Breadcrumb> {

    let mut result = Vec::new();


    let mut current = PathBuf::new();


    for component in path.components() {

        current.push(component.as_os_str());


        let label = component
            .as_os_str()
            .to_string_lossy()
            .to_string();


        result.push(Breadcrumb {

            label,

            path: current
                .to_string_lossy()
                .to_string(),
        });
    }


    result
}