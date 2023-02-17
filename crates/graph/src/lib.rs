use anyhow::Result;
use bindings::graph::{
    Component, ComponentId, EncodeOptions, Export, Graph, Import, InstanceId, ItemKind,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_compose::graph::CompositionGraph;
use wasmparser::{ComponentExternalKind, ComponentTypeRef};
use wit_component::DocumentPrinter;

static GRAPH: Lazy<Mutex<CompositionGraph>> = Lazy::new(Default::default);

struct GraphComponent;

impl Graph for GraphComponent {
    fn add_component(name: String, bytes: Vec<u8>) -> Result<Component, String> {
        let component = wasm_compose::graph::Component::from_bytes(name, bytes)
            .map_err(|e| format!("{e:#}"))?;

        let mut graph = GRAPH.lock().unwrap();

        let id = graph
            .add_component(component)
            .map_err(|e| format!("{e:#}"))?;

        let component = graph.get_component(id).unwrap();
        let wit = match wit_component::decode(component.name(), component.bytes()) {
            Ok(decoded) => {
                // Print the wit for the component
                let resolve = decoded.resolve();
                let mut printer = DocumentPrinter::default();
                let mut wit = String::new();
                for (i, (id, _)) in resolve.documents.iter().enumerate() {
                    if i > 0 {
                        wit.push_str("\n\n");
                    }
                    match printer.print(resolve, id) {
                        Ok(s) => wit.push_str(&s),
                        Err(e) => {
                            // If we can't print the document, just use the error text
                            wit = format!("{e:#}");
                            break;
                        }
                    }
                }
                wit
            }
            Err(e) => {
                // If we can't decode the component, just use the error text
                format!("{e:#}")
            }
        };

        Ok(Component {
            id: id.0 as u32,
            name: component.name().to_string(),
            imports: component
                .imports()
                .map(|(_, name, _, ty)| Import {
                    name: name.to_string(),
                    kind: match ty {
                        ComponentTypeRef::Module(_) => ItemKind::Module,
                        ComponentTypeRef::Func(_) => ItemKind::Function,
                        ComponentTypeRef::Value(_) => ItemKind::Value,
                        ComponentTypeRef::Type(_, _) => ItemKind::Type,
                        ComponentTypeRef::Instance(_) => ItemKind::Instance,
                        ComponentTypeRef::Component(_) => ItemKind::Component,
                    },
                })
                .collect(),
            exports: component
                .exports()
                .map(|(_, name, _, kind, _)| Export {
                    name: name.to_string(),
                    kind: match kind {
                        ComponentExternalKind::Module => ItemKind::Module,
                        ComponentExternalKind::Func => ItemKind::Function,
                        ComponentExternalKind::Value => ItemKind::Value,
                        ComponentExternalKind::Type => ItemKind::Type,
                        ComponentExternalKind::Instance => ItemKind::Instance,
                        ComponentExternalKind::Component => ItemKind::Component,
                    },
                })
                .collect(),
            wit,
        })
    }

    fn instantiate_component(id: ComponentId) -> Result<InstanceId, String> {
        GRAPH
            .lock()
            .unwrap()
            .instantiate(id as usize)
            .map(|id| id.0 as u32)
            .map_err(|e| format!("{e:#}"))
    }

    fn connect_instances(
        source: InstanceId,
        source_export: Option<u32>,
        target: InstanceId,
        target_import: u32,
    ) -> Result<(), String> {
        GRAPH
            .lock()
            .unwrap()
            .connect(
                source as usize,
                source_export.map(|e| e as usize),
                target as usize,
                target_import as usize,
            )
            .map_err(|e| format!("{e:#}"))
    }

    fn remove_component(id: ComponentId) {
        GRAPH.lock().unwrap().remove_component(id as usize);
    }

    fn remove_instance(id: InstanceId) {
        GRAPH.lock().unwrap().remove_instance(id as usize);
    }

    fn disconnect_instances(
        source: InstanceId,
        target: InstanceId,
        target_import: u32,
    ) -> Result<(), String> {
        GRAPH
            .lock()
            .unwrap()
            .disconnect(source as usize, target as usize, target_import as usize)
            .map_err(|e| format!("{e:#}"))
    }

    fn print_graph() -> String {
        format!("{:#?}", GRAPH.lock().unwrap())
    }

    fn encode_graph(options: EncodeOptions) -> Result<Vec<u8>, String> {
        GRAPH
            .lock()
            .unwrap()
            .encode(wasm_compose::graph::EncodeOptions {
                define_components: options.define_components,
                export: options
                    .export
                    .map(|i| wasm_compose::graph::InstanceId(i as usize)),
                validate: options.validate,
            })
            .map_err(|e| format!("{e:#}"))
    }
}

bindings::export!(GraphComponent);
