use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{ser::SerializeMap, ser::SerializeSeq, Serialize, Serializer};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_compose::graph::{Component, ComponentId, CompositionGraph, EncodeOptions};
use wasmparser::{ComponentExternalKind, ComponentTypeRef};
use wit_component::{decode_component_interfaces, InterfacePrinter};

static GRAPH: Lazy<Mutex<CompositionGraph>> = Lazy::new(Default::default);

struct ImportsSerializer<'a>(&'a Component<'a>);

impl<'a> Serialize for ImportsSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct ImportSerializer<'a>(&'a str, ComponentTypeRef);
        impl<'a> Serialize for ImportSerializer<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("name", self.0)?;
                map.serialize_entry(
                    "kind",
                    match self.1 {
                        ComponentTypeRef::Module(_) => "module",
                        ComponentTypeRef::Func(_) => "function",
                        ComponentTypeRef::Value(_) => "value",
                        ComponentTypeRef::Type(_, _) => "type",
                        ComponentTypeRef::Instance(_) => "instance",
                        ComponentTypeRef::Component(_) => "component",
                    },
                )?;
                map.end()
            }
        }

        let imports = self.0.imports();
        let mut seq = serializer.serialize_seq(Some(imports.len()))?;
        for (_, name, kind) in imports {
            seq.serialize_element(&ImportSerializer(name, kind))?;
        }

        seq.end()
    }
}

struct ExportsSerializer<'a>(&'a Component<'a>);

impl<'a> Serialize for ExportsSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct ExportSerializer<'a>(&'a str, ComponentExternalKind);
        impl<'a> Serialize for ExportSerializer<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("name", self.0)?;
                map.serialize_entry(
                    "kind",
                    match self.1 {
                        ComponentExternalKind::Module => "module",
                        ComponentExternalKind::Func => "function",
                        ComponentExternalKind::Value => "value",
                        ComponentExternalKind::Type => "type",
                        ComponentExternalKind::Instance => "instance",
                        ComponentExternalKind::Component => "component",
                    },
                )?;
                map.end()
            }
        }

        let exports = self.0.exports();
        let mut seq = serializer.serialize_seq(Some(exports.len()))?;
        for (_, name, kind, _) in self.0.exports() {
            seq.serialize_element(&ExportSerializer(name, kind))?;
        }

        seq.end()
    }
}

struct ComponentSerializer<'a>(ComponentId, &'a Component<'a>);

impl Serialize for ComponentSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.0 .0)?;
        map.serialize_entry("name", self.1.name())?;
        map.serialize_entry("imports", &ImportsSerializer(self.1))?;
        map.serialize_entry("exports", &ExportsSerializer(self.1))?;

        let interfaces =
            decode_component_interfaces(self.1.bytes()).map_err(serde::ser::Error::custom)?;

        if let Some(default) = &interfaces.default {
            let mut printer = InterfacePrinter::default();
            map.serialize_entry(
                "interface",
                printer
                    .print(default)
                    .map_err(serde::ser::Error::custom)?
                    .trim(),
            )?;
        }
        map.end()
    }
}

#[wasm_bindgen(js_name = "addComponent")]
pub fn add_component(name: String, bytes: Vec<u8>) -> Result<JsValue, JsValue> {
    let component =
        Component::from_bytes(name, bytes).map_err(|e| JsValue::from(format!("{e:#}")))?;

    let mut graph = GRAPH.lock().unwrap();

    let id = graph
        .add_component(component)
        .map_err(|e| JsValue::from(format!("{e:#}")))?;

    ComponentSerializer(id, graph.get_component(id).unwrap())
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsValue::from(format!("{e:#}")))
}

#[wasm_bindgen(js_name = "instantiateComponent")]
pub fn instantiate_component(id: usize) -> Result<usize, JsValue> {
    GRAPH
        .lock()
        .unwrap()
        .instantiate(id)
        .map(|id| id.0)
        .map_err(|e| JsValue::from(format!("{e:#}")))
}

#[wasm_bindgen(js_name = "connectInstances")]
pub fn connect_instances(
    source: usize,
    source_export: Option<usize>,
    target: usize,
    target_import: usize,
) -> Result<(), JsValue> {
    GRAPH
        .lock()
        .unwrap()
        .connect(source, source_export, target, target_import)
        .map_err(|e| JsValue::from(format!("{e:#}")))
}

#[wasm_bindgen(js_name = "removeComponent")]
pub fn remove_component(id: usize) {
    GRAPH.lock().unwrap().remove_component(id)
}

#[wasm_bindgen(js_name = "removeInstance")]
pub fn remove_instance(id: usize) {
    GRAPH.lock().unwrap().remove_instance(id)
}

#[wasm_bindgen(js_name = "disconnectInstances")]
pub fn disconnect_instances(
    source: usize,
    target: usize,
    target_import: usize,
) -> Result<(), JsValue> {
    GRAPH
        .lock()
        .unwrap()
        .disconnect(source, target, target_import)
        .map_err(|e| JsValue::from(format!("{e:#}")))
}

#[wasm_bindgen(js_name = "printGraph")]
pub fn print_graph() -> String {
    format!("{:#?}", GRAPH.lock().unwrap())
}

#[wasm_bindgen(js_name = "encodeGraph")]
pub fn encode_graph(define_components: bool, export: Option<usize>) -> Result<Vec<u8>, JsValue> {
    GRAPH
        .lock()
        .unwrap()
        .encode(EncodeOptions {
            define_components,
            export: export.map(Into::into),
            validate: true,
        })
        .map_err(|e| JsValue::from(format!("{e:#}")))
}
