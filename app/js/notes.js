async function write_note() {
    const note_title = document.getElementById("note_title");
    const note_body = document.getElementById("note_body");

    const data = {
        title: note_title.value.trim(),
        body: note_body.value.trim(),
    };

    if (!data.title || !data.body) {
        note_title.value = data.title;
        note_body.value = data.body;
        toast("Empty", "Empty title or body");
        return;
    }

    try {
        const response = await fetch("http://127.0.0.1:42069/api/notes", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });

        switch (response.status) {
            case 201:
                toast("Success", "Note written");
                const response_json = await response.json();
                show_note(undefined, response_json);
                break;
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
}

function toggle_note_options() {
    const note = document.getElementById("show_note_options").classList;
    const del = document.getElementById("delete_confirmation").classList;

    if (del.contains("hidden")) {
        del.add("flex");
        note.add("hidden");
        del.remove("hidden");
    } else {
        del.remove("flex");
        del.add("hidden");
        note.remove("hidden");
    }
}

async function read_note(note_id) {
    try {
        const response = await fetch(
            `http://127.0.0.1:42069/api/notes/${note_id}`,
            {
                method: "GET",
            }
        );

        switch (response.status) {
            case 200:
                return await response.json();
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
    show_notes();
}

async function read_notes() {
    try {
        const response = await fetch("http://127.0.0.1:42069/api/notes", {
            method: "GET",
        });

        switch (response.status) {
            case 200:
                return await response.json();
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
    return [];
}

async function update_note(note_id) {
    const note_title = document.getElementById("note_title");
    const note_body = document.getElementById("note_body");

    const data = {
        title: note_title.value.trim(),
        body: note_body.value.trim(),
    };

    if (!data.title || !data.body) {
        note_title.value = data.title;
        note_body.value = data.body;
        toast("Empty", "Empty title or body");
        return;
    }

    try {
        const response = await fetch(
            `http://127.0.0.1:42069/api/notes/${note_id}`,
            {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
            }
        );

        switch (response.status) {
            case 200:
                const response_json = await response.json();
                note_title.value = response_json.title;
                note_body.value = response_json.body;
                toast("Success", "Note updated");
                break;
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
}

async function delete_note(note_id) {
    try {
        const response = await fetch(
            `http://127.0.0.1:42069/api/notes/${note_id}`,
            {
                method: "DELETE",
            }
        );

        switch (response.status) {
            case 200:
                toast("Success", "Note deleted");
                show_notes();
                break;
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
}

// onload
(async function () {
    const session_id = Cookies.get("session_id");

    if (session_id == undefined) {
        location.href = "auth";
        return;
    }

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/session_id",
            {
                method: "PUT",
            }
        );

        switch (response.status) {
            case 200:
                const response_json = await response.json();
                Cookies.set("session_id", await response_json.session_id);
                break;
            default:
                Cookies.remove("session_id");
                location.href = "auth";
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }

    show_notes();
})();
