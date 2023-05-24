// use std::sync::{Arc, RwLock};
//
// use bevy::{
//     asset::{Asset, HandleId, LoadState},
//     prelude::*
// };
// use std::marker::PhantomData;
//
// pub struct CommonAssetPhantom;
// /// 判断资产是否已经加载完毕
// pub trait Loading {
//     fn loading(&mut self, server: &AssetServer) -> bool;
// }
//
// pub trait DefaultNew<T: Loading> {
//     fn new(v: T) -> Self;
// }
//
// #[derive(Resource)]
// pub struct CustomLoader<V: Loading, B>(
//     bool,
//     pub(crate) V,
//     ArcBool,
//     PhantomData<B>
// );
//
// pub type CommonAssetLoader<V> = CustomLoader<V,
// CommonAssetPhantom>;
//
// impl<V, B> DefaultNew<V> for CustomLoader<V, B>
// where
//     V: Loading
// {
//     fn new(v: V) -> Self {
//         Self(false, v, ArcBool::default(), PhantomData)
//     }
// }
// impl<V, B> CustomLoader<V, B>
// where
//     V: Loading
// {
//     pub fn get_asset_ref(&self) -> &V {
//         &self.1
//     }
//
//     pub fn is_inited(&self) -> bool {
//         *self.2.read()?
//     }
//
//     pub fn set_inited(&self) {
//         *self.2.write()? = true;
//     }
//
//     pub fn get_init_status(&self) -> ArcBool {
//         self.2.clone()
//     }
//
//     #[inline]
//     pub fn is_loaded(&self) -> bool {
//         self.0
//     }
// }
//
// impl<V, B> Loading for CustomLoader<V, B>
// where
//     V: Loading
// {
//     fn loading(&mut self, server: &AssetServer) -> bool {
//         if !self.0 {
//             self.0 = self.1.loading(server);
//         }
//         self.0
//     }
// }
//
// impl<V, B> From<Handle<V>> for CustomLoader<Handle<V>, B>
// where
//     V: Asset
// {
//     fn from(item: Handle<V>) -> Self {
//         CustomLoader(false, item, ArcBool::default(), PhantomData)
//     }
// }
// impl<B> From<Vec<HandleUntyped>>
//     for CustomLoader<Vec<HandleUntyped>, B>
// {
//     fn from(item: Vec<HandleUntyped>) -> Self {
//         CustomLoader(false, item, ArcBool::default(), PhantomData)
//     }
// }
//
// impl<V, B> From<&CustomLoader<Handle<V>, B>> for HandleId
// where
//     V: Asset
// {
//     fn from(item: &CustomLoader<Handle<V>, B>) -> Self {
//         item.1.id()
//     }
// }
//
// pub trait ArcBoolTrait {
//     fn is_true(&self) -> bool;
//     fn set_true(&mut self);
// }
//
// pub type ArcBool = Arc<RwLock<bool>>;
// impl ArcBoolTrait for ArcBool {
//     fn is_true(&self) -> bool {
//         *self.read()?
//     }
//
//     fn set_true(&mut self) {
//         *self.write()? = true;
//     }
// }
//
// impl<T> Loading for Handle<T>
// where
//     T: Asset
// {
//     fn loading(&mut self, server: &AssetServer) -> bool {
//         matches!(server.get_load_state(self.id()),
// LoadState::Loaded)     }
// }
// impl Loading for HandleUntyped {
//     fn loading(&mut self, server: &AssetServer) -> bool {
//         matches!(server.get_load_state(self.id()),
// LoadState::Loaded)     }
// }
// impl Loading for Vec<HandleUntyped> {
//     fn loading(&mut self, server: &AssetServer) -> bool {
//         matches!(
//             server.get_group_load_state(
//                 self.iter().map(|handle| handle.id())
//             ),
//             LoadState::Loaded
//         )
//     }
// }
