pub async fn health() -> &'static str {
    "sumire is alive"
}

/* mod api;
 *
 * mod file_name;
 * pub use file_name::function_name;
 */

mod wah;
pub use wah::wah;

mod notes;
pub use notes::create_note;
pub use notes::delete_note;
pub use notes::read_notes;
pub use notes::update_note;
