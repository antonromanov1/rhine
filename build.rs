extern crate yaml_rust;

use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::yaml;

static mut YAML_HASHES: u16 = 0;

fn dump_node(doc: &yaml::Yaml, indent: usize) {
    match *doc {
        yaml::Yaml::Array(ref v) => {
            for x in v {
                dump_node(x, indent + 1);
            }
        }
        yaml::Yaml::Hash(ref h) => {
            for (_k, v) in h {
                dump_node(v, indent + 1);
                unsafe {
                    YAML_HASHES = YAML_HASHES + 1;
                }
            }
        }
        _ => (),
    }
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let enum_opcode_path = Path::new(&out_dir).join("opcode.rs");
    let graph_path = Path::new(&out_dir).join("graph_create_inst.rs");

    let mut f = fs::File::open("src/instructions.yaml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
    for doc in &docs {
        dump_node(doc, 0);
    }

    let opcodes_count: u16;

    unsafe {
        opcodes_count = (YAML_HASHES - 1) / 3;
    }

    let mut enum_opcode =
        "#[derive(Clone, Copy, PartialEq, Debug)]\npub enum Opcode {\n".to_string();
    let mut graph_inst_creations = "impl Graph {\n".to_string();

    let mut opcodes = Vec::new();

    for i in 0..opcodes_count {
        let opcode = docs[0]["instructions"][i as usize]["opcode"]
            .as_str()
            .unwrap();
        let base = docs[0]["instructions"][i as usize]["base"]
            .as_str()
            .unwrap();

        opcodes.push(opcode.to_string());
        enum_opcode.push_str(&format!("  {},\n", opcode));

        graph_inst_creations.push_str(&format!(
            "
    pub fn create_inst_{}(&mut self) -> *mut {} {{
        self.inst_cur_id = self.inst_cur_id + 1;
        let inst : *mut {};
        unsafe {{
            inst = libc::malloc(std::mem::size_of::<{}>()) as *mut {};
            (*inst).set_id(self.inst_cur_id);
            (*inst).set_opcode(Opcode::{});
        }}
        self.instructions.push(inst);
        inst
    }}\n",
            opcode.to_lowercase(),
            base,
            base,
            base,
            base,
            opcode
        ));
    }

    enum_opcode
        .push_str("}\n\nfn get_opcode_string(opcode: Opcode) -> &'static str {\nmatch opcode {\n");
    for opcode in opcodes {
        enum_opcode.push_str(&format!("    Opcode::{} => \"{}\",\n", opcode, opcode));
    }
    enum_opcode.push_str("}\n}\n");

    graph_inst_creations.push_str("}\n");

    fs::write(&enum_opcode_path, &enum_opcode).unwrap();
    fs::write(&graph_path, &graph_inst_creations).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
