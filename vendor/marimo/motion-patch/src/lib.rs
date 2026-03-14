use std::collections::BTreeMap;

use hash40::Hash40;
use serde::{Deserialize, Serialize};

use motion_lib::mlist::{self as mot, MList};

macro_rules! apply {
    ($self:ident, $name:ident, $other:ident) => {{
        apply!($self, $name, $name, $other)
    }};
    ($self:ident, $name:ident, $other_name:ident, $other:ident) => {{
        if let Some(val) = $self.$name {
            $other.$other_name = val;
        }
    }};
}

macro_rules! create {
    ($this:ident, $name:ident, $src:ident, $dst:ident) => {{
        create!($this, $name, $name, $src, $dst)
    }};
    ($this:ident, $name:ident, $other_name:ident, $src:ident, $dst:ident) => {{
        if $src.$other_name != $dst.$other_name {
            $this.$name = Some($dst.$other_name);
        }
    }};
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct FlagsPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<bool>,
    #[serde(rename = "loop", skip_serializing_if = "Option::is_none")]
    pub loop_: Option<bool>,
    #[serde(rename = "move", skip_serializing_if = "Option::is_none")]
    pub move_: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_trans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_rot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_40: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_80: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_100: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_200: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_400: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_800: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_1000: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_2000: Option<bool>,
}

