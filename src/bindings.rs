#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod jcbhmr {
        #[allow(dead_code)]
        pub mod lexopt {
            #[allow(dead_code, clippy::all)]
            pub mod lexopt {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// Docs for PARSER
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Parser {
                    handle: _rt::Resource<Parser>,
                }
                type _ParserRep<T> = Option<T>;
                impl Parser {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Parser`.
                    pub fn new<T: GuestParser>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ParserRep<T> = Some(val);
                        let ptr: *mut _ParserRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestParser>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestParser>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestParser>(self) -> T {
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
                        let _ = _rt::Box::from_raw(handle as *mut _ParserRep<T>);
                    }
                    fn as_ptr<T: GuestParser>(&self) -> *mut _ParserRep<T> {
                        Parser::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Parser`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ParserBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Parser>,
                }
                impl<'a> ParserBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestParser>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _ParserRep<T> {
                        Parser::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Parser {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-drop]parser"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct RawArgs {
                    handle: _rt::Resource<RawArgs>,
                }
                type _RawArgsRep<T> = Option<T>;
                impl RawArgs {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `RawArgs`.
                    pub fn new<T: GuestRawArgs>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _RawArgsRep<T> = Some(val);
                        let ptr: *mut _RawArgsRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestRawArgs>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestRawArgs>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestRawArgs>(self) -> T {
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
                        let _ = _rt::Box::from_raw(handle as *mut _RawArgsRep<T>);
                    }
                    fn as_ptr<T: GuestRawArgs>(&self) -> *mut _RawArgsRep<T> {
                        RawArgs::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`RawArgs`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct RawArgsBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a RawArgs>,
                }
                impl<'a> RawArgsBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestRawArgs>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _RawArgsRep<T> {
                        RawArgs::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for RawArgs {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-drop]raw-args"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ValuesIter {
                    handle: _rt::Resource<ValuesIter>,
                }
                type _ValuesIterRep<T> = Option<T>;
                impl ValuesIter {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `ValuesIter`.
                    pub fn new<T: GuestValuesIter>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ValuesIterRep<T> = Some(val);
                        let ptr: *mut _ValuesIterRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestValuesIter>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestValuesIter>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestValuesIter>(self) -> T {
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
                        let _ = _rt::Box::from_raw(handle as *mut _ValuesIterRep<T>);
                    }
                    fn as_ptr<T: GuestValuesIter>(&self) -> *mut _ValuesIterRep<T> {
                        ValuesIter::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`ValuesIter`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ValuesIterBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a ValuesIter>,
                }
                impl<'a> ValuesIterBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestValuesIter>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _ValuesIterRep<T> {
                        ValuesIter::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for ValuesIter {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-drop]values-iter"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Clone)]
                pub enum Arg {
                    Short(char),
                    Long(_rt::String),
                    Value(_rt::Vec<u8>),
                }
                impl ::core::fmt::Debug for Arg {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Arg::Short(e) => {
                                f.debug_tuple("Arg::Short").field(e).finish()
                            }
                            Arg::Long(e) => f.debug_tuple("Arg::Long").field(e).finish(),
                            Arg::Value(e) => {
                                f.debug_tuple("Arg::Value").field(e).finish()
                            }
                        }
                    }
                }
                #[derive(Clone)]
                pub struct ErrorMissingValue {
                    pub option: Option<_rt::String>,
                }
                impl ::core::fmt::Debug for ErrorMissingValue {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ErrorMissingValue")
                            .field("option", &self.option)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct ErrorUnexpectedValue {
                    pub option: _rt::String,
                    pub value: _rt::Vec<u8>,
                }
                impl ::core::fmt::Debug for ErrorUnexpectedValue {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ErrorUnexpectedValue")
                            .field("option", &self.option)
                            .field("value", &self.value)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct ErrorParsingFailed {
                    pub value: _rt::String,
                    pub error: _rt::String,
                }
                impl ::core::fmt::Debug for ErrorParsingFailed {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ErrorParsingFailed")
                            .field("value", &self.value)
                            .field("error", &self.error)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub enum Error {
                    MissingValue(ErrorMissingValue),
                    UnexpectedOption(_rt::String),
                    UnexpectedArgument(_rt::Vec<u8>),
                    UnexpectedValue(ErrorUnexpectedValue),
                    ParsingFailed(ErrorParsingFailed),
                    NonUnicodeValue(_rt::Vec<u8>),
                    Custom(_rt::String),
                }
                impl ::core::fmt::Debug for Error {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Error::MissingValue(e) => {
                                f.debug_tuple("Error::MissingValue").field(e).finish()
                            }
                            Error::UnexpectedOption(e) => {
                                f.debug_tuple("Error::UnexpectedOption").field(e).finish()
                            }
                            Error::UnexpectedArgument(e) => {
                                f.debug_tuple("Error::UnexpectedArgument").field(e).finish()
                            }
                            Error::UnexpectedValue(e) => {
                                f.debug_tuple("Error::UnexpectedValue").field(e).finish()
                            }
                            Error::ParsingFailed(e) => {
                                f.debug_tuple("Error::ParsingFailed").field(e).finish()
                            }
                            Error::NonUnicodeValue(e) => {
                                f.debug_tuple("Error::NonUnicodeValue").field(e).finish()
                            }
                            Error::Custom(e) => {
                                f.debug_tuple("Error::Custom").field(e).finish()
                            }
                        }
                    }
                }
                impl ::core::fmt::Display for Error {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl std::error::Error for Error {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_bin_name_cabi<T: GuestParser>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::bin_name(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_parser_bin_name<T: GuestParser>(
                    arg0: *mut u8,
                ) {
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
                pub unsafe fn _export_static_parser_from_args_cabi<T: GuestParser>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base3 = arg0;
                    let len3 = arg1;
                    let mut result3 = _rt::Vec::with_capacity(len3);
                    for i in 0..len3 {
                        let base = base3.add(i * 8);
                        let e3 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let len2 = l1;
                            _rt::Vec::from_raw_parts(l0.cast(), len2, len2)
                        };
                        result3.push(e3);
                    }
                    _rt::cabi_dealloc(base3, len3 * 8, 4);
                    let result4 = T::from_args(result3);
                    (result4).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_static_parser_from_env_cabi<T: GuestParser>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::from_env();
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_static_parser_from_iter_cabi<T: GuestParser>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base3 = arg0;
                    let len3 = arg1;
                    let mut result3 = _rt::Vec::with_capacity(len3);
                    for i in 0..len3 {
                        let base = base3.add(i * 8);
                        let e3 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let len2 = l1;
                            _rt::Vec::from_raw_parts(l0.cast(), len2, len2)
                        };
                        result3.push(e3);
                    }
                    _rt::cabi_dealloc(base3, len3 * 8, 4);
                    let result4 = T::from_iter(result3);
                    (result4).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_next_cabi<T: GuestParser>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::next(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            match e {
                                Some(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    match e {
                                        Arg::Short(e) => {
                                            *ptr1.add(8).cast::<u8>() = (0i32) as u8;
                                            *ptr1.add(12).cast::<i32>() = _rt::as_i32(e);
                                        }
                                        Arg::Long(e) => {
                                            *ptr1.add(8).cast::<u8>() = (1i32) as u8;
                                            let vec2 = (e.into_bytes()).into_boxed_slice();
                                            let ptr2 = vec2.as_ptr().cast::<u8>();
                                            let len2 = vec2.len();
                                            ::core::mem::forget(vec2);
                                            *ptr1.add(16).cast::<usize>() = len2;
                                            *ptr1.add(12).cast::<*mut u8>() = ptr2.cast_mut();
                                        }
                                        Arg::Value(e) => {
                                            *ptr1.add(8).cast::<u8>() = (2i32) as u8;
                                            let vec3 = (e).into_boxed_slice();
                                            let ptr3 = vec3.as_ptr().cast::<u8>();
                                            let len3 = vec3.len();
                                            ::core::mem::forget(vec3);
                                            *ptr1.add(16).cast::<usize>() = len3;
                                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                                        }
                                    }
                                }
                                None => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            match e {
                                Error::MissingValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                    let ErrorMissingValue { option: option4 } = e;
                                    match option4 {
                                        Some(e) => {
                                            *ptr1.add(8).cast::<u8>() = (1i32) as u8;
                                            let vec5 = (e.into_bytes()).into_boxed_slice();
                                            let ptr5 = vec5.as_ptr().cast::<u8>();
                                            let len5 = vec5.len();
                                            ::core::mem::forget(vec5);
                                            *ptr1.add(16).cast::<usize>() = len5;
                                            *ptr1.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                                        }
                                        None => {
                                            *ptr1.add(8).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                }
                                Error::UnexpectedOption(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec6 = (e.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *ptr1.add(12).cast::<usize>() = len6;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                                Error::UnexpectedArgument(e) => {
                                    *ptr1.add(4).cast::<u8>() = (2i32) as u8;
                                    let vec7 = (e).into_boxed_slice();
                                    let ptr7 = vec7.as_ptr().cast::<u8>();
                                    let len7 = vec7.len();
                                    ::core::mem::forget(vec7);
                                    *ptr1.add(12).cast::<usize>() = len7;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr7.cast_mut();
                                }
                                Error::UnexpectedValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (3i32) as u8;
                                    let ErrorUnexpectedValue {
                                        option: option8,
                                        value: value8,
                                    } = e;
                                    let vec9 = (option8.into_bytes()).into_boxed_slice();
                                    let ptr9 = vec9.as_ptr().cast::<u8>();
                                    let len9 = vec9.len();
                                    ::core::mem::forget(vec9);
                                    *ptr1.add(12).cast::<usize>() = len9;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr9.cast_mut();
                                    let vec10 = (value8).into_boxed_slice();
                                    let ptr10 = vec10.as_ptr().cast::<u8>();
                                    let len10 = vec10.len();
                                    ::core::mem::forget(vec10);
                                    *ptr1.add(20).cast::<usize>() = len10;
                                    *ptr1.add(16).cast::<*mut u8>() = ptr10.cast_mut();
                                }
                                Error::ParsingFailed(e) => {
                                    *ptr1.add(4).cast::<u8>() = (4i32) as u8;
                                    let ErrorParsingFailed { value: value11, error: error11 } = e;
                                    let vec12 = (value11.into_bytes()).into_boxed_slice();
                                    let ptr12 = vec12.as_ptr().cast::<u8>();
                                    let len12 = vec12.len();
                                    ::core::mem::forget(vec12);
                                    *ptr1.add(12).cast::<usize>() = len12;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr12.cast_mut();
                                    let vec13 = (error11.into_bytes()).into_boxed_slice();
                                    let ptr13 = vec13.as_ptr().cast::<u8>();
                                    let len13 = vec13.len();
                                    ::core::mem::forget(vec13);
                                    *ptr1.add(20).cast::<usize>() = len13;
                                    *ptr1.add(16).cast::<*mut u8>() = ptr13.cast_mut();
                                }
                                Error::NonUnicodeValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (5i32) as u8;
                                    let vec14 = (e).into_boxed_slice();
                                    let ptr14 = vec14.as_ptr().cast::<u8>();
                                    let len14 = vec14.len();
                                    ::core::mem::forget(vec14);
                                    *ptr1.add(12).cast::<usize>() = len14;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr14.cast_mut();
                                }
                                Error::Custom(e) => {
                                    *ptr1.add(4).cast::<u8>() = (6i32) as u8;
                                    let vec15 = (e.into_bytes()).into_boxed_slice();
                                    let ptr15 = vec15.as_ptr().cast::<u8>();
                                    let len15 = vec15.len();
                                    ::core::mem::forget(vec15);
                                    *ptr1.add(12).cast::<usize>() = len15;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr15.cast_mut();
                                }
                            }
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_parser_next<T: GuestParser>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = i32::from(*arg0.add(4).cast::<u8>());
                            match l1 {
                                0 => {}
                                _ => {
                                    let l2 = i32::from(*arg0.add(8).cast::<u8>());
                                    match l2 {
                                        0 => {}
                                        1 => {
                                            let l3 = *arg0.add(12).cast::<*mut u8>();
                                            let l4 = *arg0.add(16).cast::<usize>();
                                            _rt::cabi_dealloc(l3, l4, 1);
                                        }
                                        _ => {
                                            let l5 = *arg0.add(12).cast::<*mut u8>();
                                            let l6 = *arg0.add(16).cast::<usize>();
                                            let base7 = l5;
                                            let len7 = l6;
                                            _rt::cabi_dealloc(base7, len7 * 1, 1);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            let l8 = i32::from(*arg0.add(4).cast::<u8>());
                            match l8 {
                                0 => {
                                    let l9 = i32::from(*arg0.add(8).cast::<u8>());
                                    match l9 {
                                        0 => {}
                                        _ => {
                                            let l10 = *arg0.add(12).cast::<*mut u8>();
                                            let l11 = *arg0.add(16).cast::<usize>();
                                            _rt::cabi_dealloc(l10, l11, 1);
                                        }
                                    }
                                }
                                1 => {
                                    let l12 = *arg0.add(8).cast::<*mut u8>();
                                    let l13 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l12, l13, 1);
                                }
                                2 => {
                                    let l14 = *arg0.add(8).cast::<*mut u8>();
                                    let l15 = *arg0.add(12).cast::<usize>();
                                    let base16 = l14;
                                    let len16 = l15;
                                    _rt::cabi_dealloc(base16, len16 * 1, 1);
                                }
                                3 => {
                                    let l17 = *arg0.add(8).cast::<*mut u8>();
                                    let l18 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l17, l18, 1);
                                    let l19 = *arg0.add(16).cast::<*mut u8>();
                                    let l20 = *arg0.add(20).cast::<usize>();
                                    let base21 = l19;
                                    let len21 = l20;
                                    _rt::cabi_dealloc(base21, len21 * 1, 1);
                                }
                                4 => {
                                    let l22 = *arg0.add(8).cast::<*mut u8>();
                                    let l23 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l22, l23, 1);
                                    let l24 = *arg0.add(16).cast::<*mut u8>();
                                    let l25 = *arg0.add(20).cast::<usize>();
                                    _rt::cabi_dealloc(l24, l25, 1);
                                }
                                5 => {
                                    let l26 = *arg0.add(8).cast::<*mut u8>();
                                    let l27 = *arg0.add(12).cast::<usize>();
                                    let base28 = l26;
                                    let len28 = l27;
                                    _rt::cabi_dealloc(base28, len28 * 1, 1);
                                }
                                _ => {
                                    let l29 = *arg0.add(8).cast::<*mut u8>();
                                    let l30 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l29, l30, 1);
                                }
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_optional_value_cabi<T: GuestParser>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::optional_value(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_parser_optional_value<T: GuestParser>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_raw_args_cabi<T: GuestParser>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::raw_args(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            match e {
                                Error::MissingValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                    let ErrorMissingValue { option: option2 } = e;
                                    match option2 {
                                        Some(e) => {
                                            *ptr1.add(8).cast::<u8>() = (1i32) as u8;
                                            let vec3 = (e.into_bytes()).into_boxed_slice();
                                            let ptr3 = vec3.as_ptr().cast::<u8>();
                                            let len3 = vec3.len();
                                            ::core::mem::forget(vec3);
                                            *ptr1.add(16).cast::<usize>() = len3;
                                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                                        }
                                        None => {
                                            *ptr1.add(8).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                }
                                Error::UnexpectedOption(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec4 = (e.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *ptr1.add(12).cast::<usize>() = len4;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                }
                                Error::UnexpectedArgument(e) => {
                                    *ptr1.add(4).cast::<u8>() = (2i32) as u8;
                                    let vec5 = (e).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *ptr1.add(12).cast::<usize>() = len5;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                }
                                Error::UnexpectedValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (3i32) as u8;
                                    let ErrorUnexpectedValue {
                                        option: option6,
                                        value: value6,
                                    } = e;
                                    let vec7 = (option6.into_bytes()).into_boxed_slice();
                                    let ptr7 = vec7.as_ptr().cast::<u8>();
                                    let len7 = vec7.len();
                                    ::core::mem::forget(vec7);
                                    *ptr1.add(12).cast::<usize>() = len7;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr7.cast_mut();
                                    let vec8 = (value6).into_boxed_slice();
                                    let ptr8 = vec8.as_ptr().cast::<u8>();
                                    let len8 = vec8.len();
                                    ::core::mem::forget(vec8);
                                    *ptr1.add(20).cast::<usize>() = len8;
                                    *ptr1.add(16).cast::<*mut u8>() = ptr8.cast_mut();
                                }
                                Error::ParsingFailed(e) => {
                                    *ptr1.add(4).cast::<u8>() = (4i32) as u8;
                                    let ErrorParsingFailed { value: value9, error: error9 } = e;
                                    let vec10 = (value9.into_bytes()).into_boxed_slice();
                                    let ptr10 = vec10.as_ptr().cast::<u8>();
                                    let len10 = vec10.len();
                                    ::core::mem::forget(vec10);
                                    *ptr1.add(12).cast::<usize>() = len10;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr10.cast_mut();
                                    let vec11 = (error9.into_bytes()).into_boxed_slice();
                                    let ptr11 = vec11.as_ptr().cast::<u8>();
                                    let len11 = vec11.len();
                                    ::core::mem::forget(vec11);
                                    *ptr1.add(20).cast::<usize>() = len11;
                                    *ptr1.add(16).cast::<*mut u8>() = ptr11.cast_mut();
                                }
                                Error::NonUnicodeValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (5i32) as u8;
                                    let vec12 = (e).into_boxed_slice();
                                    let ptr12 = vec12.as_ptr().cast::<u8>();
                                    let len12 = vec12.len();
                                    ::core::mem::forget(vec12);
                                    *ptr1.add(12).cast::<usize>() = len12;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr12.cast_mut();
                                }
                                Error::Custom(e) => {
                                    *ptr1.add(4).cast::<u8>() = (6i32) as u8;
                                    let vec13 = (e.into_bytes()).into_boxed_slice();
                                    let ptr13 = vec13.as_ptr().cast::<u8>();
                                    let len13 = vec13.len();
                                    ::core::mem::forget(vec13);
                                    *ptr1.add(12).cast::<usize>() = len13;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr13.cast_mut();
                                }
                            }
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_parser_raw_args<T: GuestParser>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = i32::from(*arg0.add(4).cast::<u8>());
                            match l1 {
                                0 => {
                                    let l2 = i32::from(*arg0.add(8).cast::<u8>());
                                    match l2 {
                                        0 => {}
                                        _ => {
                                            let l3 = *arg0.add(12).cast::<*mut u8>();
                                            let l4 = *arg0.add(16).cast::<usize>();
                                            _rt::cabi_dealloc(l3, l4, 1);
                                        }
                                    }
                                }
                                1 => {
                                    let l5 = *arg0.add(8).cast::<*mut u8>();
                                    let l6 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                }
                                2 => {
                                    let l7 = *arg0.add(8).cast::<*mut u8>();
                                    let l8 = *arg0.add(12).cast::<usize>();
                                    let base9 = l7;
                                    let len9 = l8;
                                    _rt::cabi_dealloc(base9, len9 * 1, 1);
                                }
                                3 => {
                                    let l10 = *arg0.add(8).cast::<*mut u8>();
                                    let l11 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l10, l11, 1);
                                    let l12 = *arg0.add(16).cast::<*mut u8>();
                                    let l13 = *arg0.add(20).cast::<usize>();
                                    let base14 = l12;
                                    let len14 = l13;
                                    _rt::cabi_dealloc(base14, len14 * 1, 1);
                                }
                                4 => {
                                    let l15 = *arg0.add(8).cast::<*mut u8>();
                                    let l16 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l15, l16, 1);
                                    let l17 = *arg0.add(16).cast::<*mut u8>();
                                    let l18 = *arg0.add(20).cast::<usize>();
                                    _rt::cabi_dealloc(l17, l18, 1);
                                }
                                5 => {
                                    let l19 = *arg0.add(8).cast::<*mut u8>();
                                    let l20 = *arg0.add(12).cast::<usize>();
                                    let base21 = l19;
                                    let len21 = l20;
                                    _rt::cabi_dealloc(base21, len21 * 1, 1);
                                }
                                _ => {
                                    let l22 = *arg0.add(8).cast::<*mut u8>();
                                    let l23 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l22, l23, 1);
                                }
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_try_raw_args_cabi<T: GuestParser>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::try_raw_args(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_value_cabi<T: GuestParser>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::value(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
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
                            match e {
                                Error::MissingValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                    let ErrorMissingValue { option: option3 } = e;
                                    match option3 {
                                        Some(e) => {
                                            *ptr1.add(8).cast::<u8>() = (1i32) as u8;
                                            let vec4 = (e.into_bytes()).into_boxed_slice();
                                            let ptr4 = vec4.as_ptr().cast::<u8>();
                                            let len4 = vec4.len();
                                            ::core::mem::forget(vec4);
                                            *ptr1.add(16).cast::<usize>() = len4;
                                            *ptr1.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                                        }
                                        None => {
                                            *ptr1.add(8).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                }
                                Error::UnexpectedOption(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec5 = (e.into_bytes()).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *ptr1.add(12).cast::<usize>() = len5;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                }
                                Error::UnexpectedArgument(e) => {
                                    *ptr1.add(4).cast::<u8>() = (2i32) as u8;
                                    let vec6 = (e).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *ptr1.add(12).cast::<usize>() = len6;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                                Error::UnexpectedValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (3i32) as u8;
                                    let ErrorUnexpectedValue {
                                        option: option7,
                                        value: value7,
                                    } = e;
                                    let vec8 = (option7.into_bytes()).into_boxed_slice();
                                    let ptr8 = vec8.as_ptr().cast::<u8>();
                                    let len8 = vec8.len();
                                    ::core::mem::forget(vec8);
                                    *ptr1.add(12).cast::<usize>() = len8;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr8.cast_mut();
                                    let vec9 = (value7).into_boxed_slice();
                                    let ptr9 = vec9.as_ptr().cast::<u8>();
                                    let len9 = vec9.len();
                                    ::core::mem::forget(vec9);
                                    *ptr1.add(20).cast::<usize>() = len9;
                                    *ptr1.add(16).cast::<*mut u8>() = ptr9.cast_mut();
                                }
                                Error::ParsingFailed(e) => {
                                    *ptr1.add(4).cast::<u8>() = (4i32) as u8;
                                    let ErrorParsingFailed { value: value10, error: error10 } = e;
                                    let vec11 = (value10.into_bytes()).into_boxed_slice();
                                    let ptr11 = vec11.as_ptr().cast::<u8>();
                                    let len11 = vec11.len();
                                    ::core::mem::forget(vec11);
                                    *ptr1.add(12).cast::<usize>() = len11;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr11.cast_mut();
                                    let vec12 = (error10.into_bytes()).into_boxed_slice();
                                    let ptr12 = vec12.as_ptr().cast::<u8>();
                                    let len12 = vec12.len();
                                    ::core::mem::forget(vec12);
                                    *ptr1.add(20).cast::<usize>() = len12;
                                    *ptr1.add(16).cast::<*mut u8>() = ptr12.cast_mut();
                                }
                                Error::NonUnicodeValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (5i32) as u8;
                                    let vec13 = (e).into_boxed_slice();
                                    let ptr13 = vec13.as_ptr().cast::<u8>();
                                    let len13 = vec13.len();
                                    ::core::mem::forget(vec13);
                                    *ptr1.add(12).cast::<usize>() = len13;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr13.cast_mut();
                                }
                                Error::Custom(e) => {
                                    *ptr1.add(4).cast::<u8>() = (6i32) as u8;
                                    let vec14 = (e.into_bytes()).into_boxed_slice();
                                    let ptr14 = vec14.as_ptr().cast::<u8>();
                                    let len14 = vec14.len();
                                    ::core::mem::forget(vec14);
                                    *ptr1.add(12).cast::<usize>() = len14;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr14.cast_mut();
                                }
                            }
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_parser_value<T: GuestParser>(
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
                            let l4 = i32::from(*arg0.add(4).cast::<u8>());
                            match l4 {
                                0 => {
                                    let l5 = i32::from(*arg0.add(8).cast::<u8>());
                                    match l5 {
                                        0 => {}
                                        _ => {
                                            let l6 = *arg0.add(12).cast::<*mut u8>();
                                            let l7 = *arg0.add(16).cast::<usize>();
                                            _rt::cabi_dealloc(l6, l7, 1);
                                        }
                                    }
                                }
                                1 => {
                                    let l8 = *arg0.add(8).cast::<*mut u8>();
                                    let l9 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l8, l9, 1);
                                }
                                2 => {
                                    let l10 = *arg0.add(8).cast::<*mut u8>();
                                    let l11 = *arg0.add(12).cast::<usize>();
                                    let base12 = l10;
                                    let len12 = l11;
                                    _rt::cabi_dealloc(base12, len12 * 1, 1);
                                }
                                3 => {
                                    let l13 = *arg0.add(8).cast::<*mut u8>();
                                    let l14 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l13, l14, 1);
                                    let l15 = *arg0.add(16).cast::<*mut u8>();
                                    let l16 = *arg0.add(20).cast::<usize>();
                                    let base17 = l15;
                                    let len17 = l16;
                                    _rt::cabi_dealloc(base17, len17 * 1, 1);
                                }
                                4 => {
                                    let l18 = *arg0.add(8).cast::<*mut u8>();
                                    let l19 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l18, l19, 1);
                                    let l20 = *arg0.add(16).cast::<*mut u8>();
                                    let l21 = *arg0.add(20).cast::<usize>();
                                    _rt::cabi_dealloc(l20, l21, 1);
                                }
                                5 => {
                                    let l22 = *arg0.add(8).cast::<*mut u8>();
                                    let l23 = *arg0.add(12).cast::<usize>();
                                    let base24 = l22;
                                    let len24 = l23;
                                    _rt::cabi_dealloc(base24, len24 * 1, 1);
                                }
                                _ => {
                                    let l25 = *arg0.add(8).cast::<*mut u8>();
                                    let l26 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l25, l26, 1);
                                }
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_values_cabi<T: GuestParser>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::values(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            match e {
                                Error::MissingValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                    let ErrorMissingValue { option: option2 } = e;
                                    match option2 {
                                        Some(e) => {
                                            *ptr1.add(8).cast::<u8>() = (1i32) as u8;
                                            let vec3 = (e.into_bytes()).into_boxed_slice();
                                            let ptr3 = vec3.as_ptr().cast::<u8>();
                                            let len3 = vec3.len();
                                            ::core::mem::forget(vec3);
                                            *ptr1.add(16).cast::<usize>() = len3;
                                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                                        }
                                        None => {
                                            *ptr1.add(8).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                }
                                Error::UnexpectedOption(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec4 = (e.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *ptr1.add(12).cast::<usize>() = len4;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                }
                                Error::UnexpectedArgument(e) => {
                                    *ptr1.add(4).cast::<u8>() = (2i32) as u8;
                                    let vec5 = (e).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *ptr1.add(12).cast::<usize>() = len5;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                }
                                Error::UnexpectedValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (3i32) as u8;
                                    let ErrorUnexpectedValue {
                                        option: option6,
                                        value: value6,
                                    } = e;
                                    let vec7 = (option6.into_bytes()).into_boxed_slice();
                                    let ptr7 = vec7.as_ptr().cast::<u8>();
                                    let len7 = vec7.len();
                                    ::core::mem::forget(vec7);
                                    *ptr1.add(12).cast::<usize>() = len7;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr7.cast_mut();
                                    let vec8 = (value6).into_boxed_slice();
                                    let ptr8 = vec8.as_ptr().cast::<u8>();
                                    let len8 = vec8.len();
                                    ::core::mem::forget(vec8);
                                    *ptr1.add(20).cast::<usize>() = len8;
                                    *ptr1.add(16).cast::<*mut u8>() = ptr8.cast_mut();
                                }
                                Error::ParsingFailed(e) => {
                                    *ptr1.add(4).cast::<u8>() = (4i32) as u8;
                                    let ErrorParsingFailed { value: value9, error: error9 } = e;
                                    let vec10 = (value9.into_bytes()).into_boxed_slice();
                                    let ptr10 = vec10.as_ptr().cast::<u8>();
                                    let len10 = vec10.len();
                                    ::core::mem::forget(vec10);
                                    *ptr1.add(12).cast::<usize>() = len10;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr10.cast_mut();
                                    let vec11 = (error9.into_bytes()).into_boxed_slice();
                                    let ptr11 = vec11.as_ptr().cast::<u8>();
                                    let len11 = vec11.len();
                                    ::core::mem::forget(vec11);
                                    *ptr1.add(20).cast::<usize>() = len11;
                                    *ptr1.add(16).cast::<*mut u8>() = ptr11.cast_mut();
                                }
                                Error::NonUnicodeValue(e) => {
                                    *ptr1.add(4).cast::<u8>() = (5i32) as u8;
                                    let vec12 = (e).into_boxed_slice();
                                    let ptr12 = vec12.as_ptr().cast::<u8>();
                                    let len12 = vec12.len();
                                    ::core::mem::forget(vec12);
                                    *ptr1.add(12).cast::<usize>() = len12;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr12.cast_mut();
                                }
                                Error::Custom(e) => {
                                    *ptr1.add(4).cast::<u8>() = (6i32) as u8;
                                    let vec13 = (e.into_bytes()).into_boxed_slice();
                                    let ptr13 = vec13.as_ptr().cast::<u8>();
                                    let len13 = vec13.len();
                                    ::core::mem::forget(vec13);
                                    *ptr1.add(12).cast::<usize>() = len13;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr13.cast_mut();
                                }
                            }
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_parser_values<T: GuestParser>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = i32::from(*arg0.add(4).cast::<u8>());
                            match l1 {
                                0 => {
                                    let l2 = i32::from(*arg0.add(8).cast::<u8>());
                                    match l2 {
                                        0 => {}
                                        _ => {
                                            let l3 = *arg0.add(12).cast::<*mut u8>();
                                            let l4 = *arg0.add(16).cast::<usize>();
                                            _rt::cabi_dealloc(l3, l4, 1);
                                        }
                                    }
                                }
                                1 => {
                                    let l5 = *arg0.add(8).cast::<*mut u8>();
                                    let l6 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                }
                                2 => {
                                    let l7 = *arg0.add(8).cast::<*mut u8>();
                                    let l8 = *arg0.add(12).cast::<usize>();
                                    let base9 = l7;
                                    let len9 = l8;
                                    _rt::cabi_dealloc(base9, len9 * 1, 1);
                                }
                                3 => {
                                    let l10 = *arg0.add(8).cast::<*mut u8>();
                                    let l11 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l10, l11, 1);
                                    let l12 = *arg0.add(16).cast::<*mut u8>();
                                    let l13 = *arg0.add(20).cast::<usize>();
                                    let base14 = l12;
                                    let len14 = l13;
                                    _rt::cabi_dealloc(base14, len14 * 1, 1);
                                }
                                4 => {
                                    let l15 = *arg0.add(8).cast::<*mut u8>();
                                    let l16 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l15, l16, 1);
                                    let l17 = *arg0.add(16).cast::<*mut u8>();
                                    let l18 = *arg0.add(20).cast::<usize>();
                                    _rt::cabi_dealloc(l17, l18, 1);
                                }
                                5 => {
                                    let l19 = *arg0.add(8).cast::<*mut u8>();
                                    let l20 = *arg0.add(12).cast::<usize>();
                                    let base21 = l19;
                                    let len21 = l20;
                                    _rt::cabi_dealloc(base21, len21 * 1, 1);
                                }
                                _ => {
                                    let l22 = *arg0.add(8).cast::<*mut u8>();
                                    let l23 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l22, l23, 1);
                                }
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_clone_cabi<T: GuestParser>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::clone(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_parser_clone_from_cabi<T: GuestParser>(
                    arg0: *mut u8,
                    arg1: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::clone_from(
                        ParserBorrow::lift(arg0 as u32 as usize).get(),
                        Parser::from_handle(arg1 as u32),
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_raw_args_as_slice_cabi<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::as_slice(
                        RawArgsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec3 = result0;
                    let len3 = vec3.len();
                    let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec3.len() * 8,
                        4,
                    );
                    let result3 = if layout3.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout3);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec3.into_iter().enumerate() {
                        let base = result3.add(i * 8);
                        {
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *base.add(4).cast::<usize>() = len2;
                            *base.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                    }
                    *ptr1.add(4).cast::<usize>() = len3;
                    *ptr1.add(0).cast::<*mut u8>() = result3;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_raw_args_as_slice<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base5 = l0;
                    let len5 = l1;
                    for i in 0..len5 {
                        let base = base5.add(i * 8);
                        {
                            let l2 = *base.add(0).cast::<*mut u8>();
                            let l3 = *base.add(4).cast::<usize>();
                            let base4 = l2;
                            let len4 = l3;
                            _rt::cabi_dealloc(base4, len4 * 1, 1);
                        }
                    }
                    _rt::cabi_dealloc(base5, len5 * 8, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_raw_args_peek_cabi<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::peek(
                        RawArgsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_raw_args_peek<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_raw_args_next_cabi<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::next(
                        RawArgsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_raw_args_next<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_raw_args_size_hint_cabi<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::size_hint(
                        RawArgsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let (t2_0, t2_1) = result0;
                    *ptr1.add(0).cast::<i32>() = _rt::as_i32(t2_0);
                    match t2_1 {
                        Some(e) => {
                            *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(8).cast::<i32>() = _rt::as_i32(e);
                        }
                        None => {
                            *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_raw_args_count_cabi<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::count(
                        RawArgsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_raw_args_last_cabi<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::last(
                        RawArgsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_raw_args_last<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_raw_args_nth_cabi<T: GuestRawArgs>(
                    arg0: *mut u8,
                    arg1: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::nth(
                        RawArgsBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_raw_args_nth<T: GuestRawArgs>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_arg_unexpected_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let v2 = match arg0 {
                        0 => {
                            let e2 = _rt::char_lift(arg1 as i32 as u32);
                            Arg::Short(e2)
                        }
                        1 => {
                            let e2 = {
                                let len0 = arg2;
                                let bytes0 = _rt::Vec::from_raw_parts(
                                    arg1.cast(),
                                    len0,
                                    len0,
                                );
                                _rt::string_lift(bytes0)
                            };
                            Arg::Long(e2)
                        }
                        n => {
                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                            let e2 = {
                                let len1 = arg2;
                                _rt::Vec::from_raw_parts(arg1.cast(), len1, len1)
                            };
                            Arg::Value(e2)
                        }
                    };
                    let result3 = T::arg_unexpected(v2);
                    let ptr4 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result3 {
                        Error::MissingValue(e) => {
                            *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                            let ErrorMissingValue { option: option5 } = e;
                            match option5 {
                                Some(e) => {
                                    *ptr4.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec6 = (e.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *ptr4.add(12).cast::<usize>() = len6;
                                    *ptr4.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                                None => {
                                    *ptr4.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Error::UnexpectedOption(e) => {
                            *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                            let vec7 = (e.into_bytes()).into_boxed_slice();
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            ::core::mem::forget(vec7);
                            *ptr4.add(8).cast::<usize>() = len7;
                            *ptr4.add(4).cast::<*mut u8>() = ptr7.cast_mut();
                        }
                        Error::UnexpectedArgument(e) => {
                            *ptr4.add(0).cast::<u8>() = (2i32) as u8;
                            let vec8 = (e).into_boxed_slice();
                            let ptr8 = vec8.as_ptr().cast::<u8>();
                            let len8 = vec8.len();
                            ::core::mem::forget(vec8);
                            *ptr4.add(8).cast::<usize>() = len8;
                            *ptr4.add(4).cast::<*mut u8>() = ptr8.cast_mut();
                        }
                        Error::UnexpectedValue(e) => {
                            *ptr4.add(0).cast::<u8>() = (3i32) as u8;
                            let ErrorUnexpectedValue {
                                option: option9,
                                value: value9,
                            } = e;
                            let vec10 = (option9.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr4.add(8).cast::<usize>() = len10;
                            *ptr4.add(4).cast::<*mut u8>() = ptr10.cast_mut();
                            let vec11 = (value9).into_boxed_slice();
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            ::core::mem::forget(vec11);
                            *ptr4.add(16).cast::<usize>() = len11;
                            *ptr4.add(12).cast::<*mut u8>() = ptr11.cast_mut();
                        }
                        Error::ParsingFailed(e) => {
                            *ptr4.add(0).cast::<u8>() = (4i32) as u8;
                            let ErrorParsingFailed { value: value12, error: error12 } = e;
                            let vec13 = (value12.into_bytes()).into_boxed_slice();
                            let ptr13 = vec13.as_ptr().cast::<u8>();
                            let len13 = vec13.len();
                            ::core::mem::forget(vec13);
                            *ptr4.add(8).cast::<usize>() = len13;
                            *ptr4.add(4).cast::<*mut u8>() = ptr13.cast_mut();
                            let vec14 = (error12.into_bytes()).into_boxed_slice();
                            let ptr14 = vec14.as_ptr().cast::<u8>();
                            let len14 = vec14.len();
                            ::core::mem::forget(vec14);
                            *ptr4.add(16).cast::<usize>() = len14;
                            *ptr4.add(12).cast::<*mut u8>() = ptr14.cast_mut();
                        }
                        Error::NonUnicodeValue(e) => {
                            *ptr4.add(0).cast::<u8>() = (5i32) as u8;
                            let vec15 = (e).into_boxed_slice();
                            let ptr15 = vec15.as_ptr().cast::<u8>();
                            let len15 = vec15.len();
                            ::core::mem::forget(vec15);
                            *ptr4.add(8).cast::<usize>() = len15;
                            *ptr4.add(4).cast::<*mut u8>() = ptr15.cast_mut();
                        }
                        Error::Custom(e) => {
                            *ptr4.add(0).cast::<u8>() = (6i32) as u8;
                            let vec16 = (e.into_bytes()).into_boxed_slice();
                            let ptr16 = vec16.as_ptr().cast::<u8>();
                            let len16 = vec16.len();
                            ::core::mem::forget(vec16);
                            *ptr4.add(8).cast::<usize>() = len16;
                            *ptr4.add(4).cast::<*mut u8>() = ptr16.cast_mut();
                        }
                    }
                    ptr4
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_arg_unexpected<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = i32::from(*arg0.add(4).cast::<u8>());
                            match l1 {
                                0 => {}
                                _ => {
                                    let l2 = *arg0.add(8).cast::<*mut u8>();
                                    let l3 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l2, l3, 1);
                                }
                            }
                        }
                        1 => {
                            let l4 = *arg0.add(4).cast::<*mut u8>();
                            let l5 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                        2 => {
                            let l6 = *arg0.add(4).cast::<*mut u8>();
                            let l7 = *arg0.add(8).cast::<usize>();
                            let base8 = l6;
                            let len8 = l7;
                            _rt::cabi_dealloc(base8, len8 * 1, 1);
                        }
                        3 => {
                            let l9 = *arg0.add(4).cast::<*mut u8>();
                            let l10 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l9, l10, 1);
                            let l11 = *arg0.add(12).cast::<*mut u8>();
                            let l12 = *arg0.add(16).cast::<usize>();
                            let base13 = l11;
                            let len13 = l12;
                            _rt::cabi_dealloc(base13, len13 * 1, 1);
                        }
                        4 => {
                            let l14 = *arg0.add(4).cast::<*mut u8>();
                            let l15 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l14, l15, 1);
                            let l16 = *arg0.add(12).cast::<*mut u8>();
                            let l17 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l16, l17, 1);
                        }
                        5 => {
                            let l18 = *arg0.add(4).cast::<*mut u8>();
                            let l19 = *arg0.add(8).cast::<usize>();
                            let base20 = l18;
                            let len20 = l19;
                            _rt::cabi_dealloc(base20, len20 * 1, 1);
                        }
                        _ => {
                            let l21 = *arg0.add(4).cast::<*mut u8>();
                            let l22 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l21, l22, 1);
                        }
                    }
                }
                pub trait Guest {
                    type Parser: GuestParser;
                    type RawArgs: GuestRawArgs;
                    type ValuesIter: GuestValuesIter;
                    fn arg_unexpected(self_: Arg) -> Error;
                }
                pub trait GuestParser: 'static {
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
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-new]parser"]
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
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-rep]parser"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn bin_name(&self) -> Option<_rt::String>;
                    fn from_args(args: _rt::Vec<_rt::Vec<u8>>) -> Parser;
                    fn from_env() -> Parser;
                    fn from_iter(iter: _rt::Vec<_rt::Vec<u8>>) -> Parser;
                    fn next(&self) -> Result<Option<Arg>, Error>;
                    fn optional_value(&self) -> Option<_rt::Vec<u8>>;
                    fn raw_args(&self) -> Result<RawArgs, Error>;
                    fn try_raw_args(&self) -> Option<RawArgs>;
                    fn value(&self) -> Result<_rt::Vec<u8>, Error>;
                    fn values(&self) -> Result<ValuesIter, Error>;
                    /// Clone trait
                    fn clone(&self) -> Parser;
                    fn clone_from(&self, source: Parser) -> Parser;
                }
                pub trait GuestRawArgs: 'static {
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
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-new]raw-args"]
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
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-rep]raw-args"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn as_slice(&self) -> _rt::Vec<_rt::Vec<u8>>;
                    fn peek(&self) -> Option<_rt::Vec<u8>>;
                    /// Iterator trait
                    fn next(&self) -> Option<_rt::Vec<u8>>;
                    fn size_hint(&self) -> (u32, Option<u32>);
                    fn count(&self) -> u32;
                    fn last(&self) -> Option<_rt::Vec<u8>>;
                    fn nth(&self, n: u32) -> Option<_rt::Vec<u8>>;
                }
                pub trait GuestValuesIter: 'static {
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
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-new]values-iter"]
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
                            #[link(wasm_import_module = "[export]jcbhmr:lexopt/lexopt")]
                            extern "C" {
                                #[link_name = "[resource-rep]values-iter"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                }
                #[doc(hidden)]
                macro_rules! __export_jcbhmr_lexopt_lexopt_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]parser.bin-name"] unsafe extern "C"
                        fn export_method_parser_bin_name(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_parser_bin_name_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name
                        = "cabi_post_jcbhmr:lexopt/lexopt#[method]parser.bin-name"]
                        unsafe extern "C" fn _post_return_method_parser_bin_name(arg0 : *
                        mut u8,) { $($path_to_types)*::
                        __post_return_method_parser_bin_name::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[static]parser.from-args"] unsafe extern
                        "C" fn export_static_parser_from_args(arg0 : * mut u8, arg1 :
                        usize,) -> i32 { $($path_to_types)*::
                        _export_static_parser_from_args_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0, arg1) }
                        #[export_name = "jcbhmr:lexopt/lexopt#[static]parser.from-env"]
                        unsafe extern "C" fn export_static_parser_from_env() -> i32 {
                        $($path_to_types)*:: _export_static_parser_from_env_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Parser > () } #[export_name =
                        "jcbhmr:lexopt/lexopt#[static]parser.from-iter"] unsafe extern
                        "C" fn export_static_parser_from_iter(arg0 : * mut u8, arg1 :
                        usize,) -> i32 { $($path_to_types)*::
                        _export_static_parser_from_iter_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0, arg1) }
                        #[export_name = "jcbhmr:lexopt/lexopt#[method]parser.next"]
                        unsafe extern "C" fn export_method_parser_next(arg0 : * mut u8,)
                        -> * mut u8 { $($path_to_types)*::
                        _export_method_parser_next_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Parser > (arg0) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]parser.next"] unsafe
                        extern "C" fn _post_return_method_parser_next(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_method_parser_next::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]parser.optional-value"] unsafe
                        extern "C" fn export_method_parser_optional_value(arg0 : * mut
                        u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_parser_optional_value_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]parser.optional-value"]
                        unsafe extern "C" fn
                        _post_return_method_parser_optional_value(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_parser_optional_value::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]parser.raw-args"] unsafe extern "C"
                        fn export_method_parser_raw_args(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_parser_raw_args_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name
                        = "cabi_post_jcbhmr:lexopt/lexopt#[method]parser.raw-args"]
                        unsafe extern "C" fn _post_return_method_parser_raw_args(arg0 : *
                        mut u8,) { $($path_to_types)*::
                        __post_return_method_parser_raw_args::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]parser.try-raw-args"] unsafe extern
                        "C" fn export_method_parser_try_raw_args(arg0 : * mut u8,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_method_parser_try_raw_args_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]parser.value"] unsafe extern "C" fn
                        export_method_parser_value(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_parser_value_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]parser.value"] unsafe
                        extern "C" fn _post_return_method_parser_value(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_method_parser_value::<<$ty
                        as $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name
                        = "jcbhmr:lexopt/lexopt#[method]parser.values"] unsafe extern "C"
                        fn export_method_parser_values(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_parser_values_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]parser.values"] unsafe
                        extern "C" fn _post_return_method_parser_values(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_method_parser_values::<<$ty
                        as $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name
                        = "jcbhmr:lexopt/lexopt#[method]parser.clone"] unsafe extern "C"
                        fn export_method_parser_clone(arg0 : * mut u8,) -> i32 {
                        $($path_to_types)*:: _export_method_parser_clone_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]parser.clone-from"] unsafe extern
                        "C" fn export_method_parser_clone_from(arg0 : * mut u8, arg1 :
                        i32,) -> i32 { $($path_to_types)*::
                        _export_method_parser_clone_from_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Parser > (arg0, arg1) }
                        #[export_name = "jcbhmr:lexopt/lexopt#[method]raw-args.as-slice"]
                        unsafe extern "C" fn export_method_raw_args_as_slice(arg0 : * mut
                        u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_raw_args_as_slice_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]raw-args.as-slice"]
                        unsafe extern "C" fn _post_return_method_raw_args_as_slice(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_raw_args_as_slice::<<$ty as
                        $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]raw-args.peek"] unsafe extern "C"
                        fn export_method_raw_args_peek(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_raw_args_peek_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]raw-args.peek"] unsafe
                        extern "C" fn _post_return_method_raw_args_peek(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_method_raw_args_peek::<<$ty
                        as $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name
                        = "jcbhmr:lexopt/lexopt#[method]raw-args.next"] unsafe extern "C"
                        fn export_method_raw_args_next(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_raw_args_next_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]raw-args.next"] unsafe
                        extern "C" fn _post_return_method_raw_args_next(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_method_raw_args_next::<<$ty
                        as $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name
                        = "jcbhmr:lexopt/lexopt#[method]raw-args.size-hint"] unsafe
                        extern "C" fn export_method_raw_args_size_hint(arg0 : * mut u8,)
                        -> * mut u8 { $($path_to_types)*::
                        _export_method_raw_args_size_hint_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]raw-args.count"] unsafe extern "C"
                        fn export_method_raw_args_count(arg0 : * mut u8,) -> i32 {
                        $($path_to_types)*:: _export_method_raw_args_count_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name =
                        "jcbhmr:lexopt/lexopt#[method]raw-args.last"] unsafe extern "C"
                        fn export_method_raw_args_last(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_raw_args_last_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]raw-args.last"] unsafe
                        extern "C" fn _post_return_method_raw_args_last(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_method_raw_args_last::<<$ty
                        as $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name
                        = "jcbhmr:lexopt/lexopt#[method]raw-args.nth"] unsafe extern "C"
                        fn export_method_raw_args_nth(arg0 : * mut u8, arg1 : i32,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_method_raw_args_nth_cabi::<<$ty as $($path_to_types)*::
                        Guest >::RawArgs > (arg0, arg1) } #[export_name =
                        "cabi_post_jcbhmr:lexopt/lexopt#[method]raw-args.nth"] unsafe
                        extern "C" fn _post_return_method_raw_args_nth(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_method_raw_args_nth::<<$ty
                        as $($path_to_types)*:: Guest >::RawArgs > (arg0) } #[export_name
                        = "jcbhmr:lexopt/lexopt#arg-unexpected"] unsafe extern "C" fn
                        export_arg_unexpected(arg0 : i32, arg1 : * mut u8, arg2 : usize,)
                        -> * mut u8 { $($path_to_types)*::
                        _export_arg_unexpected_cabi::<$ty > (arg0, arg1, arg2) }
                        #[export_name = "cabi_post_jcbhmr:lexopt/lexopt#arg-unexpected"]
                        unsafe extern "C" fn _post_return_arg_unexpected(arg0 : * mut
                        u8,) { $($path_to_types)*:: __post_return_arg_unexpected::<$ty >
                        (arg0) } const _ : () = { #[doc(hidden)] #[export_name =
                        "jcbhmr:lexopt/lexopt#[dtor]parser"] #[allow(non_snake_case)]
                        unsafe extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        Parser::dtor::< <$ty as $($path_to_types)*:: Guest >::Parser >
                        (rep) } }; const _ : () = { #[doc(hidden)] #[export_name =
                        "jcbhmr:lexopt/lexopt#[dtor]raw-args"] #[allow(non_snake_case)]
                        unsafe extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        RawArgs::dtor::< <$ty as $($path_to_types)*:: Guest >::RawArgs >
                        (rep) } }; const _ : () = { #[doc(hidden)] #[export_name =
                        "jcbhmr:lexopt/lexopt#[dtor]values-iter"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: ValuesIter::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::ValuesIter > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_jcbhmr_lexopt_lexopt_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 24]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 24],
                );
            }
        }
    }
}
mod _rt {
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
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
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
    pub unsafe fn char_lift(val: u32) -> char {
        if cfg!(debug_assertions) {
            core::char::from_u32(val).unwrap()
        } else {
            core::char::from_u32_unchecked(val)
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
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
macro_rules! __export_lexopt_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::jcbhmr::lexopt::lexopt::__export_jcbhmr_lexopt_lexopt_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::jcbhmr::lexopt::lexopt);
    };
}
#[doc(inline)]
pub(crate) use __export_lexopt_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:jcbhmr:lexopt:lexopt-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1359] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xcc\x09\x01A\x02\x01\
A\x02\x01BD\x04\0\x06parser\x03\x01\x04\0\x08raw-args\x03\x01\x04\0\x0bvalues-it\
er\x03\x01\x01p}\x01q\x03\x05short\x01t\0\x04long\x01s\0\x05value\x01\x03\0\x04\0\
\x03arg\x03\0\x04\x01ks\x01r\x01\x06option\x06\x04\0\x13error-missing-value\x03\0\
\x07\x01r\x02\x06options\x05value\x03\x04\0\x16error-unexpected-value\x03\0\x09\x01\
r\x02\x05values\x05errors\x04\0\x14error-parsing-failed\x03\0\x0b\x01q\x07\x0dmi\
ssing-value\x01\x08\0\x11unexpected-option\x01s\0\x13unexpected-argument\x01\x03\
\0\x10unexpected-value\x01\x0a\0\x0eparsing-failed\x01\x0c\0\x11non-unicode-valu\
e\x01\x03\0\x06custom\x01s\0\x04\0\x05error\x03\0\x0d\x01h\0\x01@\x01\x04self\x0f\
\0\x06\x04\0\x17[method]parser.bin-name\x01\x10\x01p\x03\x01i\0\x01@\x01\x04args\
\x11\0\x12\x04\0\x18[static]parser.from-args\x01\x13\x01@\0\0\x12\x04\0\x17[stat\
ic]parser.from-env\x01\x14\x01@\x01\x04iter\x11\0\x12\x04\0\x18[static]parser.fr\
om-iter\x01\x15\x01k\x05\x01j\x01\x16\x01\x0e\x01@\x01\x04self\x0f\0\x17\x04\0\x13\
[method]parser.next\x01\x18\x01k\x03\x01@\x01\x04self\x0f\0\x19\x04\0\x1d[method\
]parser.optional-value\x01\x1a\x01i\x01\x01j\x01\x1b\x01\x0e\x01@\x01\x04self\x0f\
\0\x1c\x04\0\x17[method]parser.raw-args\x01\x1d\x01k\x1b\x01@\x01\x04self\x0f\0\x1e\
\x04\0\x1b[method]parser.try-raw-args\x01\x1f\x01j\x01\x03\x01\x0e\x01@\x01\x04s\
elf\x0f\0\x20\x04\0\x14[method]parser.value\x01!\x01i\x02\x01j\x01\"\x01\x0e\x01\
@\x01\x04self\x0f\0#\x04\0\x15[method]parser.values\x01$\x01@\x01\x04self\x0f\0\x12\
\x04\0\x14[method]parser.clone\x01%\x01@\x02\x04self\x0f\x06source\x12\0\x12\x04\
\0\x19[method]parser.clone-from\x01&\x01h\x01\x01@\x01\x04self'\0\x11\x04\0\x19[\
method]raw-args.as-slice\x01(\x01@\x01\x04self'\0\x19\x04\0\x15[method]raw-args.\
peek\x01)\x04\0\x15[method]raw-args.next\x01)\x01ky\x01o\x02y*\x01@\x01\x04self'\
\0+\x04\0\x1a[method]raw-args.size-hint\x01,\x01@\x01\x04self'\0y\x04\0\x16[meth\
od]raw-args.count\x01-\x04\0\x15[method]raw-args.last\x01)\x01@\x02\x04self'\x01\
ny\0\x19\x04\0\x14[method]raw-args.nth\x01.\x01@\x01\x04self\x05\0\x0e\x04\0\x0e\
arg-unexpected\x01/\x04\0\x14jcbhmr:lexopt/lexopt\x05\0\x04\0\x1ajcbhmr:lexopt/l\
exopt-world\x04\0\x0b\x12\x01\0\x0clexopt-world\x03\0\0\0G\x09producers\x01\x0cp\
rocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
