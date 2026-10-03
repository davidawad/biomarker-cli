use serde_json::json;

use crate::cli::{PersonAddArgs, PersonCmd, PersonEditArgs};
use crate::context::Ctx;
use crate::db::{int, RowExt};
use crate::error::{AppError, Result};
use crate::output::{to_record, Record, Report};
use crate::store::{self, Person};
use crate::util::{merge_tags, now_iso, parse_date, validate_slug};

pub fn run(ctx: &Ctx, cmd: PersonCmd) -> Result<()> {
    match cmd {
        PersonCmd::Add(a) => add(ctx, a),
        PersonCmd::List => list(ctx),
        PersonCmd::Show { slug } => show(ctx, &slug),
        PersonCmd::Edit(a) => edit(ctx, a),
        PersonCmd::Rm { slug, force } => rm(ctx, &slug, force),
    }
}

fn person_record(p: &Person) -> Record {
    to_record(p)
}

fn norm_dob(ctx: &Ctx, dob: Option<String>) -> Result<Option<String>> {
    dob.map(|d| parse_date(&d, &ctx.tz).map(|d| d.to_string())).transpose()
}

fn add(ctx: &Ctx, a: PersonAddArgs) -> Result<()> {
    let db = ctx.db()?;
    let p = Person {
        id: 0,
        slug: validate_slug(&a.slug)?,
        name: a.name,
        sex: a.sex.map(|s| s.as_str().to_string()),
        dob: norm_dob(ctx, a.dob)?,
        notes: a.notes,
        tags: merge_tags(&[], &a.tags, &[]),
        created_at: now_iso(),
    };
    let saved = store::insert_person(&db, &p)?;
    ctx.info(&format!("added person {}", saved.slug));
    ctx.emit_mutation(&Report::object("person", person_record(&saved)))
}

fn list(ctx: &Ctx) -> Result<()> {
    let db = ctx.db()?;
    let rows = store::list_people(&db)?.iter().map(person_record).collect();
    ctx.emit(&Report::list("people", rows).table_columns(&["id", "slug", "name", "sex", "dob", "tags"]))
}

fn show(ctx: &Ctx, slug: &str) -> Result<()> {
    let db = ctx.db()?;
    let p = store::get_person(&db, slug)?;
    let stats = db
        .query_opt(
            "SELECT count(*), count(DISTINCT marker_id), min(taken_at), max(taken_at) FROM measurements WHERE person_id = ?1",
            &[int(p.id)],
        )?
        .unwrap_or_default();
    let mut rec = person_record(&p);
    rec.insert("measurements".into(), json!(stats.i(0).unwrap_or(0)));
    rec.insert("markers".into(), json!(stats.i(1).unwrap_or(0)));
    rec.insert("first_date".into(), json!(stats.s(2)));
    rec.insert("last_date".into(), json!(stats.s(3)));
    ctx.emit(&Report::object("person", rec).dates(&["first_date", "last_date", "dob"]))
}

fn edit(ctx: &Ctx, a: PersonEditArgs) -> Result<()> {
    let db = ctx.db()?;
    let p = store::get_person(&db, &a.slug)?;
    let updated = Person {
        slug: a.rename.as_deref().map(validate_slug).transpose()?.unwrap_or(p.slug.clone()),
        name: a.name.or(p.name.clone()),
        sex: a.sex.map(|s| s.as_str().to_string()).or(p.sex.clone()),
        dob: norm_dob(ctx, a.dob)?.or(p.dob.clone()),
        notes: a.notes.or(p.notes.clone()),
        tags: merge_tags(&p.tags, &a.add_tags, &a.rm_tags),
        ..p
    };
    let saved = store::update_person(&db, &updated)?;
    ctx.info(&format!("updated person {}", saved.slug));
    ctx.emit_mutation(&Report::object("person", person_record(&saved)))
}

fn rm(ctx: &Ctx, slug: &str, force: bool) -> Result<()> {
    let db = ctx.db()?;
    let p = store::get_person(&db, slug)?;
    let n = db.query_scalar_i64("SELECT count(*) FROM measurements WHERE person_id = ?1", &[int(p.id)])?;
    if n > 0 && !force {
        return Err(AppError::invalid(format!("person '{slug}' has {n} measurements; use --force to delete them too")));
    }
    let deleted = store::delete_person(&db, p.id)?;
    ctx.info(&format!("removed person {slug} ({deleted} measurements)"));
    ctx.emit_mutation(&Report::object(
        "removed",
        to_record(&json!({"person": p.slug, "measurements_deleted": deleted})),
    ))
}
