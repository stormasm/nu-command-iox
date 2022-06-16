use nu_protocol::engine::{EngineState, StateWorkingSet};

use std::path::Path;

use crate::*;

pub fn create_default_context(cwd: impl AsRef<Path>) -> EngineState {
    let mut engine_state = EngineState::new();

    let delta = {
        let mut working_set = StateWorkingSet::new(&engine_state);

        macro_rules! bind_command {
            ( $( $command:expr ),* $(,)? ) => {
                $( working_set.add_decl(Box::new($command)); )*
            };
        }

        // If there are commands that have the same name as default declarations,
        // they have to be registered before the main declarations. This helps to make
        // them only accessible if the correct input value category is used with the
        // declaration
        #[cfg(feature = "dataframe")]
        add_dataframe_decls(&mut working_set);

        // Database-related
        // Adds all related commands to query databases
        #[cfg(feature = "database")]
        add_database_decls(&mut working_set);

        // Core
        bind_command! {
            Alias,
            Debug,
            Def,
            DefEnv,
            Describe,
            Do,
            Echo,
            ErrorMake,
            ExportAlias,
            ExportCommand,
            ExportDef,
            ExportDefEnv,
            ExportEnv,
            ExportExtern,
            Extern,
            For,
            Help,
            Hide,
            History,
            If,
            Ignore,
            Overlay,
            OverlayAdd,
            OverlayList,
            OverlayNew,
            OverlayRemove,
            Let,
            Metadata,
            Module,
            Source,
            Use,
        };

        // Shells
        bind_command! {
            Exit,
        };

        // Viewers
        bind_command! {
            Table,
        };

        // Env
        bind_command! {
            Env,
            LetEnv,
            LoadEnv,
            WithEnv,
            ConfigNu,
            ConfigEnv,
            ConfigMeta,
        };

        // System
        bind_command! {
            External,
        };

        working_set.render()
    };

    let _ = engine_state.merge_delta(delta, None, &cwd);

    engine_state
}
