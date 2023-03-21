use std::{
    any::{Any, TypeId},
    collections::HashMap,
    path::Path,
    sync::{Arc, Weak},
};

use module::{DynamicModule, Module};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use smash::{app::BattleObjectModuleAccessor, phx::Hash40};

use crate::WorkParamObject;

struct LoadedParams {
    params: Weak<(dyn Any + Sync + Send + 'static)>,
    loader: Box<(dyn Fn() -> Arc<(dyn Any + Sync + Send + 'static)> + Sync + Send + 'static)>,
}

static PARAM_LOADERS: Lazy<RwLock<HashMap<Hash40, LoadedParams>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn add<T: DeserializeOwned + Any + Sync + Send + 'static>(
    agent_kind: &str,
    path: impl AsRef<Path>,
) {
    let path = path.as_ref().to_path_buf();
    let agent_kind = agent_kind.to_string();
    PARAM_LOADERS.write().insert(
        Hash40::new(agent_kind.as_str()),
        LoadedParams {
            params: Weak::<T>::new(),
            loader: Box::new(move || {
                let file_data = match std::fs::read_to_string(&path) {
                    Ok(data) => data,
                    Err(e) => {
                        panic!(
                            "Failed to read '{}' when loading params for '{}': {:?}",
                            path.display(),
                            agent_kind,
                            e
                        );
                    }
                };

                let params = match toml::from_str::<T>(&file_data) {
                    Ok(params) => params,
                    Err(e) => {
                        panic!(
                            "Failed to parse '{}' when loading params for '{}': {:?}",
                            path.display(),
                            agent_kind,
                            e
                        );
                    }
                };

                Arc::new(params) as _
            }),
        },
    );
}

pub struct ParamModule {
    agent_params: Option<Arc<(dyn Any + Sync + Send + 'static)>>,
    work_cache: HashMap<TypeId, Arc<(dyn Any + Sync + Send + 'static)>>,
}

impl ParamModule {
    pub fn get<T: Any>(module_accessor: *const BattleObjectModuleAccessor) -> &'static T {
        let module = module::get_module_by_name::<Self>(module_accessor);
        let Some(params) = module.agent_params.as_ref() else {
            panic!("No agent parameters exist!");
        };

        let Some(params) = params.downcast_ref::<T>() else {
            panic!("Parameters are not of type {}", std::any::type_name::<T>());
        };

        params
    }

    pub fn get_from_work<T: Any + WorkParamObject + Send + Sync + 'static>(
        module_accessor: *mut BattleObjectModuleAccessor,
    ) -> &'static T {
        let module = module::get_module_by_name::<Self>(module_accessor);

        let entry = module
            .work_cache
            .entry(std::any::TypeId::of::<T>())
            .or_insert_with(|| Arc::new(T::parse(module_accessor, T::NAME)) as _);

        let Some(params) = entry.downcast_ref() else {
            panic!("Work parameters are not of type {}", std::any::type_name::<T>());
        };

        params
    }
}

use super::WorkParams;
#[derive(WorkParams, Debug)]
#[work(object = "param_fireball")]
pub struct MarioFireballParams {
    pub life: i32,
    pub speed: f32,
    pub gravity_accel: f32,
    pub angle: f32,
    pub bound_ref_speed_mul: f32,
    pub bounded_speed_y_min: f32,
    pub bounded_speed_y_max: f32,
    pub bound_count_max: i32,
    pub wall_check_angle: f32,
    pub is_penetration: bool,
    pub gravity_acl_max: f32,
}

impl Module for ParamModule {
    const NAME: &'static str = "ParamModule";

    fn new(init_args: module::InitArgs) -> Option<Self> {
        let mut params = PARAM_LOADERS.write();
        let agent_params = if let Some(agent_params) = params.get_mut(&init_args.agent_kind_hash) {
            if let Some(agent_params) = agent_params.params.upgrade() {
                Some(agent_params)
            } else {
                let params = (agent_params.loader)();
                agent_params.params = Arc::downgrade(&params);
                Some(params)
            }
        } else {
            None
        };

        Some(ParamModule {
            agent_params,
            work_cache: HashMap::new(),
        })
    }
}

impl DynamicModule for ParamModule {
    fn start(&mut self, _args: module::StartArgs) {}

    fn end(&mut self) {}

    fn finalize(&mut self) {}
}
