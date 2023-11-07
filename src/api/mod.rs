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
