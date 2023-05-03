use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::asset::AssetIo;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

#[allow(unused)]
pub mod jar;

pub struct CustomAssetIoPlugin<IO, P>(P, PhantomData<IO>);

impl<IO, P> CustomAssetIoPlugin<IO, P> {
    fn new(props: P) -> Self {
        CustomAssetIoPlugin(props, PhantomData)
    }
}

impl<IO, P> Plugin for CustomAssetIoPlugin<IO, P>
where
    IO: AssetIo + TryFrom<AssetIoProps<P>> + Sync + Send + 'static,
    IO::Error: Debug,
    P: Clone + Sync + Send + 'static,
{
    fn build(&self, app: &mut App) {
        let task_pool = app
            .world
            .get_resource::<IoTaskPool>()
            .expect("`IoTaskPool` resource not found.")
            .clone();

        let asset_plugin = AssetPlugin {
            asset_folder: "assets".to_string(),
            watch_for_changes: false,
        };
        let base = asset_plugin.create_platform_default_asset_io();

        let props = AssetIoProps {
            base,
            props: self.0.clone(),
        };
        let source = IO::try_from(props).expect("could not initialize asset IO");
        app.insert_resource(AssetServer::new(source, task_pool));
    }
}

pub struct AssetIoProps<P> {
    base: Box<dyn AssetIo>,
    props: P,
}
