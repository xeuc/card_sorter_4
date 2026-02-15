use bevy::prelude::*;
use serde::{Deserialize, Serialize, de};


macro_rules! define_tiers {
    ($($name:ident => $label:expr),+ $(,)?) => {
        #[derive(Clone, Copy, Serialize, Deserialize, Component, Debug)]
        #[repr(u8)]
        pub enum Tier {
            $($name),+
        }

        pub struct TierInfo {
            pub label: &'static str,
        }

        pub const TIER_INFOS: [TierInfo; count_idents!($($name),+)] = [
            $(TierInfo { label: $label }),+
        ];

        impl Tier {
            pub const ALL: [Tier; count_idents!($($name),+)] = [
                $(Tier::$name),+
            ];

            #[inline]
            pub const fn index(self) -> usize {
                self as usize
            }

            pub fn label(self) -> &'static str {
                TIER_INFOS[self.index()].label
            }

            pub fn color(self) -> Color {
                let i = self.index() as f32;
                let n = TIER_INFOS.len() as f32;
                Color::hsl(360.0 * i / n, 0.95, 0.7)
            }
        }
    };
}


macro_rules! count_idents {
    ($head:ident $(, $tail:ident)*) => {
        1 + count_idents!($($tail),*)
    };
    () => { 0 };
}


define_tiers! {
    DIVINE  => "DIVINE",
    ULTRA   => "ULTRA",
    RARE    => "RARE",
    COMMON  => "COMMON",
    // IDK     => "IDK",
    VERSO   => "VERSO",
    // CENSOR  => "CENSOR",
    // DUPLI   => "DUPLI",
    TRASH   => "TRASH",
}

// SelectedContainer makes it easy to pick Container user clicked on
#[derive(Component)]
pub struct SelectedContainer;
