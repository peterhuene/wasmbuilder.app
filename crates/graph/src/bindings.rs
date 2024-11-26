#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod wasmbuilder_app {
        #[allow(dead_code)]
        pub mod graph {
            #[allow(dead_code, clippy::all)]
            pub mod provider {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// Represents a kind of import or export in a WebAssembly component.
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                pub enum ItemKind {
                    /// The item is a core module.
                    Module,
                    /// The item is a function.
                    Function,
                    /// The item is a value.
                    Value,
                    /// The item is a type.
                    Type,
                    /// The item is an instance.
                    Instance,
                    /// The item is a component.
                    Component,
                }
                impl ::core::fmt::Debug for ItemKind {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            ItemKind::Module => {
                                f.debug_tuple("ItemKind::Module").finish()
                            }
                            ItemKind::Function => {
                                f.debug_tuple("ItemKind::Function").finish()
                            }
                            ItemKind::Value => f.debug_tuple("ItemKind::Value").finish(),
                            ItemKind::Type => f.debug_tuple("ItemKind::Type").finish(),
                            ItemKind::Instance => {
                                f.debug_tuple("ItemKind::Instance").finish()
                            }
                            ItemKind::Component => {
                                f.debug_tuple("ItemKind::Component").finish()
                            }
                        }
                    }
                }
                impl ItemKind {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> ItemKind {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => ItemKind::Module,
                            1 => ItemKind::Function,
                            2 => ItemKind::Value,
                            3 => ItemKind::Type,
                            4 => ItemKind::Instance,
                            5 => ItemKind::Component,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                /// Represents an import in a WebAssembly component.
                #[derive(Clone)]
                pub struct Import {
                    /// The import name.
                    pub name: _rt::String,
                    /// The import kind.
                    pub kind: ItemKind,
                }
                impl ::core::fmt::Debug for Import {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Import")
                            .field("name", &self.name)
                            .field("kind", &self.kind)
                            .finish()
                    }
                }
                /// Represents an export in a WebAssembly component.
                #[derive(Clone)]
                pub struct Export {
                    /// The export name.
                    pub name: _rt::String,
                    /// The export kind.
                    pub kind: ItemKind,
                }
                impl ::core::fmt::Debug for Export {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Export")
                            .field("name", &self.name)
                            .field("kind", &self.kind)
                            .finish()
                    }
                }
                /// Represents a component identifier in the graph.
                pub type ComponentId = u32;
                /// Represents a WebAssembly component.
                #[derive(Clone)]
                pub struct Component {
                    /// The id of the component in the graph/
                    pub id: ComponentId,
                    /// The name of the component.
                    pub name: _rt::String,
                    /// The imports of the component.
                    pub imports: _rt::Vec<Import>,
                    /// The exports of the component.
                    pub exports: _rt::Vec<Export>,
                    /// The WIT definition of the component's world.
                    pub wit: _rt::String,
                }
                impl ::core::fmt::Debug for Component {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Component")
                            .field("id", &self.id)
                            .field("name", &self.name)
                            .field("imports", &self.imports)
                            .field("exports", &self.exports)
                            .field("wit", &self.wit)
                            .finish()
                    }
                }
                /// Represents an instance identifier in the graph.
                pub type InstanceId = u32;
                /// Represents options for encoding the graph.
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct EncodeOptions {
                    /// Whether or not to define components in the output.
                    pub define_components: bool,
                    /// The instance to export from the output.
                    pub export: Option<InstanceId>,
                    /// Whether or not to validate the output.
                    pub validate: bool,
                }
                impl ::core::fmt::Debug for EncodeOptions {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("EncodeOptions")
                            .field("define-components", &self.define_components)
                            .field("export", &self.export)
                            .field("validate", &self.validate)
                            .finish()
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Graph {
                    handle: _rt::Resource<Graph>,
                }
                type _GraphRep<T> = Option<T>;
                impl Graph {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Graph`.
                    pub fn new<T: GuestGraph>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _GraphRep<T> = Some(val);
                        let ptr: *mut _GraphRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestGraph>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestGraph>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestGraph>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _GraphRep<T>);
                    }
                    fn as_ptr<T: GuestGraph>(&self) -> *mut _GraphRep<T> {
                        Graph::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Graph`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct GraphBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Graph>,
                }
                impl<'a> GraphBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestGraph>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _GraphRep<T> {
                        Graph::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Graph {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]wasmbuilder-app:graph/provider"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]graph"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_graph_cabi<T: GuestGraph>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = Graph::new(T::new());
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_graph_add_component_cabi<T: GuestGraph>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let result2 = T::add_component(
                        GraphBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        _rt::Vec::from_raw_parts(arg3.cast(), len1, len1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let Component {
                                id: id4,
                                name: name4,
                                imports: imports4,
                                exports: exports4,
                                wit: wit4,
                            } = e;
                            *ptr3.add(4).cast::<i32>() = _rt::as_i32(id4);
                            let vec5 = (name4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(12).cast::<usize>() = len5;
                            *ptr3.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                            let vec8 = imports4;
                            let len8 = vec8.len();
                            let layout8 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec8.len() * 12,
                                4,
                            );
                            let result8 = if layout8.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout8).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout8);
                                }
                                ptr
                            } else {
                                { ::core::ptr::null_mut() }
                            };
                            for (i, e) in vec8.into_iter().enumerate() {
                                let base = result8.add(i * 12);
                                {
                                    let Import { name: name6, kind: kind6 } = e;
                                    let vec7 = (name6.into_bytes()).into_boxed_slice();
                                    let ptr7 = vec7.as_ptr().cast::<u8>();
                                    let len7 = vec7.len();
                                    ::core::mem::forget(vec7);
                                    *base.add(4).cast::<usize>() = len7;
                                    *base.add(0).cast::<*mut u8>() = ptr7.cast_mut();
                                    *base.add(8).cast::<u8>() = (kind6.clone() as i32) as u8;
                                }
                            }
                            *ptr3.add(20).cast::<usize>() = len8;
                            *ptr3.add(16).cast::<*mut u8>() = result8;
                            let vec11 = exports4;
                            let len11 = vec11.len();
                            let layout11 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec11.len() * 12,
                                4,
                            );
                            let result11 = if layout11.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout11);
                                }
                                ptr
                            } else {
                                { ::core::ptr::null_mut() }
                            };
                            for (i, e) in vec11.into_iter().enumerate() {
                                let base = result11.add(i * 12);
                                {
                                    let Export { name: name9, kind: kind9 } = e;
                                    let vec10 = (name9.into_bytes()).into_boxed_slice();
                                    let ptr10 = vec10.as_ptr().cast::<u8>();
                                    let len10 = vec10.len();
                                    ::core::mem::forget(vec10);
                                    *base.add(4).cast::<usize>() = len10;
                                    *base.add(0).cast::<*mut u8>() = ptr10.cast_mut();
                                    *base.add(8).cast::<u8>() = (kind9.clone() as i32) as u8;
                                }
                            }
                            *ptr3.add(28).cast::<usize>() = len11;
                            *ptr3.add(24).cast::<*mut u8>() = result11;
                            let vec12 = (wit4.into_bytes()).into_boxed_slice();
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            ::core::mem::forget(vec12);
                            *ptr3.add(36).cast::<usize>() = len12;
                            *ptr3.add(32).cast::<*mut u8>() = ptr12.cast_mut();
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let vec13 = (e.into_bytes()).into_boxed_slice();
                            let ptr13 = vec13.as_ptr().cast::<u8>();
                            let len13 = vec13.len();
                            ::core::mem::forget(vec13);
                            *ptr3.add(8).cast::<usize>() = len13;
                            *ptr3.add(4).cast::<*mut u8>() = ptr13.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_graph_add_component<T: GuestGraph>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                            let l3 = *arg0.add(16).cast::<*mut u8>();
                            let l4 = *arg0.add(20).cast::<usize>();
                            let base7 = l3;
                            let len7 = l4;
                            for i in 0..len7 {
                                let base = base7.add(i * 12);
                                {
                                    let l5 = *base.add(0).cast::<*mut u8>();
                                    let l6 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                }
                            }
                            _rt::cabi_dealloc(base7, len7 * 12, 4);
                            let l8 = *arg0.add(24).cast::<*mut u8>();
                            let l9 = *arg0.add(28).cast::<usize>();
                            let base12 = l8;
                            let len12 = l9;
                            for i in 0..len12 {
                                let base = base12.add(i * 12);
                                {
                                    let l10 = *base.add(0).cast::<*mut u8>();
                                    let l11 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l10, l11, 1);
                                }
                            }
                            _rt::cabi_dealloc(base12, len12 * 12, 4);
                            let l13 = *arg0.add(32).cast::<*mut u8>();
                            let l14 = *arg0.add(36).cast::<usize>();
                            _rt::cabi_dealloc(l13, l14, 1);
                        }
                        _ => {
                            let l15 = *arg0.add(4).cast::<*mut u8>();
                            let l16 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l15, l16, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_graph_instantiate_component_cabi<
                    T: GuestGraph,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::instantiate_component(
                        GraphBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(4).cast::<i32>() = _rt::as_i32(e);
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_graph_instantiate_component<
                    T: GuestGraph,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_graph_connect_instances_cabi<T: GuestGraph>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::connect_instances(
                        GraphBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        match arg2 {
                            0 => None,
                            1 => {
                                let e = arg3 as u32;
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        arg4 as u32,
                        arg5 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_graph_connect_instances<
                    T: GuestGraph,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_graph_remove_component_cabi<T: GuestGraph>(
                    arg0: *mut u8,
                    arg1: i32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::remove_component(
                        GraphBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_graph_remove_instance_cabi<T: GuestGraph>(
                    arg0: *mut u8,
                    arg1: i32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::remove_instance(
                        GraphBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_graph_disconnect_instances_cabi<
                    T: GuestGraph,
                >(arg0: *mut u8, arg1: i32, arg2: i32, arg3: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::disconnect_instances(
                        GraphBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u32,
                        arg3 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_graph_disconnect_instances<
                    T: GuestGraph,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_graph_print_graph_cabi<T: GuestGraph>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::print_graph(
                        GraphBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_graph_print_graph<T: GuestGraph>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_graph_encode_graph_cabi<T: GuestGraph>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::encode_graph(
                        GraphBorrow::lift(arg0 as u32 as usize).get(),
                        EncodeOptions {
                            define_components: _rt::bool_lift(arg1 as u8),
                            export: match arg2 {
                                0 => None,
                                1 => {
                                    let e = arg3 as u32;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            validate: _rt::bool_lift(arg4 as u8),
                        },
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(8).cast::<usize>() = len3;
                            *ptr1.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_graph_encode_graph<T: GuestGraph>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(4).cast::<*mut u8>();
                            let l5 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                pub trait Guest {
                    type Graph: GuestGraph;
                }
                pub trait GuestGraph: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]wasmbuilder-app:graph/provider"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]graph"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]wasmbuilder-app:graph/provider"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]graph"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    /// Constructs a new graph.
                    fn new() -> Self;
                    /// Adds a component to the graph.
                    fn add_component(
                        &self,
                        name: _rt::String,
                        bytes: _rt::Vec<u8>,
                    ) -> Result<Component, _rt::String>;
                    /// Instantiates a component in the graph.
                    fn instantiate_component(
                        &self,
                        id: ComponentId,
                    ) -> Result<InstanceId, _rt::String>;
                    /// Connects two instances in the graph.
                    fn connect_instances(
                        &self,
                        source: InstanceId,
                        source_export: Option<u32>,
                        target: InstanceId,
                        target_import: u32,
                    ) -> Result<(), _rt::String>;
                    /// Remove a component from the graph.
                    fn remove_component(&self, id: ComponentId);
                    /// Remove an instance from the graph.
                    fn remove_instance(&self, id: InstanceId);
                    /// Disconnect connected instances in the graph.
                    fn disconnect_instances(
                        &self,
                        source: InstanceId,
                        target: InstanceId,
                        target_import: u32,
                    ) -> Result<(), _rt::String>;
                    /// Print the current graph state.
                    fn print_graph(&self) -> _rt::String;
                    /// Encode the current graph state as a new component.
                    fn encode_graph(
                        &self,
                        options: EncodeOptions,
                    ) -> Result<_rt::Vec<u8>, _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_wasmbuilder_app_graph_provider_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "wasmbuilder-app:graph/provider#[constructor]graph"] unsafe
                        extern "C" fn export_constructor_graph() -> i32 {
                        $($path_to_types)*:: _export_constructor_graph_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > () } #[export_name =
                        "wasmbuilder-app:graph/provider#[method]graph.add-component"]
                        unsafe extern "C" fn export_method_graph_add_component(arg0 : *
                        mut u8, arg1 : * mut u8, arg2 : usize, arg3 : * mut u8, arg4 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_graph_add_component_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_wasmbuilder-app:graph/provider#[method]graph.add-component"]
                        unsafe extern "C" fn _post_return_method_graph_add_component(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_graph_add_component::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0) } #[export_name =
                        "wasmbuilder-app:graph/provider#[method]graph.instantiate-component"]
                        unsafe extern "C" fn
                        export_method_graph_instantiate_component(arg0 : * mut u8, arg1 :
                        i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_graph_instantiate_component_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_wasmbuilder-app:graph/provider#[method]graph.instantiate-component"]
                        unsafe extern "C" fn
                        _post_return_method_graph_instantiate_component(arg0 : * mut u8,)
                        { $($path_to_types)*::
                        __post_return_method_graph_instantiate_component::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0) } #[export_name =
                        "wasmbuilder-app:graph/provider#[method]graph.connect-instances"]
                        unsafe extern "C" fn export_method_graph_connect_instances(arg0 :
                        * mut u8, arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 :
                        i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_graph_connect_instances_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0, arg1, arg2, arg3,
                        arg4, arg5) } #[export_name =
                        "cabi_post_wasmbuilder-app:graph/provider#[method]graph.connect-instances"]
                        unsafe extern "C" fn
                        _post_return_method_graph_connect_instances(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_graph_connect_instances::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0) } #[export_name =
                        "wasmbuilder-app:graph/provider#[method]graph.remove-component"]
                        unsafe extern "C" fn export_method_graph_remove_component(arg0 :
                        * mut u8, arg1 : i32,) { $($path_to_types)*::
                        _export_method_graph_remove_component_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0, arg1) }
                        #[export_name =
                        "wasmbuilder-app:graph/provider#[method]graph.remove-instance"]
                        unsafe extern "C" fn export_method_graph_remove_instance(arg0 : *
                        mut u8, arg1 : i32,) { $($path_to_types)*::
                        _export_method_graph_remove_instance_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0, arg1) }
                        #[export_name =
                        "wasmbuilder-app:graph/provider#[method]graph.disconnect-instances"]
                        unsafe extern "C" fn
                        export_method_graph_disconnect_instances(arg0 : * mut u8, arg1 :
                        i32, arg2 : i32, arg3 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_graph_disconnect_instances_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0, arg1, arg2, arg3) }
                        #[export_name =
                        "cabi_post_wasmbuilder-app:graph/provider#[method]graph.disconnect-instances"]
                        unsafe extern "C" fn
                        _post_return_method_graph_disconnect_instances(arg0 : * mut u8,)
                        { $($path_to_types)*::
                        __post_return_method_graph_disconnect_instances::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0) } #[export_name =
                        "wasmbuilder-app:graph/provider#[method]graph.print-graph"]
                        unsafe extern "C" fn export_method_graph_print_graph(arg0 : * mut
                        u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_graph_print_graph_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0) } #[export_name =
                        "cabi_post_wasmbuilder-app:graph/provider#[method]graph.print-graph"]
                        unsafe extern "C" fn _post_return_method_graph_print_graph(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_graph_print_graph::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0) } #[export_name =
                        "wasmbuilder-app:graph/provider#[method]graph.encode-graph"]
                        unsafe extern "C" fn export_method_graph_encode_graph(arg0 : *
                        mut u8, arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32,) -> * mut
                        u8 { $($path_to_types)*::
                        _export_method_graph_encode_graph_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_wasmbuilder-app:graph/provider#[method]graph.encode-graph"]
                        unsafe extern "C" fn _post_return_method_graph_encode_graph(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_graph_encode_graph::<<$ty as
                        $($path_to_types)*:: Guest >::Graph > (arg0) } const _ : () = {
                        #[doc(hidden)] #[export_name =
                        "wasmbuilder-app:graph/provider#[dtor]graph"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Graph::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Graph > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_wasmbuilder_app_graph_provider_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 40]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 40],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub use alloc_crate::alloc;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_component_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::wasmbuilder_app::graph::provider::__export_wasmbuilder_app_graph_provider_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::wasmbuilder_app::graph::provider);
    };
}
#[doc(inline)]
pub(crate) use __export_component_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:component:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1034] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x8a\x07\x01A\x02\x01\
A\x02\x01B,\x01m\x06\x06module\x08function\x05value\x04type\x08instance\x09compo\
nent\x04\0\x09item-kind\x03\0\0\x01r\x02\x04names\x04kind\x01\x04\0\x06import\x03\
\0\x02\x01r\x02\x04names\x04kind\x01\x04\0\x06export\x03\0\x04\x01y\x04\0\x0ccom\
ponent-id\x03\0\x06\x01p\x03\x01p\x05\x01r\x05\x02id\x07\x04names\x07imports\x08\
\x07exports\x09\x03wits\x04\0\x09component\x03\0\x0a\x01y\x04\0\x0binstance-id\x03\
\0\x0c\x01k\x0d\x01r\x03\x11define-components\x7f\x06export\x0e\x08validate\x7f\x04\
\0\x0eencode-options\x03\0\x0f\x04\0\x05graph\x03\x01\x01i\x11\x01@\0\0\x12\x04\0\
\x12[constructor]graph\x01\x13\x01h\x11\x01p}\x01j\x01\x0b\x01s\x01@\x03\x04self\
\x14\x04names\x05bytes\x15\0\x16\x04\0\x1b[method]graph.add-component\x01\x17\x01\
j\x01\x0d\x01s\x01@\x02\x04self\x14\x02id\x07\0\x18\x04\0#[method]graph.instanti\
ate-component\x01\x19\x01ky\x01j\0\x01s\x01@\x05\x04self\x14\x06source\x0d\x0dso\
urce-export\x1a\x06target\x0d\x0dtarget-importy\0\x1b\x04\0\x1f[method]graph.con\
nect-instances\x01\x1c\x01@\x02\x04self\x14\x02id\x07\x01\0\x04\0\x1e[method]gra\
ph.remove-component\x01\x1d\x01@\x02\x04self\x14\x02id\x0d\x01\0\x04\0\x1d[metho\
d]graph.remove-instance\x01\x1e\x01@\x04\x04self\x14\x06source\x0d\x06target\x0d\
\x0dtarget-importy\0\x1b\x04\0\"[method]graph.disconnect-instances\x01\x1f\x01@\x01\
\x04self\x14\0s\x04\0\x19[method]graph.print-graph\x01\x20\x01j\x01\x15\x01s\x01\
@\x02\x04self\x14\x07options\x10\0!\x04\0\x1a[method]graph.encode-graph\x01\"\x04\
\x01\x1ewasmbuilder-app:graph/provider\x05\0\x04\x01\x1fwasmbuilder-app:graph/co\
mponent\x04\0\x0b\x0f\x01\0\x09component\x03\0\0\0G\x09producers\x01\x0cprocesse\
d-by\x02\x0dwit-component\x070.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
