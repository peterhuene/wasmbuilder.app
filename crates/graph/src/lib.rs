use anyhow::Result;
use bindings::interface::{
    Component, ComponentId, EncodeOptions, Export, Import, InstanceId, Interface, ItemKind,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_compose::graph::CompositionGraph;
use wasmparser::{ComponentExternalKind, ComponentTypeRef};
use wit_component::{decode_component_interfaces, InterfacePrinter};

static GRAPH: Lazy<Mutex<CompositionGraph>> = Lazy::new(Default::default);

struct GraphComponent;

impl Interface for GraphComponent {
    fn add_component(name: String, bytes: Vec<u8>) -> Result<Component, String> {
        let component = wasm_compose::graph::Component::from_bytes(name, bytes)
            .map_err(|e| format!("{e:#}"))?;

        let mut graph = GRAPH.lock().unwrap();

        let id = graph
            .add_component(component)
            .map_err(|e| format!("{e:#}"))?;

        let component = graph.get_component(id).unwrap();

        let interfaces =
            decode_component_interfaces(component.bytes()).map_err(|e| format!("{e:#}"))?;

        Ok(Component {
            id: id.0 as u32,
            name: component.name().to_string(),
            imports: component
                .imports()
                .map(|(_, name, ty)| Import {
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
                .map(|(_, name, kind, _)| Export {
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
            interface: interfaces
                .default
                .map(|i| {
                    let mut printer = InterfacePrinter::default();
                    printer.print(&i)
                })
                .transpose()
                .map_err(|e| format!("{e:#}"))?,
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
