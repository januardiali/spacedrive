use crate::{invalidate_query, prisma::object};

use rspc::Type;
use serde::Deserialize;

use super::{utils::LibraryRequest, RouterBuilder};

pub(crate) fn mount() -> RouterBuilder {
	<RouterBuilder>::new()
		.library_query("readMetadata", |t| {
			t(|_, _id: i32, _| async move {
				#[allow(unreachable_code)]
				Ok(todo!())
			})
		})
		.library_mutation("setNote", |t| {
			#[derive(Type, Deserialize)]
			pub struct SetNoteArgs {
				pub id: i32,
				pub note: Option<String>,
			}

			t(|_, args: SetNoteArgs, library| async move {
				library
					.db
					.object()
					.update(
						object::id::equals(args.id),
						vec![object::note::set(args.note)],
					)
					.exec()
					.await?;

				invalidate_query!(library, "locations.getExplorerData");

				Ok(())
			})
		})
		.library_mutation("setFavorite", |t| {
			#[derive(Type, Deserialize)]
			pub struct SetFavoriteArgs {
				pub id: i32,
				pub favorite: bool,
			}

			t(|_, args: SetFavoriteArgs, library| async move {
				library
					.db
					.object()
					.update(
						object::id::equals(args.id),
						vec![object::favorite::set(args.favorite)],
					)
					.exec()
					.await?;

				invalidate_query!(library, "locations.getExplorerData");

				Ok(())
			})
		})
		.library_mutation("delete", |t| {
			t(|_, id: i32, library| async move {
				library
					.db
					.object()
					.delete(object::id::equals(id))
					.exec()
					.await?;

				invalidate_query!(library, "locations.getExplorerData");
				Ok(())
			})
		})
}
