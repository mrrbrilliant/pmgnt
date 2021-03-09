use crate::pi_statics::{DEPGRAPH, LOCAL_DIR, SYNC_DIR};
use crate::structs::Manifest;

pub fn is_installed(name: &str) -> bool {
    let file_path = LOCAL_DIR.join(name).join("manifest.yml");
    if file_path.exists() {
        let pkg = Manifest::from_file(file_path);
        if pkg.pkgname == name {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn register_deps(name: &str) {
    let file_path = SYNC_DIR.join(name).join("db.yml");

    let pkg = Manifest::from_file(file_path);

    if let Some(deps) = pkg.depends {
        for dep in deps.iter() {
            DEPGRAPH
                .lock()
                .unwrap()
                .register_dependency(name.to_string(), dep.to_string());
            register_deps(dep)
        }
    }
}

pub fn install_list(names: Vec<String>) -> Vec<String> {
    let mut apps: Vec<String> = Vec::new();

    for name in names.iter() {
        register_deps(name);
    }

    for name in names.iter() {
        for node in DEPGRAPH
            .lock()
            .unwrap()
            .dependencies_of(&name.to_string())
            .unwrap()
        {
            apps.push(node.unwrap().clone());
        }
    }

    apps
}

// pub fn resolve_deps(name: &str) -> Vec<String> {
// let file_path = LOCAL_DIR.join(name).join("manifest.yml");

// let pkg = Manifest::from_file(file_path);

// let mut to_install: Vec<String> = Vec::new();
// if let Some(deps) = pkg.depends {
//     for dep in deps.iter() {
//         if !is_installed(dep) {}
//     }
// }
// }

// pub fn install(name: &str) {
//     let deps = resolve_deps(name);
//     if !deps.is_empty() {
//         deps.iter().for_each(|dep| {
//             if !is_installed(dep) {
//                 install(dep)
//             }
//         })
//     }
// }

// fn resolve_dependencies() {
//     // Create a new empty DepGraph.
//     let mut depgraph: DepGraph<&str> = DepGraph::new();

//     // You can register a dependency like this.  Solvent will automatically create nodes for any
//     // term it has not seen before.  This means 'b' depends on 'd'
//     depgraph.register_dependency("b", "d");

//     // You can also register multiple dependencies at once
//     depgraph.register_dependencies("a", vec!["b", "c", "d"]);
//     depgraph.register_dependencies("c", vec!["e"]);

//     // Iterate through each dependency of "a".  The dependencies will be returned in an order such
//     // that each output only depends on the previous outputs (or nothing).  The target itself will
//     // be output last.
// for node in depgraph.dependencies_of(&"a").unwrap() {
//     print!("{} ", node.unwrap());
// }
// }
