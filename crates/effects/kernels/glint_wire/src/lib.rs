//! Glint composed host: maps the typed requests of the glint workers to the ported crates and refuses the frames outside the diagonal and shift ranges.
pub mod host;
pub mod mapped;
pub mod op;
pub mod refusal;

use glint_worker::Input;

/// The op entry for the three-plane worker: the worker's refusal predicate first (before any host call), then the worker against the host.
pub fn render(inp: &Input, host: &mut host::ComposedHost) -> Result<(), String> {
    glint_worker::run(inp, host).map_err(|r| r.name().to_string())?;
    match (&host.refused, &host.err) {
        (Some(r), _) => Err((*r).to_string()),
        (_, Some(e)) => Err(e.clone()),
        _ => Ok(()),
    }
}
