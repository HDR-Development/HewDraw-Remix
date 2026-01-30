use crate::consts::{globals::*, vars};
use smash::app::*;
use smash::lib::{lua_const::*, *};
use smash::lua2cpp::*;
use smash::phx::*;
use crate::{InputModule, VarModule};

#[macro_export]
macro_rules! define_trans_map {
        (
            $(
                $to:ident => ($input:path, $from_script:expr)
            ),* $(,)?
        ) => {
            #[macro_export]
            macro_rules! trans_to {
                $(
                    ($f:expr, $to) => {{
                        if $f.is_cat_flag($input) {
                            StatusModule::change_status_request_from_script(
                                $f,
                                *$to,
                                $from_script
                            );
                        }
                    }};
                )*
    
                ($f:expr, $other:ident) => {{
                    compile_error!(concat!(
                        "trans_to!: no mapping for target status ",
                        stringify!($other)
                    ));
                }};
            }
        };
    }
    
    #[macro_export]
    macro_rules! check_input_trans {
    
        ($f:expr, [$($tokens:tt)*]) => {{
            check_input_trans!(@munch $f; $($tokens)*);
        }};
    
        (@munch $f:expr; ) => {{}};
    
        (@munch $f:expr; , $($rest:tt)*) => {{
            check_input_trans!(@munch $f; $($rest)*);
        }};
    
        (@munch $f:expr; *$to:ident $($rest:tt)*) => {{
            trans_to!($f, $to);
            check_input_trans!(@munch $f; $($rest)*);
        }};
    
        (@munch $f:expr; $to:ident $($rest:tt)*) => {{
            trans_to!($f, $to);
            check_input_trans!(@munch $f; $($rest)*);
        }};
    }
    
    define_trans_map! {
        
        // Takes a status and checks for the corresponding input before transitioning
        
        // Tilts
        FIGHTER_STATUS_KIND_ATTACK_LW3 => (Cat1::AttackLw3, false),
        FIGHTER_STATUS_KIND_ATTACK_HI3 => (Cat1::AttackHi3, false),
        FIGHTER_STATUS_KIND_ATTACK_S3 => (Cat1::AttackS3, false),
        
        // Smashes
        FIGHTER_STATUS_KIND_ATTACK_LW4_START => (Cat1::AttackLw4, true),
        FIGHTER_STATUS_KIND_ATTACK_HI4_START => (Cat1::AttackHi4, true),
        FIGHTER_STATUS_KIND_ATTACK_S4_START => (Cat1::AttackS4, true),
    
        // Specials
        FIGHTER_STATUS_KIND_SPECIAL_N  => (Cat1::SpecialN,  false),
        FIGHTER_STATUS_KIND_SPECIAL_S  => (Cat1::SpecialS,  false),
        FIGHTER_STATUS_KIND_SPECIAL_HI => (Cat1::SpecialHi, false),
        FIGHTER_STATUS_KIND_SPECIAL_LW => (Cat1::SpecialLw, false),
    
        // TODO: Add more Input -> Status pairs
    }

