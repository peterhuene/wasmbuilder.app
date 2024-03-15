mod bindings;

use anyhow::Result;
use bindings::exports::wasmbuilder_app::graph::provider::{
    Component, ComponentId, EncodeOptions, Export, Guest, GuestGraph, Import, InstanceId, ItemKind,
};
use std::cell::RefCell;
use wasm_compose::graph::CompositionGraph;
use wasmparser::{ComponentExternalKind, ComponentTypeRef};
use wit_component::WitPrinter;

pub struct Graph {
    graph: RefCell<CompositionGraph<'static>>,
}

bindings::export!(Graph with_types_in bindings);

impl Guest for Graph {
    type Graph = Self;
}

impl GuestGraph for Graph {
    fn new() -> Self {
        Self {
            graph: RefCell::new(CompositionGraph::new()),
        }
    }

    fn add_component(&self, name: String, bytes: Vec<u8>) -> Result<Component, String> {
        let component = wasm_compose::graph::Component::from_bytes(name, bytes)
            .map_err(|e| format!("{e:#}"))?;

        let mut graph = self.graph.borrow_mut();

        let id = graph
            .add_component(component)
            .map_err(|e| format!("{e:#}"))?;

        let component = graph.get_component(id).unwrap();
        let wit = match wit_component::decode(component.bytes()) {
            Ok(decoded) => {
                // Print the wit for the component
                let resolve = decoded.resolve();
                let mut printer = WitPrinter::default();
                let mut wit = String::new();
                for (i, (id, _)) in resolve.packages.iter().enumerate() {
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
                .map(|(_, name, ty)| Import {
                    name: name.to_string(),
                    kind: match ty {
                        ComponentTypeRef::Module(_) => ItemKind::Module,
                        ComponentTypeRef::Func(_) => ItemKind::Function,
                        ComponentTypeRef::Value(_) => ItemKind::Value,
                        ComponentTypeRef::Type(_) => ItemKind::Type,
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
            wit,
        })
    }

    fn instantiate_component(&self, id: ComponentId) -> Result<InstanceId, String> {
        self.graph
            .borrow_mut()
            .instantiate(id as usize)
            .map(|id| id.0 as u32)
            .map_err(|e| format!("{e:#}"))
    }

    fn connect_instances(
        &self,
        source: InstanceId,
        source_export: Option<u32>,
        target: InstanceId,
        target_import: u32,
    ) -> Result<(), String> {
        self.graph
            .borrow_mut()
            .connect(
                source as usize,
                source_export.map(|e| e as usize),
                target as usize,
                target_import as usize,
            )
            .map_err(|e| format!("{e:#}"))
    }

    fn remove_component(&self, id: ComponentId) {
        self.graph.borrow_mut().remove_component(id as usize);
    }

    fn remove_instance(&self, id: InstanceId) {
        self.graph.borrow_mut().remove_instance(id as usize);
    }

    fn disconnect_instances(
        &self,
        source: InstanceId,
        target: InstanceId,
        target_import: u32,
    ) -> Result<(), String> {
        self.graph
            .borrow_mut()
            .disconnect(source as usize, target as usize, target_import as usize)
            .map_err(|e| format!("{e:#}"))
    }

    fn print_graph(&self) -> String {
        format!("{:#?}", self.graph.borrow())
    }

    fn encode_graph(&self, options: EncodeOptions) -> Result<Vec<u8>, String> {
        self.graph
            .borrow()
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
