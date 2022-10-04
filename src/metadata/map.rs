use super::*;

use bevy_parallax::{LayerData as ParallaxLayerData, ParallaxResource};

#[derive(Component, HasLoadProgress, TypeUuid, Deserialize, Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
#[uuid = "8ede98c2-4f17-46f2-bcc5-ae0dc63b2137"]
pub struct MapMeta {
    pub name: String,
    /// The parallax background layers
    #[serde(default)]
    pub background_layers: Vec<ParallaxLayerMeta>,
    /// The background color of the map, behind the parallax layers
    pub background_color: ColorMeta,
    /// Size of the map in tiles
    pub grid_size: UVec2,
    /// The size of the tiles in pixels
    pub tile_size: UVec2,
    /// The layers of the map
    pub layers: Vec<MapLayerMeta>,
}

impl MapMeta {
    #[allow(unused)] // Until we use it
    pub fn get_parallax_resource(&self) -> ParallaxResource {
        ParallaxResource::new(
            self.background_layers
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
        )
    }
}

#[derive(HasLoadProgress, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MapLayerMeta {
    pub id: String,
    pub kind: MapLayerKind,
    #[serde(skip)]
    pub entity: Option<Entity>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum MapLayerKind {
    Tile(MapTileLayer),
    Element(MapElementLayer),
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct MapTileLayer {
    pub tilemap: String,
    #[serde(skip)]
    pub tilemap_handle: Handle<Image>,
    pub has_collision: bool,
    pub tiles: Vec<MapTileMeta>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct MapElementLayer {
    elements: Vec<MapElementSpawn>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct MapElementSpawn {
    pub pos: Vec2,
    pub name: String,
    pub category: String,
    pub element: String,
    #[serde(skip)]
    pub element_handle: Handle<MapElementMeta>,
}

impl HasLoadProgress for MapLayerKind {
    fn load_progress(
        &self,
        _loading_resources: &bevy_has_load_progress::LoadingResources,
    ) -> bevy_has_load_progress::LoadProgress {
        warn!("TODO: Implement load progress for MapLayerKindMeta");
        bevy_has_load_progress::LoadProgress::default()
    }
}

#[derive(HasLoadProgress, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MapTileMeta {
    pub pos: UVec2,
    pub idx: u32,
}

#[derive(HasLoadProgress, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ParallaxLayerMeta {
    pub speed: f32,
    pub image: String,
    #[serde(skip)]
    pub image_handle: Handle<Image>,
    pub tile_size: Vec2,
    pub cols: usize,
    pub rows: usize,
    pub scale: f32,
    pub z: f32,
    pub transition_factor: f32,
    pub position: Vec2,
}

impl Default for ParallaxLayerMeta {
    fn default() -> Self {
        Self {
            speed: default(),
            image: default(),
            image_handle: default(),
            tile_size: default(),
            cols: 1,
            rows: 1,
            scale: 1.0,
            z: default(),
            transition_factor: 1.0,
            position: default(),
        }
    }
}

impl From<ParallaxLayerMeta> for ParallaxLayerData {
    fn from(meta: ParallaxLayerMeta) -> Self {
        Self {
            speed: meta.speed,
            path: meta.image,
            tile_size: meta.tile_size,
            cols: meta.cols,
            rows: meta.rows,
            scale: meta.scale,
            z: meta.z,
            transition_factor: meta.transition_factor,
            position: meta.position,
        }
    }
}

#[derive(Component, HasLoadProgress, TypeUuid, Deserialize, Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
#[uuid = "0a4a0cc6-ee52-4b0d-a88b-871c49a06622"]
pub struct MapElementMeta {}
