use gltf::json::{self, Node};
use std::{borrow::Cow, fs, io};

fn main() -> io::Result<()> {
    for entry in fs::read_dir("assets/spacekit/models")? {
        let path = entry?.path();

        let stem = path.file_stem().unwrap().to_str().unwrap();

        let reader = io::BufReader::new(fs::File::open(path.clone())?);

        let original_glb = gltf::Glb::from_reader(reader).unwrap();

        let mut root = gltf::Gltf::open(path.clone()).unwrap().document.into_json();

        root.scenes[0].nodes[0] = json::Index::new(0);

        root.nodes = root.nodes.clone()[1..].to_vec();

        root.nodes[0].translation = Some([0., 0., 0.]);

        let node_len = root.nodes.len();

        for node in 0..node_len {
            if let Some(children) = &root.nodes[node].children {
                let mut vec: Vec<json::Index<Node>> = vec![];

                for child in children {
                    vec.push(json::Index::new((child.value() - 1) as u32))
                }

                root.nodes[node].children = Some(vec)
            }
        }

        let json_string = json::serialize::to_string(&root).expect("Serialization error");
        let glb = gltf::binary::Glb {
            header: original_glb.header,
            bin: original_glb.bin,
            json: Cow::Owned(json_string.into_bytes()),
        };

        let _ = std::fs::create_dir("assets/new_models");
        let writer =
            std::fs::File::create(format!("assets/new_models/{stem}.glb")).expect("I/O error");
        glb.to_writer(writer)
            .expect(&format!("glTF binary output error for {stem}.glb"));
    }

    Ok(())
}