impl From<mot::Flags> for FlagsPatch {
    fn from(value: mot::Flags) -> Self {
        Self {
            turn: Some(value.turn),
            loop_: Some(value.r#loop),
            move_: Some(value.r#move),
            fix_trans: Some(value.fix_trans),
            fix_rot: Some(value.fix_rot),
            fix_scale: Some(value.fix_scale),
            unk_40: Some(value.unk_40),
            unk_80: Some(value.unk_80),
            unk_100: Some(value.unk_100),
            unk_200: Some(value.unk_200),
            unk_400: Some(value.unk_400),
            unk_800: Some(value.unk_800),
            unk_1000: Some(value.unk_1000),
            unk_2000: Some(value.unk_2000),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Animation {
    pub name: Hash40,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk: Option<u8>,
}

impl FlagsPatch {
    pub fn apply(&self, other: &mut mot::Flags) {
        apply!(self, turn, other);
        apply!(self, loop_, r#loop, other);
        apply!(self, move_, r#move, other);
        apply!(self, fix_trans, other);
        apply!(self, fix_rot, other);
        apply!(self, fix_scale, other);
        apply!(self, unk_40, other);
        apply!(self, unk_80, other);
        apply!(self, unk_100, other);
        apply!(self, unk_200, other);
        apply!(self, unk_400, other);
        apply!(self, unk_800, other);
        apply!(self, unk_1000, other);
        apply!(self, unk_2000, other);
    }

    pub fn try_create(src: &mot::Flags, dst: &mot::Flags) -> Option<Self> {
        let mut this = Self::default();
        create!(this, turn, src, dst);
        create!(this, loop_, r#loop, src, dst);
        create!(this, move_, r#move, src, dst);
        create!(this, fix_trans, src, dst);
        create!(this, fix_rot, src, dst);
        create!(this, fix_scale, src, dst);
        create!(this, unk_40, src, dst);
        create!(this, unk_80, src, dst);
        create!(this, unk_100, src, dst);
        create!(this, unk_200, src, dst);
        create!(this, unk_400, src, dst);
        create!(this, unk_800, src, dst);
        create!(this, unk_1000, src, dst);
        create!(this, unk_2000, src, dst);

        (this != Self::default()).then_some(this)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ExtraPatch {
    #[serde(default, skip_serializing_if = "is_false")]
    pub remove: bool,

    #[serde(alias = "xlu_start", skip_serializing_if = "Option::is_none")]
    pub intangible_start_frame: Option<u8>,
    #[serde(alias = "xlu_end", skip_serializing_if = "Option::is_none")]
    pub intangible_end_frame: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_frame: Option<u8>,

    #[serde(alias = "no_stop_intp", skip_serializing_if = "Option::is_none")]
    pub freeze_during_hitstop: Option<bool>,
}

impl From<mot::Extra> for ExtraPatch {
    fn from(value: mot::Extra) -> Self {
        Self {
            remove: false,
            intangible_start_frame: Some(value.xlu_start),
            intangible_end_frame: Some(value.xlu_end),
            cancel_frame: Some(value.cancel_frame),
            freeze_during_hitstop: Some(value.no_stop_intp),
        }
    }
}

impl ExtraPatch {
    pub fn apply(&self, other: &mut mot::Extra) {
        apply!(self, intangible_start_frame, xlu_start, other);
        apply!(self, intangible_end_frame, xlu_end, other);
        apply!(self, cancel_frame, other);
        apply!(self, freeze_during_hitstop, no_stop_intp, other);
    }

    pub fn try_create(src: &Option<mot::Extra>, dst: &Option<mot::Extra>) -> Option<Self> {
        if src.is_none() && dst.is_none() {
            return None;
        }

        let Some(src) = src.as_ref() else {
            let dst = dst.as_ref().unwrap();
            return Some(Self {
                remove: false,
                intangible_end_frame: Some(dst.xlu_start),
                intangible_start_frame: Some(dst.xlu_end),
                cancel_frame: Some(dst.cancel_frame),
                freeze_during_hitstop: Some(dst.no_stop_intp)
            });
        };

        let Some(dst) = dst.as_ref() else {
            return Some(Self { remove: true, ..Default::default() });
        };

        let mut this = Self::default();
        create!(this, intangible_start_frame, xlu_start, src, dst);
        create!(this, intangible_end_frame, xlu_end, src, dst);
        create!(this, cancel_frame, src, dst);
        create!(this, freeze_during_hitstop, no_stop_intp, src, dst);

        (this != Self::default()).then_some(this)
    }
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct MotionPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<Hash40>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub remove: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_script: Option<Hash40>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<FlagsPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_frames: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<Animation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<Hash40>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<ExtraPatch>,
}

impl From<mot::Motion> for MotionPatch {
    fn from(value: mot::Motion) -> Self {
        let anims = value
            .animations
            .into_iter()
            .map(|anim| Animation {
                name: anim.name,
                unk: (anim.unk != 0).then_some(anim.unk),
            })
            .collect();

        Self {
            rename: None,
            remove: false,
            game_script: Some(value.game_script),
            flags: Some(value.flags.into()),
            blend_frames: Some(value.blend_frames),
            animations: Some(anims),
            scripts: Some(value.scripts),
            extra: value.extra.map(Into::into),
        }
    }
}

impl MotionPatch {
    pub fn apply(&self, entry: &mut mot::Motion) {
        apply!(self, game_script, entry);
        if let Some(flags) = self.flags.as_ref() {
            flags.apply(&mut entry.flags);
        }

        apply!(self, blend_frames, entry);

        if let Some(anims) = self.animations.as_ref() {
            entry.animations = anims
                .iter()
                .map(|anim| mot::Animation {
                    name: anim.name,
                    unk: anim.unk.unwrap_or_default(),
                })
                .collect();
        }

        if let Some(scripts) = self.scripts.as_ref() {
            entry.scripts = scripts.clone();
        }

        if let Some(extra) = self.extra.as_ref() {
            let mut extra_entry = entry.extra.clone().unwrap_or_default();
            extra.apply(&mut extra_entry);
            entry.extra = Some(extra_entry);
        }
    }

    pub fn try_create(src: &mot::Motion, dst: &mot::Motion) -> Option<Self> {
        let mut this = Self::default();

        create!(this, game_script, src, dst);

        this.flags = FlagsPatch::try_create(&src.flags, &dst.flags);

        create!(this, blend_frames, src, dst);

        if src.animations != dst.animations {
            let anims = dst
                .animations
                .iter()
                .map(|anim| Animation {
                    name: anim.name,
                    unk: (anim.unk != 0).then_some(anim.unk),
                })
                .collect();

            this.animations = Some(anims);
        }

        if src.scripts != dst.scripts {
            this.scripts = Some(dst.scripts.clone());
        }

        this.extra = ExtraPatch::try_create(&src.extra, &dst.extra);

        (this != Self::default()).then_some(this)
    }
}

pub trait MotionMapExt
where
    for<'a> &'a Self: IntoIterator<Item = (&'a Hash40, &'a MotionPatch)>,
    Self: FromIterator<(Hash40, MotionPatch)>,
{
    fn apply(&self, list: &mut MList) {
        for (name, patch) in self.into_iter() {
            if patch.remove {
                list.list.remove(name);
            }

            if let Some(new_name) = patch.rename {
                let mut old = list.list.remove(name).unwrap_or_default();
                patch.apply(&mut old);
                list.list.insert(new_name, old);
            } else {
                let entry = list.list.entry(*name).or_default();
                patch.apply(entry);
            }
        }

        let pre_sorted = std::mem::take(&mut list.list);
        let map = BTreeMap::from_iter(pre_sorted.into_iter());
        list.list = FromIterator::from_iter(map.into_iter());
    }

    fn create(source: &MList, dst: &MList) -> Self {
        let mut list = vec![];
        for (name, entry) in source.list.iter() {
            if let Some(dst) = dst.list.get(name) {
                if let Some(patch) = MotionPatch::try_create(entry, dst) {
                    list.push((*name, patch));
                }
            } else {
                list.push((
                    *name,
                    MotionPatch {
                        remove: true,
                        ..Default::default()
                    },
                ));
            }
        }

        for (name, entry) in dst.list.iter() {
            if source.list.get(name).is_none() {
                list.push((*name, MotionPatch::from(entry.clone())));
            }
        }

        Self::from_iter(list)
    }
}

impl MotionMapExt for BTreeMap<Hash40, MotionPatch> {}

#[test]
fn test() {
    let mut vanilla = motion_lib::open("./test/vanilla.bin").unwrap();
    let patch: BTreeMap<Hash40, MotionPatch> = serde_yaml::from_str(
        std::fs::read_to_string("./test/patch.yml")
            .unwrap()
            .as_str(),
    )
    .unwrap();
    patch.apply(&mut vanilla);
    let mut writer = std::io::Cursor::new(vec![]);
    motion_lib::write_stream(&mut writer, &vanilla).unwrap();
    let bytes = std::fs::read("./test/modded.bin").unwrap();
    assert_eq!(writer.into_inner(), bytes);
}
