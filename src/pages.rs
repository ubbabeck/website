pub(crate) mod add;
mod index;

use std::vec;

use futures::FutureExt;
use maud::Render;
use ssg::{sources::FileSource, FileSpec};

use crate::{
    components,
    mobs::{self, Mob},
};

fn mob_page(mob: Mob) -> FileSpec {
    FileSpec::new(
        format!("/mobs/{}.html", mob.id),
        FileSource::BytesWithFileSpecSafety(Box::new(move |targets| {
            let mob = mob.clone();

            async { Ok(components::MobPage { mob, targets }.render().0.into_bytes()) }.boxed()
        })),
    )
}

pub(crate) async fn all() -> Vec<FileSpec> {
    let mobs = mobs::read_all_mobs().await;
    let mut mob_pages = mobs.iter().cloned().map(mob_page).collect::<Vec<_>>();
    let mut pages = vec![index::page().await, add::page()];

    pages.append(&mut mob_pages);

    pages
}
