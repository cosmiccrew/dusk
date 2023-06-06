use std::borrow::Cow;
use std::path::Path;
use std::{fs, io};

use anyhow::Result;
use gltf::json::{self, Node};
use gltf::{Glb, Gltf};

const INPUT_PATH: &'static str = "assets/spacekit/old_models/";
const OUTPUT_PATH: &'static str = "assets/spacekit/models/";

fn main() -> Result<()> {
    //create the dir if it does not exist: we don't care if this errors here
    //(either the dir already exists which is fine, or with a permission error it
    //will be caught in the next few lines)
    let _ = std::fs::create_dir(OUTPUT_PATH);

    for entry in fs::read_dir(INPUT_PATH)? {
        let path = entry?.path();

        if !path.is_file() {
            panic!("all items in the directory should be files!");
        }

        //SAFETY: should be safe to unwrap as all paths are files.
        let stem = path.file_stem().unwrap().to_str().unwrap();

        let (original_glb, original_gltf) = read_glb_gltf(&path)?;

        let mut root = original_gltf.document.into_json();

        fix_gltf(&mut root);

        let glb = gltf::binary::Glb {
            header: original_glb.header,
            bin: original_glb.bin,
            json: Cow::Owned(json::serialize::to_string(&root)?.into_bytes()),
        };
        write_glb_with_stem(glb, stem);
    }

    Ok(())
}

fn fix_gltf(root: &mut json::Root) {
    //kenny.nl models have an unused "tmpParent" first node, so we overwrite this
    // with the actual first node
    root.scenes[0].nodes[0] = json::Index::new(0);

    root.nodes = root.nodes.clone()[1..].to_vec();

    //set the random offset to 0
    root.nodes[0].translation = None;

    let node_len = root.nodes.len();

    //iterate over the nodes, changing the node index's by -1 (as we removed the
    // first node)
    for node in 0..node_len {
        if let Some(children) = &root.nodes[node].children {
            let mut vec: Vec<json::Index<Node>> = vec![];

            for child in children {
                vec.push(json::Index::new((child.value() - 1) as u32))
            }

            root.nodes[node].children = Some(vec)
        }
    }
}

///we must read this file two times: once as Glb (above) to preserve the binary
/// data and header, and another time (below) as GlTF to get the json
/// interpretation we can edit. This isn't ideal but I can't be asked to do it
/// better. [this link](https://github.com/gltf-rs/gltf/blob/main/examples/export/main.rs)
/// provides a better way, if needed in future.
fn read_glb_gltf<P: AsRef<Path> + Clone>(path: P) -> Result<(Glb<'static>, Gltf)> {
    Ok((read_glb(&path)?, read_gltf(&path)?))
}

fn read_glb<P: AsRef<Path>>(path: P) -> Result<Glb<'static>> {
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);

    Ok(gltf::Glb::from_reader(reader)?)
}

fn read_gltf<P: AsRef<Path>>(path: P) -> Result<Gltf> {
    Ok(gltf::Gltf::open(&path)?)
}

fn write_glb_with_stem(glb: Glb, stem: &str) {
    let path = format!("{OUTPUT_PATH}{stem}.glb");
    write_glb_to_path(glb, path);
}

fn write_glb_to_path<P: AsRef<Path> + std::fmt::Debug>(glb: Glb, path: P) {
    let writer = std::fs::File::create(&path)
        .expect(&format!("I/O error: failed to create file at {:?}", &path));

    glb.to_writer(writer)
        .expect(&format!("glTF binary output error for {:?}", &path));
}
