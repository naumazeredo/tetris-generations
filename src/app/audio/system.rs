use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use crate::impl_imdraw_todo;
use crate::app::{
    App,
    ImDraw,
    utils::fnv_hasher::FNVHasher,
};

// @TODO macro this
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ImDraw)]
pub struct MusicId(u64);

impl MusicId {
    fn new(s: &str) -> Self {
        let mut hasher = FNVHasher::new();
        s.hash(&mut hasher);
        Self(hasher.finish())
    }
}

// @TODO macro this
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ImDraw)]
pub struct SfxId(u64);

impl SfxId {
    fn new(s: &str) -> Self {
        let mut hasher = FNVHasher::new();
        s.hash(&mut hasher);
        Self(hasher.finish())
    }
}

pub(in crate::app) struct AudioSystem<'a> {
    pub(in crate::app) musics: BTreeMap<MusicId, sdl2::mixer::Music<'a>>,
    pub(in crate::app) sfxs:   BTreeMap<SfxId,   sdl2::mixer::Chunk>,
}

impl AudioSystem<'_> {
    pub(in crate::app) fn new() -> Self {
        sdl2::mixer::open_audio(
            sdl2::mixer::DEFAULT_FREQUENCY,
            sdl2::mixer::DEFAULT_FORMAT,
            2,
            1024
        ).unwrap();

        sdl2::mixer::allocate_channels(16);

        Self {
            musics: BTreeMap::new(),
            sfxs:   BTreeMap::new(),
        }
    }
}

impl Drop for AudioSystem<'_> {
    fn drop(&mut self) {
        sdl2::mixer::close_audio();
    }
}

impl App<'_> {
    pub fn load_music(&mut self, filename: &str) -> MusicId {
        let music_id = MusicId::new(filename);
        self.audio_system.musics.entry(music_id)
            .or_insert_with(|| {
                // @TODO check result
                sdl2::mixer::Music::from_file(filename).unwrap()
            });
        music_id
    }

    // @TODO return Result
    pub fn play_music(&self, music_id: MusicId) {
        // @TODO check results
        let music = self.audio_system.musics.get(&music_id).unwrap();
        music.play(-1).unwrap();
    }

    pub fn load_sfx(&mut self, filename: &str) -> SfxId {
        let sfx_id = SfxId::new(filename);
        self.audio_system.sfxs.entry(sfx_id)
            .or_insert_with(|| {
                // @TODO check result
                sdl2::mixer::Chunk::from_file(filename).unwrap()
            });
        sfx_id
    }

    // @TODO return Result
    pub fn play_sfx(&self, sfx_id: SfxId) {
        let chunk = self.audio_system.sfxs.get(&sfx_id).unwrap();
        sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
    }

    pub fn pause_audio(&self) {
        sdl2::mixer::Channel::all().pause();
    }

    pub fn resume_audio(&self) {
        sdl2::mixer::Channel::all().resume();
    }

    pub fn halt_audio(&self) {
        sdl2::mixer::Channel::all().halt();
    }
}

impl_imdraw_todo!(AudioSystem<'_>);
